use bitmaps::Bitmap;

#[derive(Debug)]
pub struct EliminationError {}

#[derive(Clone)]
pub struct Cell {
    possibles: Bitmap<9>,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            possibles: Bitmap::mask(9)
        }
    }

    pub fn set_hint(self: &mut Cell, hint: i32) {
        self.possibles = Bitmap::mask(0);
        self.possibles.set((hint - 1) as usize, true);
    }

    pub fn eliminate_possible(self: &mut Cell, digit: i32) -> Result<(), EliminationError> {
        self.possibles.set((digit - 1) as usize, false);
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
        self.possibles.first_index().map(|i| (i + 1) as i32)
    }

    pub fn possibles_except(&self, except: i32) -> Vec<i32> {
        self.possibles
            .into_iter()
            .filter(|x| *x != (except - 1) as usize)
            .map(|i| (i + 1) as i32)
            .collect()
    }

    pub fn is_possible(&self, digit: i32) -> bool {
        self.possibles.get((digit - 1) as usize)
    }
}
