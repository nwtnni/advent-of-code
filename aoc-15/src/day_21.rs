use std::cmp;
use std::iter;

use aoc::*;

#[derive(Clone, Debug)]
pub struct RPGSimulator20XX {
    health: i64,
    damage: i64,
    armor: i64,
}

impl Fro for RPGSimulator20XX {
    fn fro(input: &str) -> Self {
        let mut iter = input.trim().split('\n');
        Self {
            health: iter.give().trim_start_matches("Hit Points: ").tap(i64::fro),
            damage: iter.give().trim_start_matches("Damage: ").tap(i64::fro),
            armor: iter.give().trim_start_matches("Armor: ").tap(i64::fro),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Player {
    weapon: Item,
    armor: Option<Item>,
    rings: [Option<Item>; 2],
}

#[derive(Copy, Clone, Debug)]
struct Item {
    #[allow(unused)]
    name: &'static str,
    cost: i64,
    damage: i64,
    armor: i64,
}

impl Item {
    const fn new(name: &'static str, cost: i64, damage: i64, armor: i64) -> Self {
        Item {
            name,
            cost,
            damage,
            armor,
        }
    }
}

// :'<,'>s/\([a-zA-Z]\+\)\s*\(\d\+\)\s*\(\d\+\)\s*\(\d\)/    Item::new("\1\", \2\, \3, \4),
const WEAPONS: [Item; 5] = [
    Item::new("Dagger", 8, 4, 0),
    Item::new("Shortsword", 10, 5, 0),
    Item::new("Warhammer", 25, 6, 0),
    Item::new("Longsword", 40, 7, 0),
    Item::new("Greataxe", 74, 8, 0),
];

const ARMORS: [Item; 5] = [
    Item::new("Leather", 13, 0, 1),
    Item::new("Chainmail", 31, 0, 2),
    Item::new("Splintmail", 53, 0, 3),
    Item::new("Bandedmail", 75, 0, 4),
    Item::new("Platemail", 102, 0, 5),
];

// :'<,'>s/\([a-zA-Z]\+ +\d\)\s*\(\d\+\)\s*\(\d\+\)\s*\(\d\)/    Item::new("\1\", \2\, \3, \4),
const RINGS: [Item; 6] = [
    Item::new("Damage +1", 25, 1, 0),
    Item::new("Damage +2", 50, 2, 0),
    Item::new("Damage +3", 100, 3, 0),
    Item::new("Defense +1", 20, 0, 1),
    Item::new("Defense +2", 40, 0, 2),
    Item::new("Defense +3", 80, 0, 3),
];

fn weapons() -> impl Iterator<Item = Item> {
    IntoIterator::into_iter(WEAPONS)
}

fn armors() -> impl Iterator<Item = Option<Item>> {
    IntoIterator::into_iter(ARMORS)
        .map(Some)
        .chain(iter::once(None))
}

fn rings() -> impl Iterator<Item = [Option<Item>; 2]> {
    let zero = iter::once([None, None]);
    let one = IntoIterator::into_iter(RINGS).map(|ring| [Some(ring), None]);
    let two = (0..RINGS.len())
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| [Some(RINGS[i]), Some(RINGS[j])]);

    zero.chain(one).chain(two)
}

fn players() -> impl Iterator<Item = Player> {
    weapons().flat_map(move |weapon| {
        armors().flat_map(move |armor| {
            rings().map(move |rings| Player {
                weapon,
                armor,
                rings,
            })
        })
    })
}

impl Player {
    fn damage(&self) -> i64 {
        self.sum(|item| item.damage)
    }

    fn armor(&self) -> i64 {
        self.sum(|item| item.armor)
    }

    fn cost(&self) -> i64 {
        self.sum(|item| item.cost)
    }

    fn sum(&self, project: fn(Item) -> i64) -> i64 {
        project(self.weapon)
            + self.armor.map_or(0, project)
            + IntoIterator::into_iter(self.rings)
                .flatten()
                .map(project)
                .sum::<i64>()
    }
}

#[derive(Copy, Clone, Debug)]
enum Victor {
    Boss,
    Player,
}

impl RPGSimulator20XX {
    fn simulate(&self, player: &Player) -> Victor {
        let player_attack = cmp::max(1, player.damage() - self.armor);
        let boss_attack = cmp::max(1, self.damage - player.armor());

        let player_alive = (100f64 / boss_attack as f64).ceil() as i64;
        let boss_alive = (self.health as f64 / player_attack as f64).ceil() as i64;

        if player_alive >= boss_alive {
            Victor::Player
        } else {
            Victor::Boss
        }
    }
}

impl Solution for RPGSimulator20XX {
    fn one(self) -> i64 {
        players()
            .filter(|player| matches!(self.simulate(player), Victor::Player))
            .map(|player| player.cost())
            .min()
            .unwrap()
    }

    fn two(self) -> i64 {
        players()
            .filter(|player| matches!(self.simulate(player), Victor::Boss))
            .map(|player| player.cost())
            .max()
            .unwrap()
    }
}
