//! Utilities to help styling DOT Graphs of the high-level intermediate
//! representation.

/// Some basic colors
pub mod colors {
    /// Black color
    pub const BLACK: &str = "#000000";
    /// Red color
    pub const RED: &str = "#B63E4A";
    /// Yellow color
    pub const YELLOW: &str = "#CA9121";
    /// Green color
    pub const GREEN: &str = "#63B679";
    /// Blue color
    pub const BLUE: &str = "#3892C2";
    /// Gray color
    pub const GRAY: &str = "#808080";
}

/// Style of edges.
pub struct EdgeStyles<'a> {
    /// Style of an edge from some parent to a child node.
    pub child: &'a str,
    /// Style of a reference from some node to another node.
    pub reference: &'a str,
}

/// Style of a node.
pub struct NodeStyle<'a> {
    /// Node title.
    pub title: Option<&'a str>,
    /// Node title color.
    pub title_color: &'a str,
    /// Miscelleaneous node style.
    pub style: &'a str,
    /// Wether to display node metadata.
    pub show_meta: bool,
}

impl<'a> Default for NodeStyle<'a> {
    fn default() -> Self {
        Self {
            title: None,
            title_color: colors::BLACK,
            style: "shape=box style=rounded",
            show_meta: false,
        }
    }
}

/// Collection of styles for the HIR nodes.
pub struct NodeStyles<'a> {
    /// Root node style.
    pub root: NodeStyle<'a>,
    /// Module node style.
    pub module: NodeStyle<'a>,
    /// Component node style.
    pub component: NodeStyle<'a>,
    /// Port node style.
    pub port: NodeStyle<'a>,
    /// Connection node style.
    pub connection: NodeStyle<'a>,
    /// Net::Port node style.
    pub net_port: NodeStyle<'a>,
    /// Net::Wire node style.
    pub wire: NodeStyle<'a>,
    /// Net::InstancePort node style.
    pub instance_port: NodeStyle<'a>,
    /// Instance node style.
    pub instance: NodeStyle<'a>,
    /// Type node style.
    pub typ: NodeStyle<'a>,
    /// LogicalType node style.
    pub logical_type: NodeStyle<'a>,
    /// Field node style.
    pub field: NodeStyle<'a>,
}

/// HIR Graphviz DOT representation style.
#[derive(Default)]
pub struct Style<'a> {
    /// Styles of the various edge types.
    pub edges: EdgeStyles<'a>,
    /// Styles of the various node types.
    pub nodes: NodeStyles<'a>,
}

impl<'a> Default for EdgeStyles<'a> {
    fn default() -> Self {
        Self {
            child: "",
            reference: "style=\"dashed\" color=\"#97c3da\"",
        }
    }
}

impl<'a> Default for NodeStyles<'a> {
    fn default() -> Self {
        Self {
            root: NodeStyle {
                style: "shape=point",
                ..Default::default()
            },
            module: NodeStyle {
                title: Some("Module"),
                title_color: colors::RED,
                ..Default::default()
            },
            component: NodeStyle {
                title: Some("Component"),
                title_color: colors::GREEN,
                ..Default::default()
            },
            port: NodeStyle {
                title: Some("Port"),
                title_color: colors::YELLOW,
                ..Default::default()
            },
            connection: NodeStyle {
                title: Some("Connection"),
                title_color: colors::YELLOW,
                ..Default::default()
            },
            net_port: NodeStyle {
                title: Some("Net::Port"),
                title_color: colors::YELLOW,
                ..Default::default()
            },
            wire: NodeStyle {
                title: Some("Net::Wire"),
                title_color: colors::YELLOW,
                ..Default::default()
            },
            instance_port: NodeStyle {
                title: Some("Net::InstancePort"),
                title_color: colors::YELLOW,
                ..Default::default()
            },
            instance: NodeStyle {
                title: Some("Instance"),
                title_color: colors::BLUE,
                ..Default::default()
            },
            typ: NodeStyle {
                title: Some("Type"),
                title_color: colors::GRAY,
                ..Default::default()
            },
            logical_type: NodeStyle {
                title: Some("LogicalType"),
                title_color: colors::GREEN,
                ..Default::default()
            },
            field: NodeStyle {
                title: Some("Field"),
                title_color: colors::GRAY,
                ..Default::default()
            },
        }
    }
}
