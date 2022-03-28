use std::cmp;
use std::collections::HashSet;

use aoc::*;
use priority_queue::PriorityQueue;

#[derive(Clone, Debug)]
pub struct WizardSimulator20XX {
    health: i64,
    damage: i64,
}

impl Fro for WizardSimulator20XX {
    fn fro(input: &str) -> Self {
        let mut iter = input.trim().split('\n');
        Self {
            health: iter.give().trim_start_matches("Hit Points: ").tap(i64::fro),
            damage: iter.give().trim_start_matches("Damage: ").tap(i64::fro),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    health_boss: u8,
    health_player: u8,
    mana_player: u16,
    timer_shield: u8,
    timer_poison: u8,
    timer_recharge: u8,
}

const COST_MAGIC_MISSILE: u16 = 53;
const COST_DRAIN: u16 = 73;
const COST_SHIELD: u16 = 113;
const COST_POISON: u16 = 173;
const COST_RECHARGE: u16 = 229;

const HEALTH_PLAYER: u8 = 50;
const MANA_PLAYER: u16 = 500;

impl State {
    fn step(mut self, hard: bool, damage: u8) -> impl Iterator<Item = (u16, Self)> {
        if hard {
            self.health_player -= 1;
        }

        let _ = self.tick();

        IntoIterator::into_iter([
            self.magic_missile(),
            self.drain(),
            self.shield(),
            self.poison(),
            self.recharge(),
        ])
        .flatten()
        .map(move |(cost, mut state)| {
            let armor = if state.tick() { 7 } else { 0 };
            state.health_player = state.health_player.saturating_sub(damage - armor);
            (cost, state)
        })
        .filter(|(_, state)| state.health_boss == 0 || state.health_player > 0)
    }

    fn tick(&mut self) -> bool {
        let shield = self.timer_shield > 0;
        let poison = self.timer_poison > 0;
        let recharge = self.timer_recharge > 0;

        self.timer_shield = self.timer_shield.saturating_sub(1);
        self.timer_poison = self.timer_poison.saturating_sub(1);
        self.timer_recharge = self.timer_recharge.saturating_sub(1);

        if poison {
            self.health_boss = self.health_boss.saturating_sub(3);
        }

        if recharge {
            self.mana_player += 101;
        }

        shield
    }

    fn magic_missile(&self) -> Option<(u16, Self)> {
        Some((
            COST_MAGIC_MISSILE,
            Self {
                health_boss: self.health_boss.saturating_sub(4),
                mana_player: self.mana_player.checked_sub(COST_MAGIC_MISSILE)?,
                ..*self
            },
        ))
    }

    fn drain(&self) -> Option<(u16, Self)> {
        Some((
            COST_DRAIN,
            Self {
                health_boss: self.health_boss.saturating_sub(2),
                health_player: self.health_player + 2,
                mana_player: self.mana_player.checked_sub(COST_DRAIN)?,
                ..*self
            },
        ))
    }

    fn effect(
        &self,
        cost: u16,
        turns: u8,
        project: fn(&mut State) -> &mut u8,
    ) -> Option<(u16, Self)> {
        let mut state = Self {
            mana_player: self.mana_player.checked_sub(cost)?,
            ..*self
        };

        if *project(&mut state) == 0 {
            *project(&mut state) = turns;
            Some((cost, state))
        } else {
            None
        }
    }

    fn shield(&self) -> Option<(u16, Self)> {
        self.effect(COST_SHIELD, 6, |state| &mut state.timer_shield)
    }

    fn poison(&self) -> Option<(u16, Self)> {
        self.effect(COST_POISON, 6, |state| &mut state.timer_poison)
    }

    fn recharge(&self) -> Option<(u16, Self)> {
        self.effect(COST_RECHARGE, 5, |state| &mut state.timer_recharge)
    }
}

impl Solution for WizardSimulator20XX {
    fn one(self) -> i64 {
        self.search(false)
    }

    fn two(self) -> i64 {
        self.search(true)
    }
}

impl WizardSimulator20XX {
    fn search(self, hard: bool) -> i64 {
        let mut seen = HashSet::new();
        let mut queue = PriorityQueue::new();

        let start = State {
            health_boss: self.health as u8,
            health_player: HEALTH_PLAYER,
            mana_player: MANA_PLAYER,
            timer_shield: 0,
            timer_poison: 0,
            timer_recharge: 0,
        };

        queue.push(start, cmp::Reverse(0));

        while let Some((state, cmp::Reverse(cost))) = queue.pop() {
            if state.health_boss == 0 {
                return cost;
            }

            seen.insert(state);

            for (mana, next) in state.step(hard, self.damage as u8) {
                if seen.contains(&next) {
                    continue;
                } else {
                    queue.push_increase(next, cmp::Reverse(cost + mana as i64));
                }
            }
        }

        unreachable!()
    }
}
