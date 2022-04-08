pub mod band;
pub mod linear;

mod scale_types;
pub(crate) use scale_types::*;

mod iter_scale;
pub(crate) use iter_scale::*;

mod continuous_scale;
pub use continuous_scale::*;

mod linear_scale_iter;
pub use linear_scale_iter::*;

#[derive(PartialEq)]
pub enum ScaleType {
    Band,
    Ordinal,
    Linear,
}

/// The Scale trait defines common operations on all scales.
pub trait Scale<T> {
    /// Get the type of the scale.
    fn get_type(&self) -> ScaleType;

    /// Get the range value for the given domain entry.
    fn scale(&self, domain: &T) -> f32;

    /// Get the bandwidth (if present).
    fn bandwidth(&self) -> Option<f32>;

    /// Get the start range value.
    fn range_start(&self) -> f32;

    /// Get the end range value.
    fn range_end(&self) -> f32;

    /// Check whether the range is in reversed order, meaning the start is greater than the end.
    fn is_range_reversed(&self) -> bool {
        self.range_start() > self.range_end()
    }

    /// Get the list of ticks that represent the scale on a chart axis.
    fn get_ticks(&self) -> Vec<T>;
}
