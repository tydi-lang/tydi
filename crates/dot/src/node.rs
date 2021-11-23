use std::fmt::Display;

use crate::style::NodeStyle;

/// Trait to return a DOT string representation of something.
///
/// Rather than using Display or ToString, this is explicitly created because those
/// traits could be implemented such that the produced character sequence is not
/// compatible with DOT syntax.
pub(crate) trait DotDisplay {
    fn to_dot(&self) -> String;
}

impl DotDisplay for String {
    fn to_dot(&self) -> String {
        // TODO sanitize
        self.to_string()
    }
}

impl DotDisplay for &str {
    fn to_dot(&self) -> String {
        // TODO sanitize
        self.to_string()
    }
}

/// Trait to return a DOT node identifier for DOT nodes.
pub(crate) trait DotNodeId {
    fn to_dot_node_id(&self) -> String;
}

impl DotNodeId for &str {
    fn to_dot_node_id(&self) -> String {
        self.to_string()
    }
}

/// A representation of a DOT node
pub(crate) struct DotNode<'a> {
    /// DOT node id (not to be confused with Salsa intern id)
    id: String,
    /// Style of the node.
    style: &'a NodeStyle<'a>,
    /// Label of the node.
    label: Option<String>,
}

impl<'a> DotNode<'a> {
    pub(crate) fn new(style: &'a NodeStyle<'a>, dot_id: impl DotNodeId) -> Self {
        Self {
            id: dot_id.to_dot_node_id(),
            style,
            label: None,
        }
    }

    pub(crate) fn with_label(mut self, label: impl DotDisplay) -> Self {
        self.label.replace(label.to_dot());
        self
    }
}

impl<'a> Display for DotNode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !(self.style.title.is_some() || self.label.is_some()) {
            // If there is not much to plot, don't create an HTML table label.
            writeln!(f, "{} [{}]", self.id, self.style.style)
        } else {
            writeln!(
                f,
                r#"{} [label=<<FONT FACE="Montserrat" POINT-SIZE="8"><TABLE BORDER="0" CELLBORDER="0" CELLSPACING="0" CELLPADDING="1">{}{}</TABLE></FONT>> {}]"#,
                self.id,
                self.style
                    .title
                    .map(|t| format!(
                        r#"<TR><TD BORDER="0"><FONT COLOR="{}"><B>{}</B></FONT></TD></TR>")"#,
                        self.style.title_color, t
                    ))
                    .unwrap_or_default(),
                self.label
                    .as_ref()
                    .map(|l| format!(
                        r#"<TR><TD BORDER="0"><FONT POINT-SIZE="10">{}</FONT></TD></TR>"#,
                        l
                    ))
                    .unwrap_or_default(),
                self.style.style
            )
        }
    }
}

/// Draw a DOT edge in a DOT source.
pub(crate) fn draw_edge(
    f: &mut std::fmt::Formatter,
    src: impl DotNodeId,
    dst: impl DotNodeId,
    style: impl Display,
    label: Option<String>,
) -> std::fmt::Result {
    writeln!(
        f,
        "{} -> {} [{}{}]",
        src.to_dot_node_id(),
        dst.to_dot_node_id(),
        style,
        if let Some(label) = label {
            format!(
                " taillabel=<<FONT POINT-SIZE=\"8\" FACE=\"Montserrat\">{}</FONT>>",
                label
            )
        } else {
            String::default()
        }
    )
}
