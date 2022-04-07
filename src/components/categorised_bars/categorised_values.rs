use std::{
    collections::{btree_map::Iter, BTreeMap},
    fmt::Display,
    hash::Hash,
    ops::AddAssign,
};

use super::{categorised_value::CategorisedValue, segmented_value::SegmentedValue};
use crate::components::OrderedSet;

#[derive(Default)]
/// Base for collecting values per category and optionally per segment
///
/// The values to collect are categorised by a key that must implement
/// the [Clone], [Default], [Display], [Hash] and [Eq] traits.
///
/// Optionally, each category can also be subdivided in segments.
/// The segment key must also implement the above mentioned traits.
///
/// The values must implement the [AddAssign], [Copy], [Default] and [Into]<[JsonValue]> traits.
///
/// # Example
/// ```rust
/// # use charts::CategorisedValues;
///
/// let categorised = CategorisedValues::new()
///             // optionally define the order of the categories. The categories must have
///             // the same type as the ones in add_data
///            .with_categories(1970..2000_i16)
///             // Also optional the order of the segments can be predefined.
///             // Again, the type of the segments must be equal to that used in add_data
///            .with_segments(vec!["8 - Track", "LP/EP", "Cassette", "DVD Audio", "CD"])
///             // add some data....
///            .add_data(vec![
///                (1977, "Cassette", 36_900_000),
///                (1977_i16, "8 - Track", 127_300_000),
///                (1979, "8 - Track", 102_300_000),
///                (1978, "8 - Track", 133_600_000),
///                (1978, "Cassette", 61_300_000),
///                (1979, "Cassette", 78_500_000),
///            ])
///             // ...and some more. Calling add_data multiple times is allowed
///             // as long as the types of the categories, the segments and data values
///             // are the same in all calls.
///            .add_data(vec![
///                (2000, "CD", 942_500_000),
///                (2000, "DVD Audio", 1_000),
///                (2000, "Cassette", 76_000_000),
///                (2010, "DVD Audio", 40_000),
///                (2010, "CD", 253_000_000),
///            ]);
///
/// let expected = "{\n\t1977: { 8 - Track: 127300000, Cassette: 36900000 },\n\t1978: { 8 - Track: 133600000, Cassette: 61300000 },\n\t1979: { 8 - Track: 102300000, Cassette: 78500000 },\n\t2000: { Cassette: 76000000, DVD Audio: 1000, CD: 942500000 },\n\t2010: { DVD Audio: 40000, CD: 253000000 }\n }";
///
/// assert_eq!(categorised.to_string(), expected );
/// ```
pub struct CategorisedValues<CAT, SEG, VAL>
where
    CAT: Clone + Default + Display + Hash + Eq,
    SEG: Clone + Default + Display + Hash + Eq,
    VAL: AddAssign<VAL> + Copy + Default + Display,
{
    category_keys: OrderedSet<CAT>,
    segment_keys: OrderedSet<SEG>,
    values: BTreeMap<usize, SegmentedValue<VAL>>,
}

