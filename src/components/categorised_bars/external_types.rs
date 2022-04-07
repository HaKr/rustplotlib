pub use super::bar_group::BarGroup;
pub use super::bar_label::BarLabel;
pub use super::categorised_value::CategorisedValue;
pub use super::categorised_values::CategorisedValues;

#[derive(Debug)]
pub struct BarPosition {
    pub key: usize,
    pub position_start: usize,
    pub position_end: usize,
}
