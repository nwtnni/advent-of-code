use std::collections::BTreeMap;
use std::collections::HashMap;
use std::num;
use std::str;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Leaderboard {
    pub event: aoc_core::Year,
    pub members: HashMap<Id, Member>,
    pub owner_id: Id,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Member {
    pub id: Id,
    pub name: Option<String>,
    pub local_score: usize,
    pub global_score: usize,
    #[serde(with = "ts_opt")]
    pub last_star_ts: Option<chrono::DateTime<chrono::Local>>,
    pub stars: usize,
    pub completion_day_level: BTreeMap<aoc_core::Day, Day>,
}

#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug)]
pub struct Day {
    #[serde(rename = "1")]
    pub one: Part,
    #[serde(rename = "2")]
    pub two: Option<Part>,
}

#[derive(
    serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct Part {
    #[serde(with = "ts")]
    pub get_star_ts: chrono::DateTime<chrono::Local>,
}

#[derive(
    serde::Deserialize, serde::Serialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct Id(#[serde(with = "id")] pub usize);

impl str::FromStr for Id {
    type Err = num::ParseIntError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        usize::from_str(string).map(Id)
    }
}

mod id {
    use serde::de::Error as _;
    use serde::Deserialize as _;
    use serde::Serialize as _;

    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<usize, D::Error> {
        String::deserialize(deserializer)?
            .parse::<usize>()
            .map_err(D::Error::custom)
    }

    pub fn serialize<S: serde::Serializer>(id: &usize, serializer: S) -> Result<S::Ok, S::Error> {
        String::serialize(&id.to_string(), serializer)
    }
}

mod ts {
    use std::fmt;

    use aoc_core::Tap as _;
    use chrono::TimeZone as _;
    use serde::Serialize as _;

    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<chrono::DateTime<chrono::Local>, D::Error> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = chrono::DateTime<chrono::Local>;
            fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                write!(fmt, "a valid Unix timestamp")
            }

            fn visit_i64<E: serde::de::Error>(self, ts: i64) -> Result<Self::Value, E> {
                chrono::Utc
                    .timestamp_opt(ts, 0)
                    .unwrap()
                    .with_timezone(&chrono::Local)
                    .tap(Result::Ok)
            }

            fn visit_u64<E: serde::de::Error>(self, ts: u64) -> Result<Self::Value, E> {
                self.visit_i64(ts as i64)
            }

            fn visit_str<E: serde::de::Error>(self, ts: &str) -> Result<Self::Value, E> {
                ts.parse::<i64>()
                    .map_err(E::custom)
                    .and_then(|ts| self.visit_i64(ts))
            }
        }

        deserializer.deserialize_any(Visitor)
    }

    pub fn serialize<S: serde::Serializer>(
        time: &chrono::DateTime<chrono::Local>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        String::serialize(&time.timestamp().to_string(), serializer)
    }
}

mod ts_opt {
    use std::fmt;

    use aoc_core::Tap as _;
    use chrono::TimeZone as _;
    use serde::Serialize as _;

    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<chrono::DateTime<chrono::Local>>, D::Error> {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Option<chrono::DateTime<chrono::Local>>;
            fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                write!(fmt, "a valid Unix timestamp, or 0 if not present")
            }

            fn visit_i64<E: serde::de::Error>(self, ts: i64) -> Result<Self::Value, E> {
                match ts {
                    0 => Ok(None),
                    _ => chrono::Utc
                        .timestamp_opt(ts, 0)
                        .unwrap()
                        .with_timezone(&chrono::Local)
                        .tap(Option::Some)
                        .tap(Result::Ok),
                }
            }

            fn visit_u64<E: serde::de::Error>(self, ts: u64) -> Result<Self::Value, E> {
                self.visit_i64(ts as i64)
            }

            fn visit_str<E: serde::de::Error>(self, ts: &str) -> Result<Self::Value, E> {
                ts.parse::<i64>()
                    .map_err(E::custom)
                    .and_then(|ts| self.visit_i64(ts))
            }
        }

        deserializer.deserialize_any(Visitor)
    }

    pub fn serialize<S: serde::Serializer>(
        time: &Option<chrono::DateTime<chrono::Local>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match time {
            None => i64::serialize(&0, serializer),
            Some(time) => String::serialize(&time.timestamp().to_string(), serializer),
        }
    }
}
