#[derive(Debug)]
pub struct DebthStack(Vec<(usize, usize)>);

impl DebthStack {
    pub fn new() -> Self {
        DebthStack(Vec::new())
    }
    pub fn push(&mut self, len: usize) {
        self.0.push((len, len));
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn advance(&mut self) -> u32 {
        let mut i = 0;
        self.0 = self
            .0
            .drain(0..)
            .filter_map(|ds| {
                if ds.0 == 0 {
                    i += 1;
                    None
                } else {
                    Some((ds.0 - 1, ds.1))
                }
            })
            .collect();
        i
    }
    pub fn iter(&self) -> DebthStackIterator {
        DebthStackIterator {
            data: &self.0,
            i: 0,
        }
    }
}

pub struct DebthStackIterator<'a> {
    data: &'a Vec<(usize, usize)>,
    i: usize,
}

impl<'a> Iterator for DebthStackIterator<'a> {
    type Item = &'a (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.i;
        self.i += 1;
        self.data.get(temp)
    }
}
