use std::iter;

use aoc::*;

#[derive(Clone, Debug)]
pub struct ScienceForHungryPeople(Vec<Ingredient>);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Fro for ScienceForHungryPeople {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .filter_map(|line| {
                let (name, ingredients) = line.split_once(": ")?;

                let mut iter = ingredients
                    .split(", ")
                    .filter_map(|ingredient| ingredient.split_whitespace().nth(1))
                    .map(i64::fro);

                Some(Ingredient {
                    name: String::from(name),
                    capacity: iter.give(),
                    durability: iter.give(),
                    flavor: iter.give(),
                    texture: iter.give(),
                    calories: iter.give(),
                })
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

fn recurse<'a>(
    ingredients: &'a [Ingredient],
    total: i64,
) -> Box<dyn Iterator<Item = [i64; 5]> + 'a> {
    match ingredients {
        [] => unreachable!(),
        [ingredient] => Box::new(iter::once([
            ingredient.capacity * total,
            ingredient.durability * total,
            ingredient.flavor * total,
            ingredient.texture * total,
            ingredient.calories * total,
        ])),
        [ingredient, ingredients @ ..] => Box::new((0..=total).flat_map(move |count| {
            recurse(ingredients, total - count).map(
                move |[capacity, durability, flavor, texture, calories]| {
                    [
                        ingredient.capacity * count + capacity,
                        ingredient.durability * count + durability,
                        ingredient.flavor * count + flavor,
                        ingredient.texture * count + texture,
                        ingredient.calories * count + calories,
                    ]
                },
            )
        })),
    }
}

impl Solution for ScienceForHungryPeople {
    fn one(self) -> i64 {
        recurse(&self.0, 100)
            .filter(|scores| scores.iter().all(|score| *score > 0))
            .map(|scores| scores.iter().rev().skip(1).product())
            .max()
            .unwrap()
    }

    fn two(self) -> i64 {
        recurse(&self.0, 100)
            .filter(|scores| scores[4] == 500)
            .filter(|scores| scores.iter().all(|score| *score > 0))
            .map(|scores| scores.iter().rev().skip(1).product())
            .max()
            .unwrap()
    }
}
