use std::{fmt::Display, hash::Hash, ops::AddAssign};

#[derive(Default)]
pub struct CategorisedValue<CAT, SEG, VAL>
where
    CAT: Clone + Default + Display + Hash + Eq,
    SEG: Clone + Default + Display + Hash + Eq,
    VAL: AddAssign<VAL> + Copy + Default + Display,
{
    pub category_key: CAT,
    pub segment_key: SEG,
    pub value: VAL,
}

impl<CAT> From<CAT> for CategorisedValue<CAT, usize, usize>
where
    CAT: Clone + Default + Display + Hash + Eq,
{
    fn from(definition: CAT) -> Self {
        CategorisedValue::new()
            .bar_key(definition)
            .stack_key(0)
            .value(1)
    }
}

impl<CAT, VAL> From<(CAT, VAL)> for CategorisedValue<CAT, usize, VAL>
where
    CAT: Clone + Default + Display + Hash + Eq,
    VAL: AddAssign<VAL> + Copy + Default + Display,
{
    fn from(definition: (CAT, VAL)) -> Self {
        CategorisedValue::new()
            .bar_key(definition.0)
            .stack_key(0)
            .value(definition.1)
    }
}

impl<CAT, SEG, VAL> From<(CAT, SEG, VAL)> for CategorisedValue<CAT, SEG, VAL>
where
    CAT: Clone + Default + Display + Hash + Eq,
    SEG: Clone + Default + Display + Hash + Eq,
    VAL: AddAssign<VAL> + Copy + Default + Display,
{
    fn from(definition: (CAT, SEG, VAL)) -> Self {
        CategorisedValue::new()
            .bar_key(definition.0)
            .stack_key(definition.1)
            .value(definition.2)
    }
}

impl<CAT, SEG, VAL> CategorisedValue<CAT, SEG, VAL>
where
    CAT: Clone + Default + Display + Hash + Eq,
    SEG: Clone + Default + Display + Hash + Eq,
    VAL: AddAssign<VAL> + Copy + Default + Display,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bar_key(mut self, key: CAT) -> Self {
        self.category_key = key;

        self
    }

    pub fn stack_key(mut self, key: SEG) -> Self {
        self.segment_key = key;

        self
    }

    pub fn value(mut self, value: VAL) -> Self {
        self.value = value;

        self
    }
}
