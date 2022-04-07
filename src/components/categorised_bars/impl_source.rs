use std::{
    collections::HashMap, fmt::Display, hash::Hash, iter::repeat, ops::AddAssign, slice::Iter,
};

use crate::{chart::Orientation, OrderedSet};

trait BarLabeledItem {
    fn label(&self) -> &str;
}

#[derive(Default)]
struct BarLabel {
    key: usize,
    label: String,
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

impl BarLabeledItem for BarLabel {
    fn label(&self) -> &str {
        self.label.as_str()
    }
}

enum BarLabelChildren {
    SubGroups(Vec<BarGroup>),
    Labels(Vec<BarLabel>),
}

struct BarGroup {
    pub label: String,
    margin_before: usize,
    margin_after: usize,
    margin_between: usize,
    children: BarLabelChildren,
}

impl Default for BarGroup {
    fn default() -> Self {
        Self {
            label: Default::default(),
            margin_before: Default::default(),
            margin_after: Default::default(),
            margin_between: Default::default(),
            children: BarLabelChildren::SubGroups(Vec::default()),
        }
    }
}

impl BarLabeledItem for BarGroup {
    fn label(&self) -> &str {
        self.label.as_str()
    }
}

