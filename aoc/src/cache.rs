use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Write as _;
use std::path;
use std::time;

use anyhow::anyhow;
use anyhow::Context as _;

use crate::api::Response;
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

        self.read(path)
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

        self.write(path, "description", Some(description), Mode::Replace)
    }

    pub fn input(
        &self,
        year: aoc_core::Year,
        day: aoc_core::Day,
    ) -> anyhow::Result<Option<String>> {
        let path = self
            .project
            .cache_dir()
            .join(self.id.0.to_string())
            .join(year.to_static_str())
            .join(day.to_static_str())
            .join("input");

        self.read(path)
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

        self.write(path, "input", Some(input), Mode::Replace)
    }

    pub fn leaderboard(
        &self,
        year: aoc_core::Year,
    ) -> anyhow::Result<Option<leaderboard::Leaderboard>> {
        let dir = self
            .project
            .cache_dir()
            .join(self.id.0.to_string())
            .join(year.to_static_str());

        let then = self
            .read(dir.join("timestamp"))?
            .and_then(|string| string.parse::<u64>().ok());

        let leaderboard = self
            .read(dir.join("leaderboard"))?
            .and_then(|string| json::from_str::<leaderboard::Leaderboard>(&string).ok());

        let now = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)?
            .as_secs();

        // > Please don't make frequent automated requests to this service - avoid sending
        // > requests more often than once every 15 minutes (900 seconds).
        //
        // From: https://adventofcode.com/$YEAR/leaderboard/private/view/$AOC_ACCOUNT_ID
        match (then, leaderboard) {
            (Some(then), Some(leaderboard)) if now < then || now - then < 15 * 60 => {
                Ok(Some(leaderboard))
            }
            (_, _) => Ok(None),
        }
    }

    pub fn set_leaderboard(
        &self,
        year: aoc_core::Year,
        leaderboard: &leaderboard::Leaderboard,
    ) -> anyhow::Result<()> {
        let path = self
            .project
            .cache_dir()
            .join(self.id.0.to_string())
            .join(year.to_static_str());

        let now = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)?
            .as_secs()
            .to_string();

        let leaderboard = json::to_string(leaderboard)?;

        self.write(&path, "timestamp", Some(&now), Mode::Replace)?;
        self.write(path, "leaderboard", Some(&leaderboard), Mode::Replace)
    }

    pub fn submitted(
        &self,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: aoc_core::Part,
    ) -> anyhow::Result<HashMap<i64, Response>> {
        let path = self
            .project
            .cache_dir()
            .join(self.id.0.to_string())
            .join(year.to_static_str())
            .join(day.to_static_str())
            .join(part.to_static_str())
            .join("submitted");

        match self.read(path)? {
            None => Ok(HashMap::new()),
            Some(submitted) => submitted
                .trim()
                .split('\n')
                .map(|line| {
                    (|| {
                        let (answer, response) = line.split_once(',')?;
                        let answer = answer.parse::<i64>().ok()?;
                        let response = json::from_str(response).ok()?;
                        Some((answer, response))
                    })()
                    .with_context(|| anyhow!("Invalid cache entry: {}", line))
                })
                .collect(),
        }
    }

    pub fn append_submitted(
        &self,
        year: aoc_core::Year,
        day: aoc_core::Day,
        part: aoc_core::Part,
        answer: i64,
        response: Response,
    ) -> anyhow::Result<()> {
        let path = self
            .project
            .cache_dir()
            .join(self.id.0.to_string())
            .join(year.to_static_str())
            .join(day.to_static_str())
            .join(part.to_static_str());

        self.write(
            path,
            "submitted",
            Some(&format!(
                "{},{}\n",
                answer,
                json::to_string(&response).expect("[INTERNAL ERROR]: failed to serialize response"),
            )),
            Mode::Append,
        )
    }

    fn read<P: AsRef<path::Path>>(&self, path: P) -> anyhow::Result<Option<String>> {
        let path = path.as_ref();

        log::info!("Reading from {}", path.display());

        match fs::read_to_string(path) {
            Ok(description) => Ok(Some(description)),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(anyhow::Error::from(error)),
        }
    }

    fn write<P: AsRef<path::Path>>(
        &self,
        path: P,
        file: &'static str,
        data: Option<&str>,
        mode: Mode,
    ) -> anyhow::Result<()> {
        let path = path.as_ref();

        fs::create_dir_all(path)
            .with_context(|| anyhow!("Could not create cache directory: {}", path.display()))?;

        let path = path.join(file);

        log::info!("Writing to {}", path.display());

        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append({
                match mode {
                    Mode::Append => true,
                    Mode::Replace => false,
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
