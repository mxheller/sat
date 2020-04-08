use crate::{
    formula,
    trimmed_formula::{clause, TrimmedFormula},
    Assignment, Assignments, ClauseIdx, DecisionLevel, History, Literal, Solution, Watched,
};

pub struct Solver {
    decision_level: usize,
    num_variables: usize,
    formula: TrimmedFormula,
    assignments: Assignments,
    history: History,
    watched: Watched,
}

enum Status {
    Ok,
    Conflict(formula::Clause),
}

impl Solver {
    pub fn solve_formula(formula: impl Into<formula::Formula>) -> Solution {
        let formula = formula.into();
        let num_variables = formula.num_variables();

        let mut solver = Self {
            formula: TrimmedFormula::new(formula.clauses.len()),
            assignments: Assignments::new(num_variables),
            history: History::new(num_variables),
            watched: Watched::new(num_variables),
            decision_level: 0,
            num_variables,
        };

        let mut units = Vec::new();
        for clause in formula.clauses.into_iter() {
            let mut literals = clause.into_iter();
            match literals.len() {
                0 => return Solution::Unsat,
                1 => {
                    let unit = literals.next().unwrap();
                    solver.assign_invariant(unit);
                    units.push(unit);
                }
                _ => solver.formula.add_clause(literals, &mut solver.watched),
            }
        }

        for unit in units.into_iter() {
            if let Status::Conflict(_) = solver.propogate(unit) {
                return Solution::Unsat;
            }
        }

        solver.solve()
    }

    fn solve(mut self) -> Solution {
        while !self.all_variables_assigned() {
            self.new_decision_level();
            let literal = self.pick_branching_variable();

            if let Status::Conflict(c) = self.assign_and_propogate_decided(literal) {
                if let Some((learned, level)) = self.analyze_conflict(c) {
                    self.backtrack(level);
                    self.learn_clause(learned.into_iter());
                } else {
                    return Solution::Unsat;
                }
            }
        }

        Solution::Sat
    }

    fn new_decision_level(&mut self) {
        self.decision_level += 1;
        self.history.new_decision_level();
    }

    fn assign_and_propogate_decided(&mut self, literal: Literal) -> Status {
        self.assignments.set(
            literal.var(),
            Assignment::decided(literal.sign(), self.decision_level),
            &mut self.history,
        );
        self.propogate(literal)
    }

    fn assign_implied(&mut self, literal: Literal, antecedent: ClauseIdx) {
        self.assignments.set(
            literal.var(),
            Assignment::implied(literal.sign(), antecedent, self.decision_level),
            &mut self.history,
        );
    }

    fn assign_invariant(&mut self, literal: Literal) {
        self.assignments
            .set_invariant(literal.var(), literal.sign());
    }

    fn propogate(&mut self, literal: Literal) -> Status {
        let mut implied = Vec::new();

        // Find clauses in which negated literal (now unsatisfied) is watched
        let affected_clauses = self.watched[!literal]
            .iter()
            .copied()
            .collect::<Vec<usize>>();

        for clause in affected_clauses {
            match self.formula[clause].update(&mut self.watched, &self.assignments, clause) {
                clause::Status::Ok => (),
                clause::Status::Conflict(literals) => return Status::Conflict(literals),
                clause::Status::Implied(literal) => {
                    self.assign_implied(literal, clause);
                    implied.push(literal);
                }
            }
        }

        for literal in implied.into_iter() {
            if let c @ Status::Conflict(_) = self.propogate(literal) {
                return c;
            }
        }

        Status::Ok
    }

    fn learn_clause(&mut self, mut clause: impl Iterator<Item = Literal> + ExactSizeIterator) {
        match clause.len() {
            0 => panic!("Trying to learn an empty (unsat) clause"),
            1 => {
                let unit = clause.next().unwrap();
                self.assign_invariant(unit);
                self.propogate(unit);
            }
            _ => self.formula.add_clause(clause, &mut self.watched),
        }
    }

    fn all_variables_assigned(&self) -> bool {
        self.history.num_assigned() == self.num_variables
    }

    fn pick_branching_variable(&self) -> Literal {
        // TODO: smart
        (0..self.num_variables)
            .find(|var| matches!(self.assignments.get(*var), None))
            .map(|var| Literal::new(var, true))
            .unwrap()
    }

    fn analyze_conflict(
        &self,
        mut clause: formula::Clause,
    ) -> Option<(formula::Clause, DecisionLevel)> {
        let (level, assignments) = (self.decision_level, &self.assignments);

        if level == 0 {
            // Cannot backtrack any farther
            return None;
        }

        while clause.literals_assigned_at(level, assignments).count() > 1 {
            // TODO: this ^ conditional can probably be combined with the below
            // section using equation 4.18

            let antecedent = clause.literals().find_map(|literal| {
                assignments.get(literal.var()).and_then(|assignment| {
                    if assignment.decision_level() == level {
                        assignment.antecedent()
                    } else {
                        None
                    }
                })
            });
            clause.resolve(&self.formula[antecedent.unwrap()]);
        }

        clause
            .backtrack_level(level, assignments)
            .map(|level| (clause, level))
    }

    fn backtrack(&mut self, level: usize) {
        self.decision_level = level;
        self.history.revert_to(level, &mut self.assignments);
    }
}
