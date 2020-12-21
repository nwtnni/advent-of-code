use std::collections::BTreeSet;

use indexmap::IndexMap;
use indexmap::IndexSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct AllergenAssessment(Vec<Food>);

#[derive(Clone, Debug)]
struct Food {
    ingredients: Vec<&'static str>,
    allergens: Vec<&'static str>,
}

impl Fro for AllergenAssessment {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let mut iter = line.split(" (");

                let ingredients = iter
                    .give()
                    .trim()
                    .split_whitespace()
                    .map(Leak::leak)
                    .collect::<Vec<_>>();

                let allergens = iter
                    .give()
                    .trim()
                    .trim_start_matches("contains ")
                    .trim_end_matches(")")
                    .split(", ")
                    .map(Leak::leak)
                    .collect::<Vec<_>>();

                Food {
                    ingredients,
                    allergens,
                }
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for AllergenAssessment {
    fn one(self) -> i64 {
        // Map from ingredient to the set of rules it appears in
        let mut ingredients: IndexMap<&'static str, IndexSet<usize>> = IndexMap::default();

        for (ingredient, index) in self
            .0
            .iter()
            .enumerate()
            .flat_map(|(index, food)| {
                food.ingredients
                    .iter()
                    .copied()
                    .map(move |ingredient| (ingredient, index))
            })
        {
            ingredients
                .entry(ingredient)
                .or_insert_with(IndexSet::default)
                .insert(index);
        }

        // Map from allergen to the set of rules it appears in
        let mut allergens: IndexMap<&'static str, IndexSet<usize>> = IndexMap::default();

        for (allergen, index) in self
            .0
            .iter()
            .enumerate()
            .flat_map(|(index, food)| {
                food.allergens
                    .iter()
                    .copied()
                    .map(move |allergen| (allergen, index))
            })
        {
            allergens
                .entry(allergen)
                .or_insert_with(IndexSet::default)
                .insert(index);
        }

        let impossible = ingredients
            .iter()
            .filter(|(&ingredient, indices)| {
                indices
                    .iter()
                    .all(|index| {
                        self.0[*index]
                            .allergens
                            .iter()
                            .all(|allergen| {
                                allergens[allergen].iter().any(|index| {
                                    !self.0[*index].ingredients.contains(&ingredient)
                                })
                            })
                    })
            })
            .map(|(ingredient, _)| ingredient)
            .copied()
            .collect::<IndexSet<_>>();

        self.0
            .iter()
            .map(|food| {
                food.ingredients
                    .iter()
                    .copied()
                    .filter(|ingredient| impossible.contains(ingredient))
                    .count()
            })
            .sum::<usize>()
            as i64
    }

    fn two(self) -> i64 {
        // Map from ingredient to the set of rules it appears in
        let mut ingredients: IndexMap<&'static str, IndexSet<usize>> = IndexMap::default();

        for (ingredient, index) in self
            .0
            .iter()
            .enumerate()
            .flat_map(|(index, food)| {
                food.ingredients
                    .iter()
                    .copied()
                    .map(move |ingredient| (ingredient, index))
            })
        {
            ingredients
                .entry(ingredient)
                .or_insert_with(IndexSet::default)
                .insert(index);
        }

        // Map from allergen to the set of rules it appears in
        let mut allergens: IndexMap<&'static str, IndexSet<usize>> = IndexMap::default();

        for (allergen, index) in self
            .0
            .iter()
            .enumerate()
            .flat_map(|(index, food)| {
                food.allergens
                    .iter()
                    .copied()
                    .map(move |allergen| (allergen, index))
            })
        {
            allergens
                .entry(allergen)
                .or_insert_with(IndexSet::default)
                .insert(index);
        }

        let impossible = ingredients
            .iter()
            .filter(|(&ingredient, indices)| {
                indices
                    .iter()
                    .all(|index| {
                        self.0[*index]
                            .allergens
                            .iter()
                            .all(|allergen| {
                                allergens[allergen].iter().any(|index| {
                                    !self.0[*index].ingredients.contains(&ingredient)
                                })
                            })
                    })
            })
            .map(|(ingredient, _)| ingredient)
            .copied()
            .collect::<IndexSet<_>>();

        ingredients.retain(|ingredient, _| !impossible.contains(ingredient));

        let mut solution: IndexMap<&'static str, &'static str> = IndexMap::default();
        let mut constraints: IndexMap<BTreeSet<&'static str>, IndexSet<&'static str>> = IndexMap::default();

        for food in &self.0 {
            let ingredients = food
                .ingredients
                .iter()
                .copied()
                .filter(|ingredient| !impossible.contains(ingredient))
                .collect::<BTreeSet<_>>();

            let allergens = food
                .allergens
                .iter()
                .copied()
                .collect::<IndexSet<_>>();

            constraints
                .entry(ingredients)
                .or_insert_with(IndexSet::default)
                .extend(allergens);
        }

        let mut constraints = constraints.into_iter().collect::<Vec<_>>();
        let total = ingredients.len();

        'outer: while solution.len() < total {
            for i in 0..constraints.len() {
                let (constraint_ingredients, constraint_allergens) = &constraints[i];

                let (ingredient, allergen, remove) = if constraint_ingredients.len() == 1
                    && constraint_allergens.len() == 1
                {
                    (
                        *constraint_ingredients.iter().give(),
                        constraint_allergens[0],
                        true,
                    )
                } else if constraint_ingredients.len() == ingredients.len() - 1
                    && constraint_allergens.len() == allergens.len() - 1
                {
                    (
                        ingredients
                            .iter()
                            .find(|(ingredient, _)| !constraint_ingredients.contains(*ingredient))
                            .map(|(ingredient, _)| *ingredient)
                            .unwrap(),
                        allergens
                            .iter()
                            .find(|(allergen, _)| !constraint_allergens.contains(*allergen))
                            .map(|(allergen, _)| *allergen)
                            .unwrap(),
                        false,
                    )
                } else {
                    continue;
                };

                solution.insert(ingredient, allergen);
                ingredients.swap_remove(ingredient);
                allergens.swap_remove(allergen);
                for (constraint_ingredients, constraint_allergens) in &mut constraints {
                    constraint_ingredients.remove(ingredient);
                    constraint_allergens.remove(allergen);
                }
                if remove {
                    constraints.swap_remove(i);
                }
                continue 'outer;
            }
        }

        let mut iter = solution
            .into_iter()
            .map(|(ingredient, allergen)| (allergen, ingredient))
            .collect::<Vec<_>>()
            .tap_mut(|collected| collected.sort())
            .into_iter()
            .map(|(_, ingredient)| ingredient);

        if let Some(ingredient) = iter.next() {
            print!("{}", ingredient);
        }

        for ingredient in iter {
            print!(",{}", ingredient);
        }

        println!();

        // Puzzle solution is a string
        unreachable!()
    }
}
