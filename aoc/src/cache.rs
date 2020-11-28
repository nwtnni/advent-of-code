use std::fs;
use std::io;
use std::path;

use anyhow::anyhow;
use anyhow::Context as _;

#[derive(Debug)]
pub struct Cache(dirs::ProjectDirs);

impl Cache {
    pub fn new() -> anyhow::Result<Self> {
        dirs::ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
            .map(Cache)
            .ok_or_else(|| anyhow!("[INTERNAL ERROR]: could not retrieve a valid home directory"))
    }

    pub fn description(&self, year: aoc_core::Year, day: aoc_core::Day) -> anyhow::Result<Option<String>> {
        let dir = self
            .0
            .cache_dir()
            .join(year.to_static_str())
            .join(day.to_static_str());

        self.read(&dir, "description")
    }

    pub fn input(&self, token: &str, year: aoc_core::Year, day: aoc_core::Day) -> anyhow::Result<Option<String>> {
        let dir = self
            .0
            .cache_dir()
            .join(token)
            .join(year.to_static_str())
            .join(day.to_static_str());

        self.read(&dir, "input")
    }

    pub fn completed(
        &self,
        token: &str,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: aoc_core::Part,
    ) -> anyhow::Result<Option<bool>> {
        let dir = self
            .0
            .cache_dir()
            .join(token)
            .join(year.to_static_str())
            .join(day.to_static_str())
            .join(part.to_static_str());

        match self.read(&dir, "completed")?.as_deref() {
        | Some("true") => Ok(Some(true)),
        | Some("false") => Ok(Some(false)),
        | Some(unknown) => Err(anyhow!("[INTERNAL ERROR]: unknown value for `completed`: '{}'", unknown)),
        | None => Ok(None),
        }
    }

    pub fn submitted(
        &self,
        token: &str,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: aoc_core::Part,
    ) -> anyhow::Result<Vec<i64>> {
        let dir = self
            .0
            .cache_dir()
            .join(token)
            .join(year.to_static_str())
            .join(day.to_static_str())
            .join(part.to_static_str());

        match self.read(&dir, "submitted")? {
        | None => return Ok(Vec::new()),
        | Some(submitted) => submitted
            .trim()
            .split_whitespace()
            .map(|submission| submission.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(anyhow::Error::from),
        }
    }

    fn read(
        &self,
        dir: &path::Path,
        file: &'static str,
    ) -> anyhow::Result<Option<String>> {
        fs::create_dir_all(&dir)
            .with_context(|| anyhow!("Could not create directory: {}", dir.display()))?;

        match fs::read_to_string(dir.join(file)) {
        | Ok(description) => Ok(Some(description)),
        | Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        | Err(error) => Err(anyhow::Error::from(error))
        }
    }
}