impl<CAT, SEG, VAL> CategorisedValues<CAT, SEG, VAL>
where
    CAT: Clone + Default + Display + Hash + Eq,
    SEG: Clone + Default + Display + Hash + Eq,
    VAL: AddAssign<VAL> + Copy + Default + Display,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_categories<I: IntoIterator<Item = CAT>>(mut self, keys: I) -> Self {
        self.category_keys.clear();
        for key in keys.into_iter() {
            self.category_keys.define_if_not_exist(&key);
        }
        self
    }

    pub fn with_segments<I: IntoIterator<Item = SEG>>(mut self, keys: I) -> Self {
        self.segment_keys.clear();
        for key in keys.into_iter() {
            self.segment_keys.define_if_not_exist(&key);
        }
        self
    }

    /// Add a collection of categorised data into this one
    ///
    /// The data comes from a collection that can be iterated over,
    /// where each item is something that supports the [Into]<[CategorisedValue]> trait.
    ///
    /// # Example
    /// ```rust
    /// # use charts::CategorisedValues;
    ///
    /// let frequencies = CategorisedValues::new()
    ///        .with_categories('a'..'z')
    ///        .add_data("hello world".chars().filter(|c| c.is_alphabetic()));
    ///
    /// let expected = r#"{ "d": 1, "e": 1, "h": 1, "l": 3, "o": 2, "r": 1, "w": 1 }"#;
    ///
    /// //assert_eq!( frequencies.to_string(), expected );
    ///
    /// ```
    pub fn add_data<T: IntoIterator<Item = impl Into<CategorisedValue<CAT, SEG, VAL>>>>(
        mut self,
        collection: T,
    ) -> Self {
        for def in collection.into_iter() {
            let bar_definition: CategorisedValue<CAT, SEG, VAL> = def.into();
            let bar_index = self
                .category_keys
                .define_if_not_exist(&bar_definition.category_key);
            let stack_index = self
                .segment_keys
                .define_if_not_exist(&bar_definition.segment_key);
            self.add_to_category(bar_index, stack_index, bar_definition.value);
        }

        self
    }

    pub fn categories<'i>(&'i self) -> Iter<'i, usize, SegmentedValue<VAL>> {
        self.values.iter()
    }

    /// Closure that maps category indices to their corresponding label value
    ///
    /// ```rust
    /// # use charts::CategorisedValues;
    ///
    /// let categorised = CategorisedValues::new()
    ///     .with_categories('a'..'z')
    ///     .add_data("hello world".chars().filter(|c| c.is_alphabetic()));
    ///
    /// let (l, l_category) = categorised
    ///     .categories()
    ///     .skip(3)
    ///     .map(categorised.category_index_to_label())
    ///     .next()
    ///     .unwrap();
    ///
    /// assert_eq!(*l, 'l');
    /// ```
    pub fn category_index_to_label<'m>(
        &'m self,
    ) -> impl Fn((&usize, &'m SegmentedValue<VAL>)) -> (&'m CAT, &'m SegmentedValue<VAL>) {
        move |(category_index, category)| (&self.category_keys[*category_index], category)
    }

    /// Closure that maps segment indices to their corresponding label value
    ///
    /// ```rust
    /// # use charts::CategorisedValues;
    ///
    /// let categorised = CategorisedValues::new().add_data(vec![
    ///     ("A", "x", 11_u16),
    ///     ("B", "y", 13),
    ///     ("C", "z", 17),
    ///     ("A", "y", 19),
    ///     ("B", "z", 23),
    ///     ("C", "x", 29),
    ///     ("A", "z", 31),
    ///     ("B", "x", 37),
    ///     ("C", "y", 41),
    ///     ("A", "y", 43),
    /// ]);
    ///
    /// assert_eq!(
    ///     categorised
    ///         .categories()
    ///         .next()
    ///         .unwrap()
    ///         .1
    ///         .values()
    ///         .map(categorised.segment_index_to_label())
    ///         .skip(1)
    ///         .next()
    ///         .unwrap(),
    ///     (&"y", &(19 + 43))
    /// );
    /// ```
    pub fn segment_index_to_label<'m>(
        &'m self,
    ) -> impl Fn((&usize, &'m VAL)) -> (&'m SEG, &'m VAL) {
        move |(segment_index, val)| (&self.segment_keys[*segment_index], val)
    }

    fn add_to_category(&mut self, bar_index: usize, stack_index: usize, value: VAL) {
        self.values
            .entry(bar_index)
            .or_insert(SegmentedValue::default())
            .add(stack_index, value);
    }

    /// String representation of a categorised values collection
    ///
    /// rust
    /// # use charts::CategorisedValues;
    ///
    /// assert_eq!(
    ///     CategorisedValues::new()
    ///         .add_data("hello world".chars().filter(|c| c.is_alphabetic()))
    ///         .to_string(),
    ///     String::from(r#"{"h":1,"e":1,"l":3,"o":2,"w":1,"r":1,"d":1}"#)
    /// );
    ///
    /// assert_eq!(
    ///     CategorisedValues::new()
    ///         .with_segments(1..=1) // force two defined segments (0 added by data)
    ///         .add_data("hello world".chars().filter(|c| c.is_alphabetic()))
    ///         .to_string(),
    ///     String::from(
    ///         r#"{"h":{"0":1},"e":{"0":1},"l":{"0":3},"o":{"0":2},"w":{"0":1},"r":{"0":1},"d":{"0":1}}"#
    ///     )
    /// );
    ///
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}

