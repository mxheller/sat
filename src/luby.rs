#[derive(Debug)]
pub enum Luby {
    Init,
    Started { u: usize, v: usize },
}

impl Luby {
    pub fn new() -> Self {
        Self::Init
    }

    pub fn next(&mut self) -> usize {
        match self {
            Luby::Init => {
                let (u, v) = (1, 1);
                *self = Self::Started { u, v };
                v
            }
            Luby::Started { u, v } => {
                let (us, vs) = (*u as isize, *v as isize);
                if us & -us == vs {
                    *u += 1;
                    *v = 1;
                } else {
                    *v *= 2;
                }
                *v
            }
        }
    }
}

impl Iterator for Luby {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        Some(self.next())
    }
}

#[test]
fn luby() {
    let luby = Luby::new();
    assert_eq!(
        luby.take(15).collect::<Vec<_>>(),
        vec![1, 1, 2, 1, 1, 2, 4, 1, 1, 2, 1, 1, 2, 4, 8]
    );
}
