#[macro_use]
extern crate nom;

use std::collections::HashMap as Map;

use nom::types::CompleteStr;

const INPUT: CompleteStr = CompleteStr(include_str!("input.txt"));

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    min: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Begin(usize),
    Sleep,
    Wake,
}

#[derive(Copy, Clone, Debug)]
pub struct Entry {
    time: Time,
    event: Event,
}

named!(parse_usize<CompleteStr, usize>,
    map!(nom::digit, |s| s.parse::<usize>().unwrap())
);

named!(parse_time<CompleteStr, Time>,
    do_parse!(
               tag_s!("[") >>
        year:  parse_usize >>
               tag_s!("-") >>
        month: parse_usize >>
               tag_s!("-") >>
        day:   parse_usize >>
               tag_s!(" ") >>
        hour:  parse_usize >>
               tag_s!(":") >>
        min:   parse_usize >>
               tag_s!("]") >>
        (Time { year, month, day, hour, min })
    )
);

named!(parse_event<CompleteStr, Event>,
    alt!(
        value!(Event::Sleep, tag_s!("falls asleep")) |
        value!(Event::Wake, tag_s!("wakes up")) |
        do_parse!(
                tag_s!("Guard #")       >>
            id: parse_usize             >>
                tag_s!(" begins shift") >>
            (Event::Begin(id))
        )
    )
);

named!(parse_entries<CompleteStr, Vec<Entry>>,
    separated_list!(
        tag_s!("\n"),
        do_parse!(
            time: parse_time   >>
                  tag_s!(" ")  >>
            event: parse_event >>
            (Entry { time, event })
        )
    )
);

fn main() {

    let (_, mut entries) = parse_entries(INPUT).unwrap();
    entries.sort_by_key(|entry| entry.time);

    let mut start = Time::default();
    let mut guard = 0;
    let mut schedule: Map<usize, Map<usize, usize>> = Map::default();

    for entry in entries {
        match entry.event {
        | Event::Begin(id) => {
            guard = id;
        }
        | Event::Sleep => {
            start = entry.time;
        }
        | Event::Wake => {
            let guard_schedule = schedule
                .entry(guard)
                .or_insert_with(Map::default);
            for m in start.min..entry.time.min {
                guard_schedule.entry(m)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
        }
    }

    let max_guard = schedule.iter()
        .map(|(guard, minutes)| (guard, minutes.values().sum::<usize>()))
        .max_by_key(|(_, total)| *total)
        .map(|(guard, _)| *guard)
        .unwrap();

    let max_minute = &schedule[&max_guard]
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(minute, _)| *minute)
        .unwrap();

    println!("{}", max_guard * max_minute);

    let same_minute = schedule.iter()
        .map(|(guard, minutes)| {
            (guard, minutes.iter().max_by_key(|(_, c)| *c).unwrap())
        })
        .max_by_key(|(_, (_, c))| *c)
        .map(|(guard, (minute, _))| guard * minute)
        .unwrap();

    println!("{}", same_minute);
}
