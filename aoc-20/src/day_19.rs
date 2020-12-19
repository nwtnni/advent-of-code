use std::collections::HashMap;
use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct MonsterMessages {
    rules: HashMap<i64, Rule>,
    messages: Vec<String>,
}

#[derive(Clone, Debug)]
enum Rule {
    Leaf(char),
    Tree(Vec<Vec<i64>>),
}

impl Fro for MonsterMessages {
    fn fro(input: &str) -> Self {
        let mut iter = input
            .trim()
            .split("\n\n");

        let rules = iter
            .give()
            .trim()
            .split('\n')
            .map(|line| {
                let mut iter = line.trim().split(':');

                let index = iter.give().to::<i64>();

                let rule = iter.give().trim();

                let rule = if rule.starts_with('"') {
                    Rule::Leaf(rule.chars().nth(1).unwrap())
                } else {
                    rule.split('|')
                        .map(|leaf| leaf.trim().split_whitespace().map(i64::fro).collect::<Vec<_>>())
                        .collect::<Vec<_>>()
                        .tap(Rule::Tree)
                };

                (index, rule)
            })
            .collect::<HashMap<_, _>>();

        let messages = iter
            .give()
            .trim()
            .split('\n')
            .map(String::from)
            .collect();

        Self {
            rules,
            messages,
        }
    }
}

impl MonsterMessages {
    fn matches(&self, message: &str, rule: i64) -> bool {
        match &self.rules[&rule] {
            Rule::Tree(tree) => {
                for subtree in tree {
                    let mut len = 0;
                    let mut all = true;

                    for rule in subtree {
                        let len_ = self.len(*rule);
                        all &= self.matches(&message[len..len + len_], *rule);
                        len += len_;
                    }

                    if all && len == message.len() {
                        return true;
                    }
                }
                false
            }
            Rule::Leaf(leaf) => {
                message.chars().all(|char| char == *leaf)
            }
        }
    }

    fn len(&self, rule: i64) -> usize {
        match &self.rules[&rule] {
            Rule::Leaf(_) => 1,
            Rule::Tree(tree) => tree[0].iter().map(|rule| self.len(*rule)).sum(),
        }
    }

    fn generate(&self, rule: i64) -> Vec<String> {
        match &self.rules[&rule] {
            Rule::Leaf(leaf) => vec![String::from(*leaf)],
            Rule::Tree(tree) => {
                tree.iter()
                    .flat_map(|subtree| {
                        let mut buffer = Vec::new();
                        for rule in subtree {
                            buffer.push(self.generate(*rule));
                        }
                        Self::product(&buffer)
                    })
                    .collect()
            }
        }
    }

    fn product(of: &[Vec<String>]) -> Vec<String> {
        if of.len() == 1 {
            return of[0].clone();
        }

        let mut strings = Vec::new();
        for prefix in &of[0] {
            for suffix in Self::product(&of[1..]) {
                strings.push(format!("{}{}", prefix, suffix));
            }
        }
        strings
    }
}

impl Solution for MonsterMessages {
    fn one(self) -> i64 {
        self.messages
            .iter()
            .filter(|message| self.matches(message, 0))
            .count() as i64
    }

    fn two(self) -> i64 {
        let _42 = self
            .generate(42)
            .into_iter()
            .collect::<HashSet<_>>();

        let _42_len = _42.iter().give().len();

        let _31 = self
            .generate(31)
            .into_iter()
            .collect::<HashSet<_>>();

        let _31_len = _31.iter().give().len();

        self.messages
            .iter()
            // Postmortem:
            //
            // In my first attempt at part two, I was processing messages from
            // right to left because I thought it would otherwise be ambiguous
            // where rule 8 ended and rule 11 began (which turned out to be
            // irrelevant to the problem). Turns out there's some overlap between
            // the sets of strings in rule 31 and 42 (but only in the actual inputs,
            // not the example), which caused a middle block to flip between rule 31
            // and 42 depending on which direction I was reading... causing my
            // solution to be one larger :'(
            //
            // Took me about an hour and @justinxu421's help to figure it out.
            //
            // Example: 42 42 42/31 would be rejected LR and accepted RL.
            .filter(|message| {
                let mut start = 0;
                let mut _31s = 0;
                let mut _42s = 0;

                while _42.contains(&message[start..start + _42_len]) {
                    _42s += 1;
                    start += _42_len;
                    if start >= message.len() {
                        return false;
                    }
                }

                while _31.contains(&message[start..start + _31_len]) {
                    _31s += 1;
                    start += _31_len;
                    if start >= message.len() {
                        break;
                    }
                }

                start == message.len() && _42s > _31s && _31s > 0
            })
            .count() as i64
    }
}
