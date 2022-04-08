use super::{Continuous, Dimension, IterableScale, LinearScaleIter};

#[derive(Debug)]
struct ContinuousScale {
    start: Continuous,
    end: Continuous,
    min: Continuous,
    max: Continuous,
    size_dimension_ratio: Continuous,
    dimension_size_ratio: Continuous,

    offset: Dimension,
}

impl ContinuousScale {
    pub fn new(dimension: Dimension, start: Continuous, end: Continuous) -> Self {
        let (min, max) = if start < end {
            (start, end)
        } else {
            (end, start)
        };

        let size_float: Continuous = end - start;
        let dimension_float: Continuous = dimension.into();

        let size_dimension_ratio = size_float / dimension_float;
        let dimension_size_ratio = dimension_float / size_float;

        Self {
            offset: 0,
            start,
            end,
            min,
            max,
            size_dimension_ratio,
            dimension_size_ratio,
        }
    }

    pub fn offset(mut self, offset: Dimension) -> Self {
        self.offset = offset;

        self
    }
}

impl IterableScale<Continuous> for ContinuousScale {
    fn contains(&self, value: Continuous) -> bool {
        self.min < value && value < self.max
    }

    fn scale(&self, value: Continuous) -> Dimension {
        let distance = value - self.start;
        let distance_float: Continuous = distance;
        let scaled: Dimension =
            Continuous::round(distance_float * self.dimension_size_ratio) as Dimension;

        self.offset + scaled
    }

    fn iter<'i>(&'i self) -> Box<dyn Iterator<Item = Continuous> + 'i> {
        Box::new(LinearScaleIter::new(
            self.start,
            self.end,
            self.size_dimension_ratio,
        ))
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
    let continuous = ContinuousScale::new(800, 0.0, 360.0).offset(400);
    assert_eq!(continuous.iter().count(), 800);
    let expected = vec![
        (0.0, 400),
        (0.45, 401),
        (0.9, 402),
        (358.65158, 1197),
        (359.1016, 1198),
        (359.5516, 1199),
    ];
    assert_eq!(sample(&continuous, 796), expected);
}

#[test]
fn iterate_over_continuous_mirrored_scale() {
    let continuous = ContinuousScale::new(720, 360.0, 0.0);
    assert_eq!(continuous.iter().count(), 721);

    let expected = vec![
        (360.0, 0),
        (359.5, 1),
        (359.0, 2),
        (1.5, 717),
        (1.0, 718),
        (0.5, 719),
        (0.0, 720),
    ];

    assert_eq!(sample(&continuous, 716), expected);
}

#[test]
fn iterate_over_continuous_mirrored_plus_minus_scale() {
    let continuous = ContinuousScale::new(720, 360.0, -360.0);
    assert_eq!(continuous.iter().count(), 721);

    let expected = vec![
        (360.0, 0),
        (359.0, 1),
        (358.0, 2),
        (-357.0, 717),
        (-358.0, 718),
        (-359.0, 719),
        (-360.0, 720),
    ];

    assert_eq!(sample(&continuous, 716), expected);
}

#[test]
fn iterate_over_continuous_plus_minus_scale() {
    let continuous = ContinuousScale::new(300, -500.0, 500.0);
    assert_eq!(continuous.iter().count(), 300);

    let expected = vec![
        (-500.0, 0),
        (-496.66666, 1),
        (-493.3333, 2),
        (490.00116, 297),
        (493.3345, 298),
        (496.66785, 299),
    ];

    assert_eq!(sample(&continuous, 296), expected);
}
