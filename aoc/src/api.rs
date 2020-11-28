use anyhow::anyhow;
use reqwest::blocking;
use reqwest::header;

use crate::cache;

static ROOT: &str = "https://adventofcode.com";

static CORRECT: &str = "TODO";
static INCORRECT: &str = "That's not the right answer.";

pub struct Client {
    _cache: cache::Cache,
    inner: blocking::Client,
}

#[derive(serde::Serialize)]
#[derive(Clone, Debug)]
struct Submission {
    #[serde(rename = "level")]
    part: aoc_core::Part,
    answer: i64,
}

impl Client {
    pub fn new(token: Option<&str>) -> anyhow::Result<Self> {
        let mut headers = header::HeaderMap::new();

        if let Some(token) = token {
            headers.insert(
                header::COOKIE,
                header::HeaderValue::from_str(&format!("session={}", token))?,
            );
        }

        Ok(Client {
            _cache: cache::Cache::new()?,
            inner: blocking::ClientBuilder::new()
                .user_agent("aoc 0.1.0 (nwtnni@gmail.com) (https://github.com/nwtnni/advent-of-code)")
                .default_headers(headers)
                .build()?,
        })
    }

    pub fn description(&self, year: aoc_core::Year, day: aoc_core::Day) -> anyhow::Result<String> {
        self.inner
            .get(&format!("{}/{}/{}", ROOT, year, day))
            .send()?
            .text()
            .map(Result::Ok)?
    }

    pub fn input(&self, year: aoc_core::Year, day: aoc_core::Day) -> anyhow::Result<String> {
        self.inner
            .get(&format!("{}/{}/{}/input", ROOT, year, day))
            .send()?
            .text()
            .map(Result::Ok)?
    }

    pub fn submit(
        &self,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: aoc_core::Part,
        answer: i64,
    ) -> anyhow::Result<bool> {
        self.inner
            .post(&format!("{}/{}/{}/answer", ROOT, year, day))
            .form(&Submission { part, answer })
            .send()?
            .text()
            .map_err(anyhow::Error::from)
            .and_then(|text| {
                if text.contains(INCORRECT) {
                    Ok(false)
                } else if text.contains(CORRECT) {
                    Ok(true)
                } else {
                    Err(anyhow!("[INTERNAL ERROR]: outdated answer patterns"))
                }
            })
    }
}
