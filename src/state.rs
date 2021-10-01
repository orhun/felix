pub struct Num {
    pub index: usize,
    pub skip: u16,
}

impl Num {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn go_up(&mut self) {
        self.index -= 1;
    }
    pub fn go_down(&mut self) {
        self.index += 1;
    }
    pub fn go_top(&mut self) {
        self.index = 0;
    }
    pub fn go_bottom(&mut self, len: usize) {
        self.index = len;
    }
    pub fn reset(&mut self) {
        self.index = 1;
        self.skip = 0;
    }
    pub fn starting_point(&mut self) {
        self.index = 1;
    }
    pub fn inc_skip(&mut self) {
        self.skip += 1;
    }
    pub fn dec_skip(&mut self) {
        self.skip -= 1;
    }
    pub fn reset_skip(&mut self) {
        self.skip = 0;
    }
}

impl Default for Num {
    fn default() -> Self {
        Num { index: 1, skip: 0 }
    }
}
