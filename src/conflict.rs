use crate::{trimmed_formula::Clause, Assignments, DecisionLevel, Evaluate, Literal, Variable};
use fixedbitset::FixedBitSet;

pub struct Conflict {
    level: DecisionLevel,
    literals: FixedBitSet,
    assigned_at_level: Variable,
}

impl Conflict {
    pub fn new(num_vars: Variable) -> Self {
        Self {
            level: 0,
            literals: FixedBitSet::with_capacity(num_vars * 2),
            assigned_at_level: 0,
        }
    }

    pub fn assigned_at_level(&self) -> Variable {
        self.assigned_at_level
    }

    pub fn contains(&self, literal: Literal) -> bool {
        self.literals.contains(literal.code())
    }

    pub fn literals<'a>(&'a self) -> impl Iterator<Item = Literal> + 'a {
        self.literals.ones().map(Literal::from_code)
    }

    pub fn variables<'a>(&'a self) -> impl Iterator<Item = Variable> + 'a {
        self.literals().map(Literal::var)
    }

    pub fn add(&mut self, literal: Literal, assignments: &Assignments) {
        if !self.literals.put(literal.code())
            && assignments.assigned_at_level(literal.var(), self.level)
        {
            self.assigned_at_level += 1;
        }
    }

    pub fn initialize(&mut self, level: DecisionLevel, clause: &Clause, assignments: &Assignments) {
        self.literals.clear();
        self.level = level;
        self.assigned_at_level = 0;

        match clause {
            Clause::Binary { a, b } => {
                for literal in [a, b].iter() {
                    self.add(**literal, assignments);
                }
            }
            Clause::Many { literals } => {
                for literal in literals.iter() {
                    self.add(*literal, assignments);
                }
            }
        };

        // Ensure there is at least one literal assigned at the conflict level
        debug_assert_ne!(
            self.assigned_at_level,
            0,
            "There should be at least one literal assigned at the conflict level in the conflict clause"
        );
        dbg!(self.assigned_at_level, self.literals().collect::<Vec<_>>());
    }

    pub fn resolve(
        &mut self,
        literal: Literal,
        other: &Clause,
        assignments: &Assignments,
    ) -> Result<(), String> {
        let code = literal.code();

        println!("resolving {}", literal);

        debug_assert!(self.literals[code]);
        self.literals.set(code, false);
        self.assigned_at_level -= 1;

        match other {
            Clause::Binary { a, b } if *a == !literal => self.add(*b, assignments),
            Clause::Binary { a, b } if *b == !literal => self.add(*a, assignments),
            Clause::Many { literals } => {
                for literal in literals.iter().filter(|x| **x != !literal) {
                    self.add(*literal, assignments);
                }
            }
            _ => return Err("'antecedent' clause wasn't actually antecedent".to_string()),
        }

        Ok(())
    }

    /// Returns a decision level from which the clause can still be satisfied
    pub fn backtrack_level(
        &self,
        conflict_level: DecisionLevel,
        assignments: &Assignments,
    ) -> Option<DecisionLevel> {
        // Make sure all literals are actually unsatisfied
        debug_assert!(self
            .literals()
            .all(|literal| matches!(literal.evaluate(assignments), Some(false))));

        // Ensure there is a single literal assigned at the conflict level
        dbg!(
            conflict_level,
            self.assigned_at_level,
            self.variables().collect::<Vec<_>>(),
            self.variables()
                .filter(|var| assignments.get(*var).unwrap().decision_level() == conflict_level)
                .collect::<Vec<_>>()
        );
        debug_assert_eq!(self.assigned_at_level, 1);
        debug_assert_eq!(
            self.variables()
                .filter(|var| assignments.get(*var).unwrap().decision_level() == conflict_level)
                .count(),
            1,
            "There should be exactly one literal assigned at the conflict level in the clause to be learned"
        );

        // Return the maximum level below the conflict level
        self.variables()
            .map(|var| assignments.get(var).unwrap().decision_level())
            .filter(|level| *level != conflict_level)
            .max()
    }
}
