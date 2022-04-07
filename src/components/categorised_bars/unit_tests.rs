use std::iter::repeat;

use super::{BarGroup, BarLabel};
#[test]
fn bar_group_labels() {
    let blg =
        BarGroup::new("seventies").define_labels(vec![BarLabel::from(1977), BarLabel::from(1978)]);

    let labels: Vec<String> = blg.labels().map(|bl| bl.label.clone()).collect();
    assert_eq!(labels, vec!["1977".to_string(), "1978".to_string()]);
}

#[test]
fn bar_subgroup_label() {
    let outer = BarGroup::new("eras")
        .define_groups(vec![BarGroup::new("seventies")
            .define_labels(vec![BarLabel::from(1977), BarLabel::from(1978)])]);

    let labels: Vec<String> = outer.labels().map(|bl| bl.label.clone()).collect();
    assert_eq!(labels, vec!["1977".to_string(), "1978".to_string()]);
}

#[test]
fn nested_groups() {
    let group = BarGroup::new("A").define_groups(vec![
        BarGroup::new("A1").define_groups(vec![
            BarGroup::new("A11").define_groups(vec![
                BarGroup::new("A111").define_labels(vec![BarLabel::from((1, "A1111"))]),
                BarGroup::new("A112").define_labels(vec![
                    BarLabel::from((2, "A1121")),
                    BarLabel::from((3, "A1122")),
                ]),
            ]),
            BarGroup::new("A12").define_groups(vec![BarGroup::new("A121").define_groups(vec![
                BarGroup::new("A1211").define_groups(vec![
                    BarGroup::new("A12111").define_labels(vec![BarLabel::from((4, "A121111"))]),
                    BarGroup::new("A12112").define_labels(vec![
                        BarLabel::from((5, "A121121")),
                        BarLabel::from((6, "A121122")),
                    ]),
                ]),
            ])]),
        ]),
        BarGroup::new("B").define_labels(vec![BarLabel::from((7, "B1"))]),
        BarGroup::new("C").define_groups(vec![
            BarGroup::new("C1")
                .define_labels(vec![BarLabel::from((8, "C11")), BarLabel::from((9, "C12"))]),
            BarGroup::new("C2").define_labels(vec![
                BarLabel::from((10, "C21")),
                BarLabel::from((11, "C22")),
            ]),
        ]),
        BarGroup::new("D").define_labels(vec![
            BarLabel::from((12, "D1")),
            BarLabel::from((13, "D2")),
            BarLabel::from((14, "D3")),
        ]),
    ]);
    let labels: Vec<String> = group.labels().map(|bl| bl.label.clone()).collect();
    let expected = [
        "A1111", "A1121", "A1122", "A121111", "A121121", "A121122", "B1", "C11", "C12", "C21",
        "C22", "D1", "D2", "D3",
    ]
    .iter()
    .map(|s| String::from(*s))
    .collect::<Vec<String>>();
    assert_eq!(labels, expected);
}

#[test]
fn bar_width() {
    let labels = (1967..=1974).map(|y| y.into()).collect::<Vec<BarLabel>>();
    let group_bar_width = BarGroup::new("years")
        .define_labels(labels)
        .calculate_bar_width(800);
    assert_eq!(group_bar_width, 100)
}

#[cfg(test)]
fn sixties_and_seventies() -> BarGroup {
    let labels_60 = (1967..1970).map(|y| y.into()).collect::<Vec<BarLabel>>();
    let labels_70 = (1970..=1974).map(|y| y.into()).collect::<Vec<BarLabel>>();

    BarGroup::new("years")
        .define_groups(vec![
            BarGroup::new("sixties")
                .with_margins(2, 3, 4)
                .define_labels(labels_60),
            BarGroup::new("seventies")
                .define_labels(labels_70)
                .with_margins(8, 9, 10),
        ])
        .with_margins(5, 6, 7)
}

#[test]
fn bar_width_with_margins() {
    let group = sixties_and_seventies();
    let outer_margin_total = group.margin_total();
    assert_eq!(outer_margin_total, 18);

    let margin_dimension = group.groups().fold(group.margin_total(), |dimension, sg| {
        dimension + sg.margin_total()
    });

    assert_eq!(margin_dimension, 5 + 6 + 7 + 2 + 2 * 3 + 4 + 8 + 4 * 9 + 10);

    let group_bar_width = group.calculate_bar_width(884);
    assert_eq!(group_bar_width, 100);

    let group_bar_width = group.calculate_bar_width(116);
    assert_eq!(group_bar_width, 4);
}

#[test]
fn labels_with_dimensions() {
    let group = sixties_and_seventies();

    let mut dimensions = group.bar_positions(116);

    let ld_67 = dimensions.next().unwrap();
    assert_eq!(ld_67.position_start, 8, "1967 should start at 8");

    let ld_68 = dimensions.next().unwrap();
    assert_eq!(ld_68.position_start, 15, "1968 should start at 15");

    let ld_69 = dimensions.next().unwrap();
    assert_eq!(ld_69.position_start, 22, "1969 should start at 22");

    let ld_70 = dimensions.next().unwrap();
    assert_eq!(ld_70.key, 1970);
    assert_eq!(ld_70.position_start, 44, "1970 should start at 44");

    let mut position = 1;
    let mut result = String::new();
    for bar in group.bar_positions(116) {
        let star_count = bar.position_start - position;
        let stars = repeat("*").take(star_count).collect::<String>();
        result.push_str(stars.as_str());
        let label = format!("{}", bar.key);
        result.push_str(label.as_str());
        position = bar.position_end + 1;
    }
    let star_count = 117 - position;
    let stars = repeat("*").take(star_count).collect::<String>();
    result.push_str(stars.as_str());

    assert_eq!(result, String::from("+++++**1967***1968***1969****++++++********1970*********1971*********1972*********1973*********1974**********+++++++").replace("+", "*"));
}

#[test]
fn group_labels_with_dimensions() {
    let group = sixties_and_seventies();

    // let mut dimensions = group.bar_positions(116);
    let bar_width = group.calculate_bar_width(116);

    let mut subgroup_iter = group.groups();

    let sixties = subgroup_iter.next().unwrap();
    assert_eq!(sixties.width_for_bar_width(bar_width), 24);

    let seventies = subgroup_iter.next().unwrap();
    assert_eq!(seventies.width_for_bar_width(bar_width), 74);

    assert_eq!(group.width_for_bar_width(bar_width), 116);
}
