use std::{
    cmp::Ordering,
    ops::{Add, Sub},
};

use super::{Dimension, IterableScale, LinearScaleIter};

type Discrete = i64;

#[derive(Debug)]
struct DiscreteScale {
    start: Discrete,
    end: Discrete,
    min: Discrete,
    max: Discrete,
    step: Discrete,

    units_per_step: Dimension,
    offset: Dimension,
}

//
impl DiscreteScale {
    pub fn new(dimension: Dimension, start: Discrete, raw_end: Discrete) -> Self {
        let mut end = raw_end;
        let (min, max) = if start <= end {
            if start == end {
                end = end + 1;
            }
            (start, end)
        } else {
            (end, start)
        };

        let size = end - start;
        let dim_64: i64 = dimension.into();
        let (step, units_per_step) = if size > dim_64 {
            (size / dim_64, 1)
        } else {
            (1, dim_64 as u16 / size as u16)
        };

        Self {
            offset: 0,
            start,
            end,
            min,
            max,
            step,
            units_per_step,
        }
    }

    pub fn offset(mut self, offset: Dimension) -> Self {
        self.offset = offset;

        self
    }

    pub fn with_step(mut self, step: Discrete) -> Self {
        let zero: Discrete = Default::default();
        self.step = if step.eq(&zero) { 1_u8.into() } else { step };

        self
    }
}

impl IterableScale<Discrete> for DiscreteScale {
    fn contains(&self, value: Discrete) -> bool {
        self.min < value && value < self.max && (value - self.start) % self.step == 0
    }

    fn scale(&self, value: Discrete) -> Dimension {
        self.offset + ((((value - self.start) / self.step) as u16) * self.units_per_step)
    }

    fn iter<'i>(&'i self) -> Box<dyn Iterator<Item = Discrete> + 'i> {
        Box::new(LinearScaleIter::new(self.start, self.end, self.step))
    }
}

#[cfg(test)]
fn sample<DR>(continuous: &dyn IterableScale<DR>, upper: usize) -> Vec<(DR, Dimension)>
where
    DR: Copy + Default + PartialOrd + PartialEq,
{
    continuous
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            if i < 3 || i > upper {
                Some((x, continuous.scale(x)))
            } else {
                None
            }
        })
        .collect()
}

#[test]
fn iterate_over_continuous_scale() {
    let discrete = DiscreteScale::new(800, 0, 100).offset(400);
    assert_eq!(discrete.scale(25), 600);
    assert_eq!(discrete.scale(75), 1_000);

    assert_eq!(discrete.iter().count(), 101);

    assert_eq!(
        sample(&discrete, 96),
        vec![
            (0, 400,),
            (1, 408,),
            (2, 416,),
            (97, 1176,),
            (98, 1184,),
            (99, 1192,),
            (100, 1200,),
        ]
    );
}

#[test]
fn discrete_2() {
    let discrete = DiscreteScale::new(100, -300, 500);
    // assert_eq!(discrete.scale(75), 1_000);

    assert_eq!(discrete.iter().count(), 101);

    assert_eq!(
        sample(&discrete, 96),
        vec![(-300, 0), (-292, 1), (-284, 2), (476, 97), (484, 98), (492, 99), (500, 100)]
    );
}
