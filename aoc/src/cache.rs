use std::fs;
use std::io;
use std::io::Write as _;
use std::path;

use anyhow::anyhow;
use anyhow::Context as _;

#[derive(Debug)]
pub struct Cache(dirs::ProjectDirs);

#[derive(Copy, Clone, PartialEq, Eq)]
enum Mode {
    Append,
    Replace,
}

impl Cache {
    pub fn new() -> anyhow::Result<Self> {
        dirs::ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
            .map(Cache)
            .ok_or_else(|| anyhow!("[INTERNAL ERROR]: could not retrieve a valid home directory"))
    }

    pub fn description(&self, year: aoc_core::Year, day: aoc_core::Day) -> anyhow::Result<Option<String>> {
        let path = self
            .0
            .cache_dir()
            .join(year.to_static_str())
            .join(day.to_static_str())
            .join("description");

        self.read(&path)
    }

    pub fn set_description(
        &self,
        year: aoc_core::Year,
        day: aoc_core::Day,
        description: &str,
    ) -> anyhow::Result<()> {
        let path = self
            .0
            .cache_dir()
            .join(year.to_static_str())
            .join(day.to_static_str());

        self.write(&path, "description", Some(description), Mode::Replace)
    }

    pub fn input(&self, token: &str, year: aoc_core::Year, day: aoc_core::Day) -> anyhow::Result<Option<String>> {
        let path = self
            .0
            .cache_dir()
            .join(token)
            .join(year.to_static_str())
            .join(day.to_static_str())
            .join("input");

        self.read(&path)
    }

    pub fn set_input(
        &self,
        token: &str,
        year: aoc_core::Year,
        day: aoc_core::Day,
        input: &str,
    ) -> anyhow::Result<()> {
        let path = self
            .0
            .cache_dir()
            .join(token)
            .join(year.to_static_str())
            .join(day.to_static_str());

        self.write(&path, "input", Some(input), Mode::Replace)
    }

    pub fn completed(
        &self,
        token: &str,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: aoc_core::Part,
    ) -> bool {
        self.0
            .cache_dir()
            .join(token)
            .join(year.to_static_str())
            .join(day.to_static_str())
            .join(part.to_static_str())
            .join("completed")
            .exists()
    }

    pub fn set_completed(
        &self,
        token: &str,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: aoc_core::Part,
    ) -> anyhow::Result<()> {
        let path = self
            .0
            .cache_dir()
            .join(token)
            .join(year.to_static_str())
            .join(day.to_static_str())
            .join(part.to_static_str());

        self.write(&path, "completed", None, Mode::Replace)
    }

    pub fn submitted(
        &self,
        token: &str,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: aoc_core::Part,
    ) -> anyhow::Result<Vec<i64>> {
        let path = self
            .0
            .cache_dir()
            .join(token)
            .join(year.to_static_str())
            .join(day.to_static_str())
            .join(part.to_static_str())
            .join("submitted");

        match self.read(&path)? {
        | None => Ok(Vec::new()),
        | Some(submitted) => submitted
            .trim()
            .split_whitespace()
            .map(|submission| submission.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(anyhow::Error::from),
        }
    }

    pub fn append_submitted(
        &self,
        token: &str,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: aoc_core::Part,
        submission: i64,
    ) -> anyhow::Result<()> {
        let path = self
            .0
            .cache_dir()
            .join(token)
            .join(year.to_static_str())
            .join(day.to_static_str())
            .join(part.to_static_str());

        self.write(&path, "submitted", Some(&submission.to_string()), Mode::Append)
    }

    fn read(
        &self,
        path: &path::Path,
    ) -> anyhow::Result<Option<String>> {
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

        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append({
                match mode {
                | Mode::Append => true,
                | Mode::Replace => false,
                }
            })
            .open(path.join(file))
            .map(io::BufWriter::new)
            .with_context(|| anyhow!("Could not open cache file: {}", path.display()))?;

        if let Some(data) = data {
            writeln!(&mut file, "{}", data)?;
        }

        Ok(())
    }
}
