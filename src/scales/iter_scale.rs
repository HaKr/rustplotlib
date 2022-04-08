use super::Dimension;

pub trait IterableScale<DR>
where
    DR: Copy + Default + PartialOrd + PartialEq,
{
    fn contains(&self, value: DR) -> bool;

    fn scale(&self, value: DR) -> Dimension;

    fn iter<'i>(&'i self) -> Box<dyn Iterator<Item = DR> + 'i>;
}
