use std::fs;
use std::io;

use anyhow::anyhow;
use anyhow::Context as _;

#[derive(Debug)]
pub struct Cache {
    root: dirs::ProjectDirs,
    token: String,
}

impl Cache {
    pub fn new(token: String) -> anyhow::Result<Self> {
        dirs::ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
            .map(|root| Cache { root, token })
            .ok_or_else(|| anyhow!("[INTERNAL ERROR]: could not retrieve a valid home directory"))
    }

    pub fn description(&self, year: aoc_core::Year, day: aoc_core::Day) -> anyhow::Result<Option<String>> {
        self.read(year, day, None, "description")
    }

    pub fn input(&self, year: aoc_core::Year, day: aoc_core::Day) -> anyhow::Result<Option<String>> {
        self.read(year, day, None, "input")
    }

    pub fn completed(
        &self,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: aoc_core::Part,
    ) -> anyhow::Result<Option<bool>> {
        let text = self.read(year, day, Some(part), "completed")?;
        match text.as_deref() {
        | Some("true") => Ok(Some(true)),
        | Some("false") => Ok(Some(false)),
        | Some(unknown) => Err(anyhow!("[INTERNAL ERROR]: unknown value for `completed`: '{}'", unknown)),
        | None => Ok(None),
        }
    }

    pub fn submitted(
        &self,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: aoc_core::Part,
    ) -> anyhow::Result<Vec<i64>> {
        let submitted = match self.read(year, day, Some(part), "submitted")? {
        | None => return Ok(Vec::new()),
        | Some(submitted) => submitted,
        };

        submitted
            .trim()
            .split_whitespace()
            .map(|submission| submission.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(anyhow::Error::from)
    }

    fn read(
        &self,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: Option<aoc_core::Part>,
        file: &'static str,
    ) -> anyhow::Result<Option<String>> {
        let mut dir = self
            .root
            .cache_dir()
            .join(&self.token)
            .join(year.to_static_str())
            .join(day.to_static_str());

        if let Some(part) = part {
            dir.push(part.to_static_str());
        }

        fs::create_dir_all(&dir)
            .with_context(|| anyhow!("Could not create directory: {}", dir.display()))?;

        match fs::read_to_string(dir.join(file)) {
        | Ok(description) => Ok(Some(description)),
        | Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        | Err(error) => Err(anyhow::Error::from(error))
        }
    }
}
