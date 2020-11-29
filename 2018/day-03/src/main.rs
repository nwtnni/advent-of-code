#[macro_use]
extern crate nom;

use std::str::FromStr;
use std::collections::HashMap as Map;

use nom::types::CompleteStr;

const INPUT: &'static str = include_str!("input.txt");

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rect {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Claim {
    id: usize,
    rect: Rect,
}

named!(parse_usize<CompleteStr, usize>,
    map!(nom::digit, |s| FromStr::from_str(s.0).unwrap())
);

named!(parse_rect<CompleteStr, Rect>,
    do_parse!(
        x: parse_usize  >>
           tag_s!(",")  >>
        y: parse_usize  >>
           tag_s!(": ") >>
        w: parse_usize  >>
           tag_s!("x")  >>
        h: parse_usize  >>
        (Rect { x, y, w, h })
    )
);

named!(parse_claim<CompleteStr, Claim>,
    do_parse!(
              tag_s!("#")   >>
        id:   parse_usize   >>
              tag_s!(" @ ") >>
        rect: parse_rect    >>
        (Claim { id, rect })
    )
);

named!(parse_claims<CompleteStr, Vec<Claim>>,
    separated_list!(tag_s!("\n"), parse_claim)
);

fn main() {
    let mut claimed: Map<(usize, usize), Vec<usize>> = Map::default();
    let (_, claims) = parse_claims(CompleteStr(INPUT)).unwrap();

    for claim in &claims {
        for y in claim.rect.y..claim.rect.y + claim.rect.h {
            for x in claim.rect.x..claim.rect.x + claim.rect.w {
                claimed.entry((x, y))
                    .and_modify(|v| v.push(claim.id))
                    .or_insert_with(|| vec![claim.id]);
            }
        }
    }

    let overlap = claimed.values()
        .filter(|count| count.len() > 1)
        .count();

    println!("{}", overlap);

    let mut intact = vec![true; claims.len()];
    for occupied in claimed.values() {
        if occupied.len() > 1 {
            for id in occupied { intact[*id - 1] = false; }
        }
    }

    let last = intact.into_iter()
        .position(|id| id)
        .unwrap() + 1;

    println!("{}", last);
}
