use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct LinearScaleIter<DR>
where
    DR: Copy + PartialOrd + Default + Sub<DR, Output = DR>,
{
    start: DR,
    end: DR,
    step: DR,
    current: Option<DR>,
    is_reversed: bool,
}

impl<DR> LinearScaleIter<DR>
where
    DR: Copy + PartialOrd + Default + Sub<DR, Output = DR>,
{
    pub fn new(start: DR, end: DR, step: DR) -> Self {
        let zero: DR = Default::default();
        let is_reversed = step < zero;

        Self {
            start,
            end,
            step,
            is_reversed,
            current: None,
        }
    }
}

impl<DR> Iterator for LinearScaleIter<DR>
where
    DR: Copy + PartialOrd + Default + Add<DR, Output = DR> + Sub<DR, Output = DR>,
{
    type Item = DR;

    fn next(&mut self) -> Option<Self::Item> {
        let next = if let Some(current) = self.current {
            current + self.step
        } else {
            self.start
        };
        self.current =
            if (self.is_reversed && next >= self.end) || (!self.is_reversed && next <= self.end) {
                Some(next)
            } else {
                None
            };

        self.current
    }
}
