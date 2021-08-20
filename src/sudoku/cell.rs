use super::candidates::Candidates;

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    value: u8,
    candidates: Candidates,
    frozen: bool,
}

impl Cell {
    pub fn new(value: u8) -> Self {
        Cell {
            value,
            candidates: Candidates::new(value == 0),
            frozen: false,
        }
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn candidates(&self) -> &Candidates {
        &self.candidates
    }

    pub fn candidates_mut(&mut self) -> &mut Candidates {
        &mut self.candidates
    }

    pub fn freeze(&mut self) {
        self.frozen = true;
    }

    pub fn thaw(&mut self) {
        self.frozen = false;
    }

    pub fn frozen(&self) -> bool {
        self.frozen
    }
}
