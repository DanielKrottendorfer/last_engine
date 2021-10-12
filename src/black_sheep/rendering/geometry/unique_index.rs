
pub struct UniqueIndex(usize);

impl UniqueIndex {
    pub fn new() -> Self {
        UniqueIndex(0)
    }
    pub fn get_next(&mut self) -> usize {
        let t = self.0;
        self.0+=1;
        t
    }
}
