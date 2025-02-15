use svg::node::element::Group;

pub(crate) mod area;
pub(crate) mod axis;
pub(crate) mod bar;
pub(crate) mod categorised_bars;
pub(crate) mod legend;
pub(crate) mod line;
mod ordered_set;
pub(crate) use ordered_set::OrderedSet;
pub(crate) mod scatter;

/// A trait that defines behavior of chart components.
pub trait DatumRepresentation {
    fn to_svg(&self) -> Result<Group, String>;
}
