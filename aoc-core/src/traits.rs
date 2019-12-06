use std::any;
use std::str;

use crate::types::Part;

pub trait Solution: Fro + Sized {
    fn one(self) -> i32;
    fn two(self) -> i32;
    fn run(input: &str, part: Part) -> i32 {
        let solution = input.to::<Self>();
        let output = match part {
        | Part::P01 => solution.one(),
        | Part::P02 => solution.two(),
        };
        output
    }
}

pub trait Fro {
    fn fro(string: &str) -> Self;
}

impl<T> Fro for T where T: str::FromStr {
    fn fro(string: &str) -> Self {
        match string.parse::<T>() {
        | Ok(value) => value,
        | Err(_) => panic!("Could not parse {} as {}", string, any::type_name::<T>()),
        }
    }
}

pub trait To {
    fn to<T: Fro>(&self) -> T;
}

impl To for String {
    fn to<T: Fro>(&self) -> T {
        T::fro(self)
    }
}

impl To for str {
    fn to<T: Fro>(&self) -> T {
        T::fro(self)
    }
}

pub trait Tap: Sized {
    fn tap<T, F: FnOnce(Self) -> T>(self, apply: F) -> T {
        apply(self)
    }

    fn tap_mut<T, F: FnOnce(&mut Self)>(mut self, apply: F) -> Self {
        apply(&mut self);
        self
    }
}

impl<T: Sized> Tap for T {}

pub trait Give: Iterator {
    fn give(&mut self) -> <Self as Iterator>::Item;
}

impl<T, I: Iterator<Item = T>> Give for I {
    fn give(&mut self) -> T {
        self.next().unwrap()
    }
}

static DIV: [i32; 10] = [
             1,
            10,
           100,
         1_000,
        10_000,
       100_000,
     1_000_000,
    10_000_000,
   100_000_000,
 1_000_000_000,
];

pub trait Digits {
    fn digit(&self, i: i32) -> Self;
}

impl Digits for i32 {
    fn digit(&self, i: i32) -> Self {
        assert!(i >= 0 && i < 10);
        (self / DIV[i as usize]) % 10
    }
}

pub trait Leak {
    fn leak(&self) -> &'static Self;
}

impl Leak for str {
    fn leak(&self) -> &'static Self {
        Box::leak(self.to_owned().into_boxed_str())
    }
}
