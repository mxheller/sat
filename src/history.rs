use crate::{assignments::Assignments, DecisionLevel, Variable};

#[derive(Clone, Debug)]
pub struct History {
    assignments: Vec<Variable>,
    decision_level_breaks: Vec<Variable>,
}

impl History {
    pub fn new(num_vars: Variable) -> Self {
        Self {
            assignments: Vec::with_capacity(num_vars),
            decision_level_breaks: Vec::new(),
        }
    }

    pub fn add(&mut self, var: Variable) {
        self.assignments.push(var);
    }

    pub fn new_decision_level(&mut self) {
        self.decision_level_breaks.push(self.assignments.len());
    }

    pub fn revert_to(&mut self, level: DecisionLevel, assignments: &mut Assignments) {
        if level < self.decision_level_breaks.len() {
            let new_end = self.decision_level_breaks[level];
            for var in self.assignments.drain(new_end..) {
                assignments.remove(var);
            }
            self.decision_level_breaks.truncate(level);
        }
    }
}

#[test]
fn rewriting_history() {
    use crate::{assignments::Assignment, sign::Sign};

    let mut history = History::new(5);
    let mut assignments = Assignments::new(5);

    let mut set = |history: &mut History, level, var| {
        assignments.set(var, Assignment::decided(Sign::Positive, level), history);
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
    set(&mut history, 2, 4);

    {
        let (mut history, mut assignments) = (history.clone(), assignments.clone());
        history.revert_to(0, &mut assignments);
        assert_eq!(history.assignments, vec![0]);
        assert_eq!(history.decision_level_breaks, vec![]);

        assert!(matches!(assignments[1], None));
    }

    {
        let (mut history, mut assignments) = (history.clone(), assignments.clone());
        history.revert_to(1, &mut assignments);
        assert_eq!(history.assignments, vec![0, 1, 2]);
        assert_eq!(history.decision_level_breaks, vec![1]);
    }

    history.revert_to(2, &mut assignments);
    assert_eq!(history.assignments, vec![0, 1, 2, 3, 4]);
    assert_eq!(history.decision_level_breaks, vec![1, 3]);
}
