use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct ReposeRecord(Vec<(Time, Event)>);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Time {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

#[derive(Copy, Clone, Debug)]
enum Event {
    Begin(i64),
    Wake,
    Sleep,
}

impl Fro for ReposeRecord {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (time, event) = line.split_once("] ").unwrap();
                let time = time.trim_start_matches('[');
                (Time::fro(time), Event::fro(event))
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Fro for Time {
    fn fro(input: &str) -> Self {
        let (date, time) = input.split_once(' ').unwrap();

        let mut iter = date.split('-');
        let year = iter.next().unwrap().tap(u16::fro);
        let month = iter.next().unwrap().tap(u8::fro);
        let day = iter.next().unwrap().tap(u8::fro);

        let (hour, minute) = time.split_once(':').unwrap();
        let hour = u8::fro(hour);
        let minute = u8::fro(minute);

        Self {
            year,
            month,
            day,
            hour,
            minute,
        }
    }
}

impl Fro for Event {
    fn fro(input: &str) -> Self {
        match input {
            "wakes up" => Self::Wake,
            "falls asleep" => Self::Sleep,
            _ => {
                let id = input
                    .trim_start_matches("Guard #")
                    .trim_end_matches(" begins shift")
                    .tap(i64::fro);
                Self::Begin(id)
            }
        }
    }
}

impl Solution for ReposeRecord {
    fn one(self) -> i64 {
        let guards = self.compile();

        let (guard, schedule) = guards
            .iter()
            .max_by_key(|(_, schedule)| schedule.iter().sum::<u16>())
            .unwrap();

        let minute = (0..60)
            .max_by_key(|minute| schedule[*minute as usize])
            .unwrap();

        guard * minute
    }

    fn two(self) -> i64 {
        let guards = self.compile();

        let (guard, minute, _) = guards
            .iter()
            .map(|(guard, schedule)| {
                let (minute, total) = (0..60)
                    .map(|minute| (minute, schedule[minute as usize]))
                    .max_by_key(|(_, total)| *total)
                    .unwrap();

                (guard, minute, total)
            })
            .max_by_key(|(_, _, total)| *total)
            .unwrap();

        *guard * minute
    }
}

impl ReposeRecord {
    fn compile(mut self) -> HashMap<i64, [u16; 60]> {
        self.0.sort_by_key(|(time, _)| *time);

        let mut guards = HashMap::new();
        let mut current = 0;
        let mut started = 0;

        for (Time { minute, .. }, event) in self.0 {
            match event {
                Event::Begin(guard) => current = guard,
                Event::Sleep => started = minute as usize,
                Event::Wake => guards.entry(current).or_insert([0; 60])[started..minute as usize]
                    .iter_mut()
                    .for_each(|asleep| *asleep += 1),
            }
        }

        guards
    }
}
