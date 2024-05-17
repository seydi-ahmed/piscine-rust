#[derive(Debug, Clone)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Numbers { numbers }
    }

    pub fn list(&self) -> &[u32] {
        &self.numbers[0..]
    }

    pub fn latest(&self) -> Option<u32> {
        self.numbers.iter().last().copied()
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().copied()
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut cp: Vec<u32> = Vec::new();
        for i in self.numbers {
            cp.push(*i);
        }
        cp.sort_by(|a, b| b.cmp(a));
        if cp.len() < 3 {
            return cp.to_vec();
        }
        cp[0..3].to_vec()
    }
}