//#[cfg(any(test, doctest))]
impl<CAT, SEG, VAL> Display for CategorisedValues<CAT, SEG, VAL>
where
    CAT: Clone + Default + Display + Hash + Eq,
    SEG: Clone + Default + Display + Hash + Eq,
    VAL: AddAssign<VAL> + Copy + Default + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let categories_count = self.categories().len();
        let is_empty = categories_count < 1;

        if is_empty {
            f.write_str("{}")
        } else {
            let values_only = self.segment_keys.len() < 2;
            let last_index = categories_count - 1;

            f.write_str("{\n")?;

            for (index, (cat_label, cat)) in self
                .categories()
                .map(self.category_index_to_label())
                .enumerate()
            {
                f.write_fmt(format_args!("\t{}: ", cat_label))?;

                if !values_only {
                    f.write_str("{ ")?;
                }

                let mut write_seg_separator = false;
                for (seg_label, val) in cat.values().map(self.segment_index_to_label()) {
                    if write_seg_separator {
                        f.write_str(", ")?;
                    } else {
                        write_seg_separator = true;
                    }

                    if !values_only {
                        f.write_fmt(format_args!("{}: ", seg_label))?;
                    }

                    f.write_fmt(format_args!("{}", val))?;
                }

                if !values_only {
                    f.write_str(" }")?;
                }

                if index < last_index {
                    f.write_str(",")?;
                }
                f.write_str("\n")?;
            }

            f.write_str(" }")
        }
    }
}

#[cfg(test)]
fn assert_output_eq<CAT, SEG, VAL>(
    categorised_values: CategorisedValues<CAT, SEG, VAL>,
    json_str: &str,
) where
    CAT: Clone + Default + Display + Hash + Eq,
    SEG: Clone + Default + Display + Hash + Eq,
    VAL: AddAssign<VAL> + Copy + Default + Display,
{
    assert_eq!(
        categorised_values
            .to_string()
            .replace("\n", "")
            .replace("\t", " "),
        String::from(json_str).replace("\n", "").replace("  ", " ")
    );
}

#[test]
fn empty_string_when_no_data() {
    assert_output_eq(CategorisedValues::<i8, i8, f32>::new(), "{}");
}

#[test]
fn categories_only() {
    assert_output_eq(
        CategorisedValues::new().add_data(vec![("C", 10_u16), ("B", 20), ("A", 30)]),
        "{ C: 10, B: 20, A: 30 }",
    )
}

#[test]
fn histogram() {
    assert_output_eq(
        CategorisedValues::new().add_data(vec!["A", "B", "A", "C", "A", "C", "A", "A"]),
        "{ A: 5, B: 1, C: 2 }",
    )
}

#[test]
fn ordered_categories_only() {
    assert_output_eq(
        CategorisedValues::new()
            .with_categories(vec!["B", "C", "A"])
            .add_data(vec![("C", 10_u16), ("B", 20), ("A", 30)]),
        "{ B: 20, C: 10, A: 30 }",
    );
}

#[test]
fn ordered_categories_with_multiple_entries() {
    assert_output_eq(
        CategorisedValues::new()
            .with_categories(vec!["A", "B", "C"])
            .add_data(vec![
                ("C", 10_u16),
                ("B", 20),
                ("A", 30),
                ("A", 10),
                ("B", 20),
                ("C", 30),
            ]),
        "{ A: 40, B: 40, C: 40 }",
    );
}

#[test]
fn categories_with_segments() {
    assert_output_eq(
        CategorisedValues::new().add_data(vec![
            ("C", 12_u32, 10_u16),
            ("B", 10_u32, 20),
            ("A", 11_u32, 30),
        ]),
        "{ C: { 12: 10 }, B: { 10: 20 }, A: { 11: 30 } }",
    );
}

