use std::fs;
use std::io;
use std::io::Write as _;
use std::path;

use anyhow::anyhow;
use anyhow::Context as _;

use crate::leaderboard;

#[derive(Debug)]
pub struct Cache {
    id: leaderboard::Id,
    project: dirs::ProjectDirs,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Mode {
    Append,
    Replace,
}

impl Cache {
    pub fn new(id: leaderboard::Id) -> anyhow::Result<Self> {
        dirs::ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
            .map(|project| Cache { id, project })
            .ok_or_else(|| anyhow!("[INTERNAL ERROR]: could not retrieve a valid home directory"))
    }

    pub fn description(
        &self,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: aoc_core::Part,
    ) -> anyhow::Result<Option<String>> {
        let path = self
            .project
            .cache_dir()
            .join(year.to_static_str())
            .join(day.to_static_str())
            .join(part.to_static_str())
            .join("description");

        self.read(&path)
    }

    pub fn set_description(
        &self,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: aoc_core::Part,
        description: &str,
    ) -> anyhow::Result<()> {
        let path = self
            .project
            .cache_dir()
            .join(year.to_static_str())
            .join(day.to_static_str())
            .join(part.to_static_str());

        self.write(&path, "description", Some(description), Mode::Replace)
    }

    pub fn input(&self, year: aoc_core::Year, day: aoc_core::Day) -> anyhow::Result<Option<String>> {
        let path = self
            .project
            .cache_dir()
            .join(self.id.0.to_string())
            .join(year.to_static_str())
            .join(day.to_static_str())
            .join("input");

        self.read(&path)
    }

    pub fn set_input(
        &self,
        year: aoc_core::Year,
        day: aoc_core::Day,
        input: &str,
    ) -> anyhow::Result<()> {
        let path = self
            .project
            .cache_dir()
            .join(self.id.0.to_string())
            .join(year.to_static_str())
            .join(day.to_static_str());

        self.write(&path, "input", Some(input), Mode::Replace)
    }

    pub fn completed(
        &self,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: aoc_core::Part,
    ) -> bool {
        self.project
            .cache_dir()
            .join(self.id.0.to_string())
            .join(year.to_static_str())
            .join(day.to_static_str())
            .join(part.to_static_str())
            .join("completed")
            .exists()
    }

    pub fn set_completed(
        &self,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: aoc_core::Part,
        correct: bool,
    ) -> anyhow::Result<()> {
        if !correct {
            return Ok(());
        }

        let path = self
            .project
            .cache_dir()
            .join(self.id.0.to_string())
            .join(year.to_static_str())
            .join(day.to_static_str())
            .join(part.to_static_str());

        self.write(&path, "completed", None, Mode::Replace)
    }

    pub fn submitted(
        &self,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: aoc_core::Part,
    ) -> anyhow::Result<Vec<i64>> {
        let path = self
            .project
            .cache_dir()
            .join(self.id.0.to_string())
            .join(year.to_static_str())
            .join(day.to_static_str())
            .join(part.to_static_str())
            .join("submitted");

        match self.read(&path)? {
        | None => Ok(Vec::new()),
        | Some(submitted) => submitted
            .trim()
            .split_whitespace()
            .map(|answer| answer.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(anyhow::Error::from),
        }
    }

    pub fn append_submitted(
        &self,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: aoc_core::Part,
        answer: i64,
    ) -> anyhow::Result<()> {
        let path = self
            .project
            .cache_dir()
            .join(self.id.0.to_string())
            .join(year.to_static_str())
            .join(day.to_static_str())
            .join(part.to_static_str());

        self.write(&path, "submitted", Some(&answer.to_string()), Mode::Append)
    }

    fn read(
        &self,
        path: &path::Path,
    ) -> anyhow::Result<Option<String>> {
        log::info!("Reading from {}", path.display());

        match fs::read_to_string(path) {
        | Ok(description) => Ok(Some(description)),
        | Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        | Err(error) => Err(anyhow::Error::from(error))
        }
    }

    fn write(
        &self,
        path: &path::Path,
        file: &'static str,
        data: Option<&str>,
        mode: Mode,
    ) -> anyhow::Result<()> {
        fs::create_dir_all(path)
            .with_context(|| anyhow!("Could not create cache directory: {}", path.display()))?;

        let path = path.join(file);

        log::info!("Writing to {}", path.display());

        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append({
                match mode {
                | Mode::Append => true,
                | Mode::Replace => false,
                }
            })
            .open(&path)
            .map(io::BufWriter::new)
            .with_context(|| anyhow!("Could not open cache file: {}", path.display()))?;

        if let Some(data) = data {
            write!(&mut file, "{}", data)?;
            file.flush()?;
        }

        Ok(())
    }
}
