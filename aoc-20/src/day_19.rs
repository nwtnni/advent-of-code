use std::collections::HashMap;

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
        let mut iter = input.trim().split("\n\n");

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
                        .map(|leaf| {
                            leaf.trim()
                                .split_whitespace()
                                .map(i64::fro)
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>()
                        .tap(Rule::Tree)
                };

                (index, rule)
            })
            .collect::<HashMap<_, _>>();

        let messages = iter.give().trim().split('\n').map(String::from).collect();

        Self { rules, messages }
    }
}

impl MonsterMessages {
    fn matches(&self, message: &str, rule: i64, memo: &mut HashMap<i64, usize>) -> bool {
        match &self.rules[&rule] {
            Rule::Leaf(leaf) => message.chars().all(|char| char == *leaf),
            Rule::Tree(tree) => {
                for subtree in tree {
                    let mut index = 0;
                    let mut all = true;

                    for rule in subtree {
                        let len = match memo.get(rule).copied() {
                            None => self.len(*rule, memo),
                            Some(len) => len,
                        };
                        all &= self.matches(&message[index..index + len], *rule, memo);
                        index += len;
                    }

                    if all && index == message.len() {
                        return true;
                    }
                }
                false
            }
        }
    }

    fn len(&self, rule: i64, memo: &mut HashMap<i64, usize>) -> usize {
        if let Some(len) = memo.get(&rule).copied() {
            return len;
        };

        let len = match &self.rules[&rule] {
            Rule::Leaf(_) => 1,
            Rule::Tree(tree) => tree[0].iter().map(|rule| self.len(*rule, memo)).sum(),
        };

        memo.insert(rule, len);
        len
    }
}

impl Solution for MonsterMessages {
    fn one(self) -> i64 {
        let mut memo = HashMap::new();
        self.messages
            .iter()
            .filter(|message| self.matches(message, 0, &mut memo))
            .count() as i64
    }

    fn two(self) -> i64 {
        let mut memo = HashMap::new();
        let len_31 = self.len(31, &mut memo);
        let len_42 = self.len(42, &mut memo);

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
                let mut index = 0;
                let mut _31s = 0;
                let mut _42s = 0;

                while self.matches(&message[index..index + len_42], 42, &mut memo) {
                    _42s += 1;
                    index += len_42;
                    if index >= message.len() {
                        return false;
                    }
                }

                while self.matches(&message[index..index + len_31], 31, &mut memo) {
                    _31s += 1;
                    index += len_31;
                    if index >= message.len() {
                        break;
                    }
                }

                index == message.len() && _42s > _31s && _31s > 0
            })
            .count() as i64
    }
}