#[test]
fn categories_with_multiple_segments() {
    assert_output_eq(
        CategorisedValues::new().add_data(vec![
            ("A", "x", 11_u16),
            ("B", "y", 13),
            ("C", "z", 17),
            ("A", "y", 19),
            ("B", "z", 23),
            ("C", "x", 29),
            ("A", "z", 31),
            ("B", "x", 37),
            ("C", "y", 41),
            ("A", "y", 43),
        ]),
        "{ A: { x: 11, y: 62, z: 31 }, B: { x: 37, y: 13, z: 23 }, C: { x: 29, y: 41, z: 17 } }",
    );
}

#[test]
fn iterate_categories_and_segments() {
    let categorised = CategorisedValues::new()
        .with_categories(1970..2000_i16)
        .with_segments(vec!["8 - Track", "LP/EP", "Cassette", "DVD Audio", "CD"])
        .add_data(vec![
            (1977_i16, "Cassette", 36_900_000_i32),
            (1977, "8 - Track", 127_300_000),
            (1979, "8 - Track", 102_300_000),
            (1978, "8 - Track", 133_600_000),
            (1978, "Cassette", 61_300_000),
            (1979, "Cassette", 78_500_000),
        ])
        .add_data(vec![
            (2000_i16, "CD", 942_500_000),
            (2000, "DVD Audio", 1_000),
            (2000, "Cassette", 76_000_000),
            (2010, "DVD Audio", 40_000),
            (2010, "CD", 253_000_000),
        ]);

    let mut categories = categorised
        .categories()
        .map(categorised.category_index_to_label());

    let (category_index, category) = categories.next().unwrap();
    assert_eq!(category.height(), 36_900_000 + 127_300_000);
    assert_eq!(*category_index, 1977);
    assert!(category.has_values());

    let mut segments_iter = category.values().map(categorised.segment_index_to_label());

    let (segment_label, segment_value) = segments_iter.next().unwrap();
    assert_eq!(*segment_value, 127_300_000);
    assert_eq!(*segment_label, "8 - Track");

    let (segment_label, segment_value) = segments_iter.next().unwrap();
    assert_eq!(*segment_value, 36_900_000);
    assert_eq!(*segment_label, "Cassette");

    assert_eq!(segments_iter.next(), None);

    let (category_index, category) = categories.next().unwrap();
    assert_eq!(category.height(), 133_600_000 + 61_300_000);
    assert_eq!(*category_index, 1978);
    assert!(category.has_values());
}

#[test]
fn iterate_frequencies() {
    let categorised = CategorisedValues::new()
        .with_categories('a'..'z')
        .add_data("hello world".chars().filter(|c| c.is_alphabetic()));

    let mut categories = categorised
        .categories()
        .map(categorised.category_index_to_label());

    let (category_label, category) = categories.next().unwrap();
    assert_eq!(category.height(), 1);
    assert_eq!(*category_label, 'd');
    assert!(category.has_values());

    let (l, l_category) = categorised
        .categories()
        .skip(3)
        .map(categorised.category_index_to_label())
        .next()
        .unwrap();
    assert_eq!(*l, 'l');
    assert_eq!(l_category.height(), 3);
    assert!(l_category.has_values());
}

#[test]
fn dbg() {
    let categorised = CategorisedValues::new()
        .with_categories('a'..'z')
        .add_data("hello world".chars().filter(|c| c.is_alphabetic()));

    println!("{}", categorised);
}

#[test]
fn to_string() {
    let categorised = CategorisedValues::new()
        .with_categories(1970..2000_i16)
        .with_segments(vec!["8 - Track", "LP/EP", "Cassette", "DVD Audio", "CD"])
        .add_data(vec![
            (1977_i16, "Cassette", 36_900_000_i32),
            (1977, "8 - Track", 127_300_000),
            (1979, "8 - Track", 102_300_000),
            (1978, "8 - Track", 133_600_000),
            (1978, "Cassette", 61_300_000),
            (1979, "Cassette", 78_500_000),
        ])
        .add_data(vec![
            (2000_i16, "CD", 942_500_000),
            (2000, "DVD Audio", 1_000),
            (2000, "Cassette", 76_000_000),
            (2010, "DVD Audio", 40_000),
            (2010, "CD", 253_000_000),
        ]);

    println!("{}", categorised);
}
