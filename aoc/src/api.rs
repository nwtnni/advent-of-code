use anyhow::anyhow;
use reqwest::blocking;
use reqwest::header;

use crate::cache;
use crate::markdown;

pub static ROOT: &str = "https://adventofcode.com";

static CORRECT: &str = "That's the right answer!";
static INCORRECT: &str = "That's not the right answer.";
static COMPLETED: &str = "You don't seem to be solving the right level.";

pub struct Client {
    cache: cache::Cache,
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
    pub fn new(cache: cache::Cache, token: &str) -> anyhow::Result<Self> {
        let mut headers = header::HeaderMap::new();

        headers.insert(
            header::COOKIE,
            header::HeaderValue::from_str(&format!("session={}", token))?,
        );

        Ok(Client {
            cache,
            inner: blocking::ClientBuilder::new()
                .user_agent("aoc 0.1.0 (nwtnni@gmail.com) (https://github.com/nwtnni/advent-of-code)")
                .default_headers(headers)
                .build()?,
        })
    }

    pub fn description(
        &self,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: aoc_core::Part,
    ) -> anyhow::Result<String> {
        if let Some(description) = self.cache.description(year, day, part)? {
            return Ok(description);
        }

        let html = self.inner
            .get(&format!("{}/{}/day/{}", ROOT, year, day))
            .send()?
            .text()
            .map(|text| scraper::Html::parse_document(&text))?;

        let selector = scraper::Selector::parse("article.day-desc")
            .expect("[INTERNAL ERROR]: invalid CSS selector");

        let mut description = html
            .select(&selector)
            .nth(part as usize - 1)
            .ok_or_else(|| anyhow!("Could not retrieve description for {}-{}-{}", year, day, part))
            .map(|html| markdown::from_html(html, year))?;

        // Remove trailing whitespace
        // https://users.rust-lang.org/t/trim-string-in-place/15809/7
        description.truncate(description.trim_end().len());

        // self.cache.set_description(year, day, part, &description)?;
        Ok(description)
    }

    pub fn input(&self, year: aoc_core::Year, day: aoc_core::Day) -> anyhow::Result<String> {
        if let Some(input) = self.cache.input(year, day)? {
            return Ok(input);
        }

        let input = self.inner
            .get(&format!("{}/{}/day/{}/input", ROOT, year, day))
            .send()?
            .text()?;

        self.cache.set_input(year, day, &input)?;
        Ok(input)
    }

    pub fn submit(
        &self,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: aoc_core::Part,
        answer: i64,
    ) -> anyhow::Result<bool> {
        let completed = self.cache.completed(year, day, part);
        let submitted = self.cache.submitted(year, day, part)?;

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
                } else if text.contains(COMPLETED) {
                    Err(anyhow!("[USAGE ERROR]: puzzle has already been solved"))
                } else {
                    Err(anyhow!("[INTERNAL ERROR]: unexpected response: {}", text))
                }
            })?;

        self.cache.append_submitted(year, day, part, answer)?;
        self.cache.set_completed(year, day, part, correct)?;
        Ok(correct)
    }
}
