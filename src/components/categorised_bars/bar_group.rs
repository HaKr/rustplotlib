use std::slice::Iter;

use super::{bar_label::BarLabel, BarPosition};

#[allow(dead_code)]
enum BarLabelChildren {
    SubGroups(Vec<BarGroup>),
    Labels(Vec<BarLabel>),
}

pub struct BarGroup {
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

#[allow(dead_code)]
impl BarGroup {
    pub fn new(label: &str) -> Self {
        let mut result = Self::default();
        result.label = label.to_owned();

        result
    }

    pub fn with_margin_before(mut self, before: usize) -> Self {
        self.margin_before = before;
        self
    }

    pub fn with_margin_after(mut self, after: usize) -> Self {
        self.margin_after = after;
        self
    }

    pub fn with_margin_between(mut self, between: usize) -> Self {
        self.margin_between = between;
        self
    }

    pub fn with_margins(self, before: usize, between: usize, after: usize) -> Self {
        self.with_margin_before(before)
            .with_margin_between(between)
            .with_margin_after(after)
    }

    pub fn define_groups<I: IntoIterator<Item = BarGroup>>(mut self, groups: I) -> Self {
        let mut subgroups = Vec::new();
        for group in groups.into_iter() {
            subgroups.push(group);
        }

        self.children = BarLabelChildren::SubGroups(subgroups);

        self
    }

    pub fn define_labels<I: IntoIterator<Item = BarLabel>>(mut self, labels: I) -> Self {
        let mut itemlabels = Vec::new();
        for label in labels.into_iter() {
            itemlabels.push(label);
        }

        self.children = BarLabelChildren::Labels(itemlabels);

        self
    }

    pub fn labels(&self) -> BarLabelIterator {
        BarLabelIterator::new(self)
    }

    pub fn groups(&self) -> BarGroupIterator {
        BarGroupIterator::new(self)
    }

    pub fn bar_positions(&self, dimension: usize) -> BarPositionIterator {
        let bar_width = self.calculate_bar_width(dimension);

        BarPositionIterator::new(
            self,
            1 + self.margin_before,
            bar_width,
            self.margin_between,
            self.margin_after,
        )
    }

    pub fn child_count(&self) -> usize {
        match &self.children {
            BarLabelChildren::SubGroups(subgroups) => subgroups.len(),
            BarLabelChildren::Labels(labels) => labels.len(),
        }
    }

    pub fn width_for_bar_width(&self, bar_width: usize) -> usize {
        let child_count = self.child_count();
        self.margin_total()
            + match &self.children {
                BarLabelChildren::SubGroups(subgroups) => {
                    subgroups.iter().fold(0, |width, subgroup| {
                        width + subgroup.width_for_bar_width(bar_width)
                    })
                }
                BarLabelChildren::Labels(_) => child_count * bar_width,
            }
    }

    pub fn margin_total(&self) -> usize {
        self.margin_before
            + self.margin_between * usize::max(self.child_count() - 1, 0)
            + self.margin_after
    }

    pub fn calculate_bar_width(&self, dimension: usize) -> usize {
        let margin_dimension = self.groups().fold(self.margin_total(), |dimension, sg| {
            dimension + sg.margin_total()
        });
        let number_of_labels = self.labels().count();
        let width = (dimension - margin_dimension) as f32 / number_of_labels as f32;

        f32::floor(width) as usize
    }
}

pub struct BarGroupIterator<'bli> {
    subgroups_iter: Option<Iter<'bli, BarGroup>>,
}

impl<'bli> BarGroupIterator<'bli> {
    fn new(group: &'bli BarGroup) -> Self {
        let subgroups_iter = match &group.children {
            BarLabelChildren::SubGroups(subgroups) => Some(subgroups.iter()),
            BarLabelChildren::Labels(_) => None,
        };

        Self { subgroups_iter }
    }
}

impl<'bli> Iterator for BarGroupIterator<'bli> {
    type Item = &'bli BarGroup;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(subgroups_iter) = self.subgroups_iter.as_mut() {
            subgroups_iter.next()
        } else {
            None
        }
    }
}

pub struct BarLabelIterator<'bli> {
    subgroups_iter: Option<Iter<'bli, BarGroup>>,
    subgroup_labels_iter: Option<Box<BarLabelIterator<'bli>>>,
    labels_iter: Option<Iter<'bli, BarLabel>>,
}

