pub struct StepIterator<T> {
    beg: T,
    end: T,
    step: T,
}

use std::ops::Add;
impl<T> StepIterator<T>
where
    T: Add<Output = T> + PartialOrd + Copy,
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator { beg, end, step }
    }
}

impl<T> std::iter::Iterator for StepIterator<T>
where
    T: Add<Output = T> + PartialOrd + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.beg < self.end {
            let next_value = self.beg;
            self.beg = self.beg + self.step;
            if next_value >= self.end {
                Some(next_value)
            } else {
                Some(next_value)
            }
        } else {
            None
        }
    }
}
