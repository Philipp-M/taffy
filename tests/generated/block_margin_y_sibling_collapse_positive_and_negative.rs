#[test]
fn block_margin_y_sibling_collapse_positive_and_negative() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, tree::Layout};
    let mut taffy = taffy::Taffy::new();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
            margin: taffy::geometry::Rect {
                left: zero(),
                right: zero(),
                top: taffy::style::LengthPercentageAuto::Length(10f32),
                bottom: taffy::style::LengthPercentageAuto::Length(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
            margin: taffy::geometry::Rect {
                left: zero(),
                right: zero(),
                top: taffy::style::LengthPercentageAuto::Length(-10f32),
                bottom: taffy::style::LengthPercentageAuto::Length(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
            margin: taffy::geometry::Rect {
                left: zero(),
                right: zero(),
                top: taffy::style::LengthPercentageAuto::Length(-5f32),
                bottom: taffy::style::LengthPercentageAuto::Length(5f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
            margin: taffy::geometry::Rect {
                left: zero(),
                right: zero(),
                top: taffy::style::LengthPercentageAuto::Length(-10f32),
                bottom: taffy::style::LengthPercentageAuto::Length(-10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
            margin: taffy::geometry::Rect {
                left: zero(),
                right: zero(),
                top: taffy::style::LengthPercentageAuto::Length(10f32),
                bottom: taffy::style::LengthPercentageAuto::Length(-10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node5 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
            margin: taffy::geometry::Rect {
                left: zero(),
                right: zero(),
                top: taffy::style::LengthPercentageAuto::Length(5f32),
                bottom: taffy::style::LengthPercentageAuto::Length(-5f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node6 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
            margin: taffy::geometry::Rect {
                left: zero(),
                right: zero(),
                top: taffy::style::LengthPercentageAuto::Length(10f32),
                bottom: taffy::style::LengthPercentageAuto::Length(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Block,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Length(50f32), height: auto() },
                ..Default::default()
            },
            &[node0, node1, node2, node3, node4, node5, node6],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();
    println!("\nComputed tree:");
    taffy::util::print_tree(&taffy, node);
    println!();
    let Layout { size, location, .. } = taffy.layout(node).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node, 50f32, size.width);
    assert_eq!(size.height, 90f32, "height of node {:?}. Expected {}. Actual {}", node, 90f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node0).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node0, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node0, 10f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node1).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node1, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node1, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 20f32, "y of node {:?}. Expected {}. Actual {}", node1, 20f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node2).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node2, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node2, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 35f32, "y of node {:?}. Expected {}. Actual {}", node2, 35f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node3).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node3, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node3, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node3, 0f32, location.x);
    assert_eq!(location.y, 40f32, "y of node {:?}. Expected {}. Actual {}", node3, 40f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node4).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node4, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node4, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node4, 0f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node4, 50f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node5).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node5, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node5, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node5, 0f32, location.x);
    assert_eq!(location.y, 55f32, "y of node {:?}. Expected {}. Actual {}", node5, 55f32, location.y);
    let Layout { size, location, .. } = taffy.layout(node6).unwrap();
    assert_eq!(size.width, 50f32, "width of node {:?}. Expected {}. Actual {}", node6, 50f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node6, 10f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node6, 0f32, location.x);
    assert_eq!(location.y, 70f32, "y of node {:?}. Expected {}. Actual {}", node6, 70f32, location.y);
}