impl<'bli> BarLabelIterator<'bli> {
    fn new(group: &'bli BarGroup) -> Self {
        let (subgroups_iter, labels_iter) = match &group.children {
            BarLabelChildren::SubGroups(subgroups) => (Some(subgroups.iter()), None),
            BarLabelChildren::Labels(labels) => (None, Some(labels.iter())),
        };

        Self {
            subgroups_iter,
            subgroup_labels_iter: None,
            labels_iter,
        }
    }

    fn next_group(&mut self) -> Option<&'bli BarLabel> {
        if let Some(subgroups_iter) = self.subgroups_iter.as_mut() {
            if let Some(group) = subgroups_iter.next() {
                self.subgroup_labels_iter = Some(Box::new(group.labels()));
                self.next_subgroup_label()
            } else {
                None
            }
        } else {
            None
        }
    }

    fn next_subgroup_label(&mut self) -> Option<&'bli BarLabel> {
        if let Some(subgroup_labels_iter) = self.subgroup_labels_iter.as_deref_mut() {
            if let Some(label) = subgroup_labels_iter.next() {
                Some(label)
            } else {
                self.subgroup_labels_iter = None;
                self.next_group()
            }
        } else {
            None
        }
    }
}

impl<'bli> Iterator for BarLabelIterator<'bli> {
    type Item = &'bli BarLabel;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(labels_iter) = self.labels_iter.as_mut() {
            labels_iter.next()
        } else if self.subgroup_labels_iter.is_some() {
            self.next_subgroup_label()
        } else if self.subgroups_iter.is_some() {
            self.next_group()
        } else {
            None
        }
    }
}

pub struct BarPositionIterator<'bli> {
    subgroups_iter: Option<Iter<'bli, BarGroup>>,
    subgroup_label_dimensions_iter: Option<Box<BarPositionIterator<'bli>>>,
    labels_iter: Option<Iter<'bli, BarLabel>>,
    bar_width: usize,
    position: usize,
    margin_between: usize,
    margin_after: usize,
}

impl<'bli> BarPositionIterator<'bli> {
    fn new(
        group: &'bli BarGroup,
        position: usize,
        bar_width: usize,
        margin_between: usize,
        margin_after: usize,
    ) -> Self {
        let (subgroups_iter, labels_iter) = match &group.children {
            BarLabelChildren::SubGroups(subgroups) => (Some(subgroups.iter()), None),
            BarLabelChildren::Labels(labels) => (None, Some(labels.iter())),
        };

        Self {
            subgroups_iter,
            subgroup_label_dimensions_iter: None,
            labels_iter,
            position,
            bar_width,
            margin_between,
            margin_after,
        }
    }

    fn next_group(&mut self) -> Option<BarPosition> {
        if let Some(subgroups_iter) = self.subgroups_iter.as_mut() {
            if let Some(group) = subgroups_iter.next() {
                self.subgroup_label_dimensions_iter = Some(Box::new(BarPositionIterator::new(
                    group,
                    self.position + group.margin_before,
                    self.bar_width,
                    group.margin_between,
                    group.margin_after,
                )));
                self.next_subgroup_label()
            } else {
                if let Some(subgroup_label_dimensions_iter) =
                    self.subgroup_label_dimensions_iter.as_deref()
                {
                    self.position = subgroup_label_dimensions_iter.position
                        + subgroup_label_dimensions_iter.margin_after;
                }
                None
            }
        } else {
            None
        }
    }

    fn next_subgroup_label(&mut self) -> Option<BarPosition> {
        if let Some(subgroup_labels_iter) = self.subgroup_label_dimensions_iter.as_deref_mut() {
            if let Some(label) = subgroup_labels_iter.next() {
                self.position = label.position_end + 1;
                Some(label)
            } else {
                self.position += subgroup_labels_iter.margin_after + self.margin_between;
                self.subgroup_label_dimensions_iter = None;
                self.next_group()
            }
        } else {
            None
        }
    }
}

impl<'bli> Iterator for BarPositionIterator<'bli> {
    type Item = BarPosition;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(labels_iter) = self.labels_iter.as_mut() {
            if let Some(label) = labels_iter.next() {
                let result = Some(BarPosition {
                    key: label.key,
                    position_start: self.position,
                    position_end: self.position + self.bar_width - 1,
                });
                self.position += self.bar_width + self.margin_between;

                result
            } else {
                self.position += self.margin_after;
                None
            }
        } else if self.subgroup_label_dimensions_iter.is_some() {
            self.next_subgroup_label()
        } else if self.subgroups_iter.is_some() {
            self.next_group()
        } else {
            None
        }
    }
}
