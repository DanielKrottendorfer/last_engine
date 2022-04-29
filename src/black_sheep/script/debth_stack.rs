#[derive(Debug)]
pub struct DebthStack(Vec<usize>);

impl DebthStack {
    pub fn new() -> Self {
        DebthStack(Vec::new())
    }
    pub fn push(&mut self) {
        self.0.push(0);
    }
    pub fn pop(&mut self) -> Option<usize> {
        self.0.pop()
    }
    pub fn advance(&mut self) {
        self.0.iter_mut().for_each(|ds| {
            *ds += 1;
        });
    }
}
