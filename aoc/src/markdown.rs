use std::ops::Deref as _;
use scraper::node;

/// Convert an HTML element recursively into Markdown.
pub fn from_html(element: scraper::ElementRef) -> String {
    fn _parse<'tree>(
        element: ego_tree::NodeRef<'tree, node::Node>,
        mut indent: usize,
        buffer: &mut String,
    ) {
        match scraper::ElementRef::wrap(element) {
        | Some(element) if element.value().name() == "code" => buffer.push('`'),
        | Some(element) if element.value().name() == "em" => buffer.push_str("***"),
        | Some(element) if element.value().name() == "h2" => buffer.push_str("## "),
        | Some(element) if element.value().name() == "li" => {
            for _ in 0..indent {
                buffer.push(' ');
            }
            indent += 2;
            buffer.push_str("- ");
        }
        | Some(element) => { dbg!(element.value().name()); },
        | None => {
            if let node::Node::Text(text) = element.value() {
                buffer.push_str(&text.text);
            }
        }
        }

        element
            .children()
            .for_each(|child| _parse(child, indent, buffer));

        match scraper::ElementRef::wrap(element) {
        | Some(element) if element.value().name() == "h2" => buffer.push('\n'),
        | Some(element) if element.value().name() == "code" => buffer.push('`'),
        | Some(element) if element.value().name() == "em" => buffer.push_str("***"),
        | _ => (),
        }
    }

    let mut buffer = String::new();
    _parse(*element.deref(), 0, &mut buffer);
    buffer
}
