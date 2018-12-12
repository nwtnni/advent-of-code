use std::str::FromStr;

const INPUT: &'static str = include_str!("input.txt");

#[derive(Clone, Debug)]
pub struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn sum_metadata(&self) -> usize {
        self.children.iter()
            .map(|child| child.sum_metadata())
            .chain(self.metadata.iter().cloned())
            .sum()
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.sum_metadata()
        } else {
            self.metadata.iter()
                .filter_map(|id| self.children.get(*id - 1))
                .map(|child| child.value())
                .sum()
        }
    }
}

fn parse_usize<I>(stream: &mut I) -> usize
    where I: Iterator<Item = &'static str>
{
    stream.next()
        .map(FromStr::from_str)
        .map(Result::unwrap)
        .unwrap()
}

fn parse_node<I>(stream: &mut I) -> Node
    where I: Iterator<Item = &'static str>
{
    let children_count = parse_usize(stream);
    let metadata_count = parse_usize(stream);

    let children = (0..children_count)
        .map(|_| parse_node(stream))
        .collect();

    let metadata = (0..metadata_count)
        .map(|_| parse_usize(stream))
        .collect();
    
    Node { children, metadata }
}

fn main() {
    let root = parse_node(&mut INPUT.trim().split_whitespace());
    let sum = root.sum_metadata();    
    println!("{}", sum);
    let value = root.value();
    println!("{}", value);
}
