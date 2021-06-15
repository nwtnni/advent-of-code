use aoc::*;

use serde_json::Value;

#[derive(Clone, Debug)]
pub struct JSAbacusFrameworkio(Value);

impl Fro for JSAbacusFrameworkio {
    fn fro(input: &str) -> Self {
        Value::fro(input).tap(Self)
    }
}

impl Solution for JSAbacusFrameworkio {
    fn one(self) -> i64 {
        fn sum(value: &Value) -> i64 {
            match value {
                Value::Null => 0,
                Value::Number(number) => number.as_i64().unwrap(),
                Value::Bool(_) => 0,
                Value::String(_) => 0,
                Value::Array(array) => array.iter().map(sum).sum(),
                Value::Object(object) => object.values().map(sum).sum(),
            }
        }

        sum(&self.0)
    }

    fn two(self) -> i64 {
        fn sum(value: &Value) -> i64 {
            match value {
                Value::Null => 0,
                Value::Number(number) => number.as_i64().unwrap(),
                Value::Bool(_) => 0,
                Value::String(_) => 0,
                Value::Array(array) => array.iter().map(sum).sum(),
                Value::Object(object) if object.values().any(|value| value == "red") => 0,
                Value::Object(object) => object.values().map(sum).sum(),
            }
        }

        sum(&self.0)
    }
}
