use std::fmt::Display;

#[derive(Default)]
pub struct BarLabel {
    pub key: usize,
    pub label: String,
}

impl From<usize> for BarLabel {
    fn from(key: usize) -> Self {
        BarLabel {
            key,
            label: format!("{}", key),
        }
    }
}

impl<D: Display> From<(usize, D)> for BarLabel {
    fn from(data: (usize, D)) -> Self {
        BarLabel {
            key: data.0 as usize,
            label: format!("{}", data.1),
        }
    }
}
