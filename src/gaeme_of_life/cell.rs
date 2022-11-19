#[derive(Clone)]
pub struct CellSet {
    pub cells: Vec<u8>,
}

impl CellSet {
    pub fn new(count: usize) -> Self {
        Self {
            cells: vec![0; count],
        }
    }

    pub fn bit_get(&self, index: usize) -> bool {
        self.cells[index] != 0
    }

    pub fn bit_set(&mut self, index: usize, alive: bool) -> bool {
        let ret = self.cells[index] != 0;
        self.cells[index] = if alive { 255 } else { 0 };
        ret
    }
}
