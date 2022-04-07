use std::{
    collections::{btree_map::Iter, BTreeMap},
    ops::AddAssign,
};

#[derive(Debug, Default)]
pub struct SegmentedValue<VAL>
where
    VAL: AddAssign<VAL> + Copy + Default,
{
    segments: BTreeMap<usize, VAL>,
    magnitude: VAL,
}

impl<VAL> SegmentedValue<VAL>
where
    VAL: AddAssign<VAL> + Copy + Default,
{
    pub fn add(&mut self, segment_index: usize, value: VAL) {
        self.magnitude += value;
        *self
            .segments
            .entry(segment_index)
            .or_insert(Default::default()) += value;
    }

    pub fn value_of_segment(&self, segment_index: usize) -> Option<VAL> {
        if let Some(segment) = self.segments.get(&segment_index) {
            Some(*segment)
        } else {
            None
        }
    }

    pub fn has_values(&self) -> bool {
        self.segments.len() > 0
    }

    pub fn height(&self) -> VAL {
        self.magnitude
    }

    pub fn values<'s>(&'s self) -> Iter<'s, usize, VAL> {
        self.segments.iter()
    }
}
