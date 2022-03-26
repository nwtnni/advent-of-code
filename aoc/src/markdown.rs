use std::borrow::Cow;
use std::ops::Deref as _;

use scraper::node::Node as Html;

use crate::api;

/// Convert an HTML element recursively into Markdown.
pub fn from_html(html: scraper::ElementRef, year: aoc_core::Year) -> String {
    fn recurse<'html>(
        html: ego_tree::NodeRef<'html, Html>,
        year: aoc_core::Year,
        notes: &mut Vec<&'html str>,
        links: &mut Vec<Cow<'html, str>>,
    ) -> String {
        macro_rules! recurse {
            () => {
                html.children()
                    .map(|child| recurse(child, year, notes, links))
                    .collect::<String>()
            };
        }

        match html.value() {
            Html::Comment(_)
            | Html::Doctype(_)
            | Html::Document
            | Html::Fragment
            | Html::ProcessingInstruction(_) => String::new(),
            Html::Element(element) if element.name() == "a" => {
                let href = match element.attr("href") {
                    Some(href) => href,
                    None => return recurse!(),
                };

                let index = links.len();

                // Handle relative links
                links.push(if href.starts_with("http") {
                    Cow::Borrowed(href)
                } else if href.starts_with('/') {
                    Cow::Owned(format!("{}{}", api::ROOT, href))
                } else {
                    Cow::Owned(format!("{}/{}/day/{}", api::ROOT, year, href))
                });

                format!("[{}][{}]", recurse!(), index)
            }
            Html::Element(element) if element.name() == "article" => recurse!(),
            Html::Element(element) if element.name() == "br" => String::from("\n"),
            Html::Element(element) if element.name() == "code" => format!("`{}`", recurse!()),
            Html::Element(element) if element.name() == "em" => format!("**{}**", recurse!()),
            Html::Element(element) if element.name() == "h2" => format!("## {}\n\n", recurse!()),
            Html::Element(element) if element.name() == "li" => {
                let item = recurse!();

                if !item.contains('\n') {
                    return format!("- {}\n", item);
                }

                let mut buffer = String::new();
                let mut lines = item.split('\n');

                if let Some(line) = lines.next() {
                    buffer.push('-');
                    if !line.trim().is_empty() {
                        buffer.push(' ');
                        buffer.push_str(line);
                    }
                    buffer.push('\n');
                }

                for line in lines {
                    if !line.trim().is_empty() {
                        buffer.push_str("  ");
                        buffer.push_str(line);
                    }
                    buffer.push('\n');
                }

                // Remove trailing newline
                if buffer.ends_with("\n\n") {
                    buffer.truncate(buffer.len() - 1);
                }

                buffer
            }
            Html::Element(element) if element.name() == "p" => format!("{}\n\n", recurse!()),
            Html::Element(element) if element.name() == "pre" => {
                let block = recurse!();
                let block = block.trim_start_matches('`').trim_end_matches('`');

                format!(
                    "```\n{}{}```\n\n",
                    &block,
                    if block.ends_with('\n') { "" } else { "\n" },
                )
            }
            Html::Element(element) if element.name() == "span" => {
                let title = match element.attr("title") {
                    Some(title) => title,
                    None => return recurse!(),
                };

                let index = notes.len();

                notes.push(title);

                format!("[{}][^{}]", recurse!(), index)
            }
            Html::Element(element) if element.name() == "ul" => format!("{}\n", recurse!()),
            Html::Element(element) => {
                eprintln!("[WARNING]: Unexpected element: {:#?}", element);
                recurse!()
            }
            Html::Text(text) if text.text.as_ref() == "\n" => String::new(),
            Html::Text(text) => text.text.replace(".  ", ". ").replace(":  ", ": "),
        }
    }

    let mut notes = Vec::new();
    let mut links = Vec::new();
    let mut markdown = recurse(*html.deref(), year, &mut notes, &mut links);

    for (index, note) in notes.iter().enumerate() {
        markdown.push_str(&format!("[^{}]: {}\n", index, note));
    }

    if !notes.is_empty() && !links.is_empty() {
        markdown.push('\n');
    }

    for (index, link) in links.iter().enumerate() {
        markdown.push_str(&format!("[{}]: {}\n", index, link));
    }

    markdown
}
