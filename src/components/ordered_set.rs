use std::{collections::HashMap, hash::Hash, ops::Index, slice::Iter};

#[derive(Debug, Default)]
pub struct OrderedSet<O>
where
    O: Default + Hash + Eq,
{
    map: HashMap<O, usize>,
    list: Vec<O>,
}

impl<O> OrderedSet<O>
where
    O: Clone + Default + Hash + Eq,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.list.clear();
        self.map.clear();
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn define_if_not_exist(&mut self, key: &O) -> usize {
        if let Some(index) = self.map.get(&key) {
            *index
        } else {
            let item = (*key).clone();
            let index = self.list.len();
            self.list.push(item.clone());
            self.map.insert(item.clone(), index);

            index
        }
    }

    pub fn index_of(&self, key: &O) -> Option<usize> {
        if let Some(index_ref) = self.map.get(key) {
            Some(*index_ref)
        } else {
            None
        }
    }

    pub fn key(&self, index: usize) -> Option<&O> {
        if index < self.list.len() {
            Some(&self.list[index])
        } else {
            None
        }
    }

    pub fn iter(&self) -> Iter<O> {
        self.list.iter()
    }
}

impl<O> Index<usize> for OrderedSet<O>
where
    O: Default + Hash + Eq,
{
    type Output = O;

    fn index(&self, index: usize) -> &Self::Output {
        &self.list[index]
    }
}

#[cfg(test)]
#[test]
fn ordered_numbers() {
    let mut indices: OrderedSet<u8> = OrderedSet::new();

    for item in [17, 12, 99, 12, 1].iter() {
        indices.define_if_not_exist(item);
    }
    let indices = indices;

    assert_eq!(indices.index_of(&17), Some(0));
    assert_eq!(indices.index_of(&12), Some(1));
    assert_eq!(indices.index_of(&99), Some(2));
    assert_eq!(indices.index_of(&1), Some(3));
    assert_eq!(indices.index_of(&u8::MAX), None);

    assert_eq!(indices.key(0), Some(&17));
    assert_eq!(indices.key(1), Some(&12));
    assert_eq!(indices.key(2), Some(&99));
    assert_eq!(indices.key(3), Some(&1));
    assert_eq!(indices.key(4), None);
    assert_eq!(indices.key(usize::MAX), None);
}

#[cfg(test)]
#[test]
fn ordered_strs() {
    let mut indices: OrderedSet<&str> = OrderedSet::new();

    for item in ["A", "C", "B", "C", "B", "D"].iter() {
        indices.define_if_not_exist(item);
    }

    let indices = indices;

    assert_eq!(indices.index_of(&"A"), Some(0));
    assert_eq!(indices.index_of(&"B"), Some(2));
    assert_eq!(indices.index_of(&"C"), Some(1));
    assert_eq!(indices.index_of(&"D"), Some(3));
    assert_eq!(indices.index_of(&"E"), None);

    assert_eq!(indices.key(0), Some(&"A"));
    assert_eq!(indices.key(1), Some(&"C"));
    assert_eq!(indices.key(2), Some(&"B"));
    assert_eq!(indices.key(3), Some(&"D"));
    assert_eq!(indices.key(4), None);
    assert_eq!(indices.key(usize::MAX), None);
}

#[cfg(test)]
#[test]
fn ordered_strings() {
    let mut indices: OrderedSet<String> = OrderedSet::new();

    let (a, b, c, c2, d, e) = (
        String::from("A"),
        String::from("B"),
        String::from("C"),
        String::from("C"),
        String::from("D"),
        String::from("E"),
    );

    let source = [
        a.clone(),
        c.clone(),
        b.clone(),
        c2.clone(),
        d.clone(),
        c.clone(),
    ];

    for item in source.iter() {
        indices.define_if_not_exist(item);
    }

    let indices = indices;

    assert_eq!(indices.index_of(&a), Some(0));
    assert_eq!(indices.index_of(&b), Some(2));
    assert_eq!(indices.index_of(&c), Some(1));
    assert_eq!(indices.index_of(&d), Some(3));
    assert_eq!(indices.index_of(&e), None);

    assert_eq!(indices.key(0), Some(&a));
    assert_eq!(indices.key(1), Some(&c));
    assert_eq!(indices.key(2), Some(&b));
    assert_eq!(indices.key(3), Some(&d));
    assert_eq!(indices.key(4), None);
    assert_eq!(indices.key(usize::MAX), None);
}

#[cfg(test)]
#[test]
fn iterate() {
    let mut indices: OrderedSet<&str> = OrderedSet::new();

    for item in ["C", "D", "A", "B", "C", "B"].iter() {
        indices.define_if_not_exist(item);
    }

    let mut iter = indices.iter();

    assert_eq!(iter.next(), Some(&"C"));
    assert_eq!(iter.next(), Some(&"D"));
    assert_eq!(iter.next(), Some(&"A"));
    assert_eq!(iter.next(), Some(&"B"));
    assert_eq!(iter.next(), None);
}
