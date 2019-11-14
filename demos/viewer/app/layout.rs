/// Tools for parsing and hacking layouts created by the Stretch flexbox crate
///
///
///
use tweek::{
    gui::*,
    tools::*,
};

use quicksilver::{
    geom::{Rectangle, Vector},
};

#[allow(unused_imports)]
use stretch::{
    geometry::*,
    node::{Node, Stretch},
    number::Number,
    result::Layout,
    style::*
};

/// A wrapper for containing the resulting layout including all children
#[derive(Debug, Clone)]
pub struct NodeLayout {
    pub id: Node,
    pub size: Size<f32>,
    pub location: Point<f32>,
    pub children: Vec<NodeLayout>,
}

impl NodeLayout {}

// #[derive(Clone, Debug, PartialEq)]
// pub struct LayoutRule {
//     pub scale_x: f32,
//     pub scale_y: f32,
//     pub margins: Margins
// }

// impl LayoutRule {
//     pub fn new(scale_x: f32, scale_y: f32) -> Self {
//         LayoutRule {
//             scale_x,
//             scale_y,
//             margins: Margins::default(),
//         }
//     }
// }

pub struct Margins {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Margins {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Margins { left, top, right, bottom }
    }
}

impl Default for Margins {
    fn default() -> Self {
        Margins {
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
        }
    }
}

pub struct LayoutBuilder {
    pub root: Node,
    stretch: Stretch,
}

impl LayoutBuilder {
    pub fn new() -> Self {
        let mut stretch = Stretch::new();
        let root = stretch.new_node(
            Style::default(),
            vec![]
        ).unwrap();
        LayoutBuilder { root, stretch }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.stretch.set_style(self.root, style);
        self
    }

    // pub fn root_node(&self)
    pub fn add_row(&mut self, parent: Node, height: f32, custom_style: Option<Style>) -> Node {
        let style = {
            if let Some(style) = custom_style {
                style
            } else {
                Style {
                    size: Size { width: Dimension::Auto, height: Dimension::Points(height) },
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                }
            }
        };
        let node = self.stretch.new_node(style, vec![]).unwrap();
        self.stretch.add_child(parent, node);
        node
    }

    pub fn add_column(&mut self, parent: Node, width: f32, custom_style: Option<Style>) -> Node {
        let style = {
            if let Some(style) = custom_style {
                style
            } else {
                Style {
                    size: Size { width: Dimension::Points(width), height: Dimension::Auto },
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                }
            }
        };
        let node = self.stretch.new_node(style, vec![]).unwrap();
        self.stretch.add_child(parent, node);
        node
    }


    pub fn add_object(&mut self, parent: Node, size: Size<f32>) -> Node {
        let node_size = Size { width: Dimension::Points(size.width), height: Dimension::Points(size.height) };
        let object = self.stretch.new_leaf(
            Style {
                size: node_size,
                ..Default::default()
            },
            Box::new(move |_| Ok(size)),
        ).unwrap();
        self.stretch.add_child(parent, object);
        object
    }

    pub fn get_layout(&self, node: Node) -> &Layout {
        let layout = self.stretch.layout(node).unwrap();
        layout
    }

    /// A helper method to convert a Layout constructed by Stretch to absolute xy coords.
    /// Additional features may be added in the future
    pub fn absolute_layout(&mut self, node: Node, origin: (f32, f32)) -> NodeLayout {
        // if let Ok(layout) =
        self.stretch.compute_layout(node, Size::undefined());

        let layout = self.stretch.layout(node).unwrap();
        let mut location = layout.location.clone();
        location.x += origin.0;
        location.y += origin.1;
        let mut result = NodeLayout {
            id: node.clone(),
            size: layout.size.clone(),
            location: location,
            children: Vec::new(),
        };
        self.abs_copy_layout(node, &mut result);
        result
    }

    /// A recursive function for performing a deep copy of a Stretch Layout and changing coordinates from
    /// relative to absolute.
    fn abs_copy_layout(&mut self, node: Node, result: &mut NodeLayout) {
        let children = self.stretch.children(node).unwrap();
        for (i, child) in children.iter().enumerate() {
            let layout = self.stretch.layout(*child).unwrap();
            let pos = Point { x: result.location.x + layout.location.x,
                            y: result.location.y + layout.location.y};
            let mut item = NodeLayout {
                id: child.clone(),
                size: layout.size.clone(),
                location: pos,
                children: Vec::new(),
            };
            if self.stretch.child_count(*child).unwrap() > 0 {
                self.abs_copy_layout(*child, &mut item);
            }
            result.children.push(item);
        }
    }
}

pub struct LayoutHelper {}

impl LayoutHelper {

    /// Center the inner frame in the outer frame with the specified offset.
    /// Offset can be Vector::ZERO for no offset.
    pub fn center_frame(outer: &Rectangle, inner: &Rectangle, offset: Vector) -> Rectangle {
        let mut result = inner.clone();
        result.pos.x = (outer.width() - inner.width()) / 2.0 + offset.x;
        result.pos.y = (outer.height() - inner.height()) / 2.0 + offset.y;
        result
    }

    // pub fn position_frame(frame: &Rectangle, spec: &AdSpec) -> Rectangle {

    // }

    // pub fn align_frame()
    // pub fn frame_for_text(text: &str, size: f32, font: &mut DrawFont) -> Rectangle {

    // }
}