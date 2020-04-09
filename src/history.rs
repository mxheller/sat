use crate::{Assignments, DecisionLevel, Literal, Variable};

#[derive(Clone, Debug)]
pub struct History {
    assignments: Vec<Literal>,
    next_to_propogate: usize,
    decision_level_breaks: Vec<Variable>,
}

impl History {
    pub fn new(num_vars: Variable) -> Self {
        Self {
            assignments: Vec::with_capacity(num_vars),
            next_to_propogate: 0,
            decision_level_breaks: Vec::new(),
        }
    }

    pub fn add(&mut self, literal: Literal) {
        self.assignments.push(literal);
    }

    pub fn new_decision_level(&mut self) {
        self.decision_level_breaks.push(self.assignments.len());
    }

    pub fn revert_to(&mut self, level: DecisionLevel, assignments: &mut Assignments) {
        if level < self.decision_level_breaks.len() {
            let new_end = self.decision_level_breaks[level];
            for literal in self.assignments.drain(new_end..) {
                assignments.remove(literal.var());
            }
            self.decision_level_breaks.truncate(level);
            self.next_to_propogate = std::cmp::min(self.next_to_propogate, new_end);
        }
    }

    pub fn num_assigned(&self) -> usize {
        self.assignments.len()
    }

    #[must_use]
    pub fn next_to_propogate(&mut self) -> Option<Literal> {
        self.assignments
            .get(self.next_to_propogate)
            .copied()
            .map(|literal| {
                self.next_to_propogate += 1;
                literal
            })
    }

    pub fn most_recently_implied_at_current_level<'a>(
        &'a self,
    ) -> impl Iterator<Item = Literal> + 'a {
        assert!(
            !self.decision_level_breaks.is_empty(),
            "trying to find implied literals at level 0"
        );

        // Find the first implied assignment, which is one after the decided
        // assignment at the current level
        let first = self.decision_level_breaks.last().unwrap() + 1;
        self.assignments[first..].iter().copied().rev()
    }
}

#[test]
fn rewriting_history() {
    use crate::{assignments::Assignment, sign::Sign::Positive, trimmed_formula::TrimmedFormula};

    let formula = TrimmedFormula::new(0);
    let mut history = History::new(6);
    let mut assignments = Assignments::new(6);

    let mut set = |history: &mut History, level, var| {
        let _ = assignments.set(var, Assignment::decided(Positive, level), &formula, history);
    };

    // Decision level 0
    set(&mut history, 0, 0);

    // Decision level 1
    history.new_decision_level();
    set(&mut history, 1, 1);
    set(&mut history, 1, 2);

    // Decision level 2
    history.new_decision_level();
    set(&mut history, 2, 3);
    let _ = assignments.set(
        4,
        Assignment::implied(Positive, 0, 2),
        &formula,
        &mut history,
    );
    let _ = assignments.set(
        5,
        Assignment::implied(Positive, 0, 2),
        &formula,
        &mut history,
    );
    assert_eq!(
        history
            .most_recently_implied_at_current_level()
            .collect::<Vec<_>>(),
        vec![Literal::new(5, Positive), Literal::new(4, Positive)]
    );

    {
        let (mut history, mut assignments) = (history.clone(), assignments.clone());
        history.revert_to(0, &mut assignments);
        assert_eq!(history.assignments, vec![Literal::new(0, Positive)]);
        assert_eq!(history.decision_level_breaks, vec![]);

        assert!(matches!(assignments.get(1), None));
    }

    {
        let (mut history, mut assignments) = (history.clone(), assignments.clone());
        history.revert_to(1, &mut assignments);
        assert_eq!(
            history.assignments,
            vec![
                Literal::new(0, Positive),
                Literal::new(1, Positive),
                Literal::new(2, Positive)
            ]
        );
        assert_eq!(history.decision_level_breaks, vec![1]);
    }

    history.revert_to(2, &mut assignments);
    assert_eq!(
        history.assignments,
        vec![
            Literal::new(0, Positive),
            Literal::new(1, Positive),
            Literal::new(2, Positive),
            Literal::new(3, Positive),
            Literal::new(4, Positive),
            Literal::new(5, Positive)
        ]
    );
    assert_eq!(history.decision_level_breaks, vec![1, 3]);
}
