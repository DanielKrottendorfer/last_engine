pub struct UniqueIndexer(usize);

impl UniqueIndexer {
    pub fn new() -> Self {
        UniqueIndexer(0)
    }
    pub fn get_next(&mut self) -> usize {
        let t = self.0;
        self.0 += 1;
        t
    }
}
