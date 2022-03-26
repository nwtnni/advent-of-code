use std::collections::HashMap;
use std::io;

use aoc::*;

#[derive(Clone, Debug)]
pub struct SpaceStoichiometry(HashMap<&'static str, Recipe>);

#[derive(Clone, Debug)]
struct Recipe {
    to: i64,
    from: Vec<(&'static str, i64)>,
}

impl Fro for SpaceStoichiometry {
    fn fro(input: &str) -> Self {
        let mut recipes = HashMap::new();
        for line in input.trim().split('\n') {
            let mut iter = line.split("=>");

            let inputs = iter.give().trim().split(',');

            let mut output = iter.give().trim().split_whitespace();

            let oc = output.give().to::<i64>();
            let om = output.give().leak();

            let mut recipe = Recipe {
                to: oc,
                from: Vec::new(),
            };

            for input in inputs {
                let mut iter = input.trim().split_whitespace();
                let ic = iter.give().to::<i64>();
                let im = iter.give().leak();
                recipe.from.push((im, ic));
            }

            recipes.insert(om, recipe);
        }
        SpaceStoichiometry(recipes)
    }
}

impl SpaceStoichiometry {
    fn ore_required(&self, fuel: i64) -> i64 {
        let mut pool = HashMap::new();
        pool.insert("FUEL", fuel);
        loop {
            let (here, here_count) = match pool
                .iter()
                .filter(|(_, count)| **count > 0)
                .find(|(chemical, _)| **chemical != "ORE")
            {
                Some((here, here_count)) => (*here, *here_count),
                None => return pool["ORE"],
            };

            let recipe = &self.0[here];
            let multiple = (here_count + recipe.to - 1) / recipe.to;

            for (next, next_count) in &recipe.from {
                let required = multiple * next_count;
                pool.entry(next)
                    .and_modify(|count| *count += required)
                    .or_insert(required);
            }

            let required = multiple * recipe.to;
            pool.entry(here)
                .and_modify(|count| *count -= required)
                .or_insert(-required);

            pool.retain(|_, count| *count != 0);
        }
    }
}

impl Solution for SpaceStoichiometry {
    fn one(self) -> i64 {
        self.ore_required(1)
    }

    fn two(self) -> i64 {
        let mut buffer = String::new();
        loop {
            io::stdin().read_line(&mut buffer).ok();
            println!("{}", self.ore_required(buffer.trim().to::<i64>()));
            buffer.clear();
        }
    }
}
