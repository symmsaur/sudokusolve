#[derive(Debug)]
pub struct EliminationError {}

#[derive(Clone)]
pub struct Cell {
    possibles: Vec<i32>,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            possibles: (1..10).collect(),
        }
    }

    pub fn set_hint(self: &mut Cell, hint: i32) {
        self.possibles.clear();
        self.possibles.push(hint);
    }

    pub fn eliminate_possible(self: &mut Cell, digit: i32) -> Result<(), EliminationError> {
        self.possibles.retain(|i| *i != digit);
        if self.possibles.len() == 0 {
            Err(EliminationError {})
        } else {
            Ok(())
        }
    }

    pub fn num_possibles(&self) -> usize {
        self.possibles.len()
    }

    pub fn first_possible(&self) -> Option<i32> {
        self.possibles.first().copied()
    }

    pub fn possibles_except(&self, except: i32) -> Vec<i32> {
        self.possibles
            .iter()
            .cloned()
            .filter(|x| *x != except)
            .collect()
    }

    pub fn is_possible(&self, digit: i32) -> bool {
        self.possibles.contains(&digit)
    }
}
