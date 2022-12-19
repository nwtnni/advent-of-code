use aoc::*;
use priority_queue::PriorityQueue;

#[derive(Clone, Debug)]
pub struct NotEnoughMinerals(Vec<Blueprint>);

#[derive(Copy, Clone, Debug)]
struct Blueprint {
    ore: i64,
    clay: i64,
    obsidian: (i64, i64),
    geode: (i64, i64),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    time: i64,
    ore: i64,
    ore_bot: i64,
    clay: i64,
    clay_bot: i64,
    obsidian: i64,
    obsidian_bot: i64,
    geode: i64,
    geode_bot: i64,
}

impl State {
    fn transition(&self, blueprint: &Blueprint) -> impl Iterator<Item = State> {
        let base = State {
            time: self.time + 1,
            ore: self.ore + self.ore_bot,
            ore_bot: self.ore_bot,
            clay: self.clay + self.clay_bot,
            clay_bot: self.clay_bot,
            obsidian: self.obsidian + self.obsidian_bot,
            obsidian_bot: self.obsidian_bot,
            geode: self.geode + self.geode_bot,
            geode_bot: self.geode_bot,
        };

        [
            (self.ore >= blueprint.geode.0 && self.obsidian >= blueprint.geode.1).then_some(
                State {
                    ore: self.ore - blueprint.geode.0,
                    obsidian: self.obsidian - blueprint.geode.1,
                    geode_bot: self.geode_bot + 1,
                    ..base
                },
            ),
            (self.ore >= blueprint.obsidian.0 && self.clay >= blueprint.obsidian.1).then_some(
                State {
                    ore: self.ore - blueprint.obsidian.0,
                    clay: self.clay - blueprint.obsidian.1,
                    obsidian_bot: self.obsidian_bot + 1,
                    ..base
                },
            ),
            (self.ore >= blueprint.clay).then_some(State {
                ore: self.ore - blueprint.clay,
                clay_bot: self.clay_bot + 1,
                ..base
            }),
            (self.ore >= blueprint.ore).then_some(State {
                ore: self.ore - blueprint.ore,
                ore_bot: self.ore_bot + 1,
                ..base
            }),
            Some(base),
        ]
        .into_iter()
        .flatten()
    }
}

impl Fro for NotEnoughMinerals {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (_, a) = line.split_once("Each ore robot costs ").unwrap();
                let (a, b) = a.split_once(" ore. Each clay robot costs ").unwrap();
                let (b, c) = b.split_once(" ore. Each obsidian robot costs ").unwrap();
                let (c, d) = c.split_once(" ore and ").unwrap();
                let (d, e) = d.split_once(" clay. Each geode robot costs ").unwrap();
                let (e, f) = e.split_once(" ore and ").unwrap();
                let f = f.trim_end_matches(" obsidian.");

                Blueprint {
                    ore: i64::fro(a),
                    clay: i64::fro(b),
                    obsidian: (i64::fro(c), i64::fro(d)),
                    geode: (i64::fro(e), i64::fro(f)),
                }
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for NotEnoughMinerals {
    fn one(self) -> i64 {
        dbg!(&self);

        self.0.iter().map(check).max().unwrap()
    }

    fn two(self) -> i64 {
        todo!()
    }
}

fn check(blueprint: &Blueprint) -> i64 {
    let mut queue = PriorityQueue::new();
    // let mut queue = VecDeque::new();

    queue.push(
        State {
            time: 0,
            ore: 0,
            ore_bot: 1,
            clay: 0,
            clay_bot: 0,
            obsidian: 0,
            obsidian_bot: 0,
            geode: 0,
            geode_bot: 0,
        },
        0,
    );

    //     queue.push_back(State {
    //         time: 0,
    //         ore: 0,
    //         ore_bot: 1,
    //         clay: 0,
    //         clay_bot: 0,
    //         obsidian: 0,
    //         obsidian_bot: 0,
    //         geode: 0,
    //         geode_bot: 0,
    //     });

    // let mut max = i64::MIN;

    while let Some((state, _)) = queue.pop() {
        // while let Some(state) = queue.pop_front() {
        if state.time < 10 {
            dbg!(&state);
        }

        if state.time == 24 {
            return state.geode;
            // max = cmp::max(max, state.geode);
            // continue;
        }

        for next in state.transition(blueprint) {
            queue.push_increase(next, next.geode);
        }

        // for next in state.transition(blueprint) {
        //     queue.push_back(next);
        // }
    }

    // max
    unreachable!()
}
