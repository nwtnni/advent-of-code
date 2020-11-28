use anyhow::anyhow;
use reqwest::blocking;
use reqwest::header;

use crate::cache;

static ROOT: &str = "https://adventofcode.com";

static CORRECT: &str = "TODO";
static INCORRECT: &str = "That's not the right answer.";

pub struct Client {
    cache: cache::Cache,
    inner: blocking::Client,
    token: String,
}

#[derive(serde::Serialize)]
#[derive(Clone, Debug)]
struct Submission {
    #[serde(rename = "level")]
    part: aoc_core::Part,
    answer: i64,
}

impl Client {
    pub fn new(token: String) -> anyhow::Result<Self> {
        let mut headers = header::HeaderMap::new();

        headers.insert(
            header::COOKIE,
            header::HeaderValue::from_str(&format!("session={}", token))?,
        );

        Ok(Client {
            cache: cache::Cache::new()?,
            inner: blocking::ClientBuilder::new()
                .user_agent("aoc 0.1.0 (nwtnni@gmail.com) (https://github.com/nwtnni/advent-of-code)")
                .default_headers(headers)
                .build()?,
            token,
        })
    }

    pub fn description(&self, year: aoc_core::Year, day: aoc_core::Day) -> anyhow::Result<String> {
        if let Some(description) = self.cache.description(year, day)? {
            return Ok(description);
        }

        // TODO: parse HTML to Markdown
        let description = self.inner
            .get(&format!("{}/{}/day/{}", ROOT, year, day))
            .send()?
            .text()?;

        self.cache.set_description(year, day, &description)?;
        Ok(description)
    }

    pub fn input(&self, year: aoc_core::Year, day: aoc_core::Day) -> anyhow::Result<String> {
        if let Some(input) = self.cache.description(year, day)? {
            return Ok(input);
        }

        let input = self.inner
            .get(&format!("{}/{}/day/{}/input", ROOT, year, day))
            .send()?
            .text()?;

        self.cache.set_input(&self.token, year, day, &input)?;
        Ok(input)
    }

    pub fn submit(
        &self,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: aoc_core::Part,
        answer: i64,
    ) -> anyhow::Result<bool> {
        let completed = self.cache.completed(&self.token, year, day, part);
        let submitted = self.cache.submitted(&self.token, year, day, part)?;

        match submitted.last() {
        | Some(last) if completed && answer == *last => return Ok(true),
        | _ if completed || submitted.contains(&answer) => return Ok(false),
        | _ => (),
        }

        let correct = self.inner
            .post(&format!("{}/{}/day/{}/answer", ROOT, year, day))
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
            })?;

        self.cache.append_submitted(&self.token, year, day, part, answer)?;
        if correct {
            self.cache.set_completed(&self.token, year, day, part)?;
        }
        Ok(correct)
    }
}
