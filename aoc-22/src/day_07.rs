use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use aoc::*;

#[derive(Clone, Debug)]
pub struct NoSpaceLeftOnDevice(Vec<Command>);

#[derive(Clone, Debug)]
enum Command {
    Cd(String),
    Ls(Vec<Entry>),
}

#[derive(Clone, Debug)]
enum Entry {
    Directory(String),
    File(i64, String),
}

impl Fro for NoSpaceLeftOnDevice {
    fn fro(input: &str) -> Self {
        let mut iter = input.trim().lines().peekable();
        let mut commands = Vec::new();

        while let Some(next) = iter.next() {
            let command = next.strip_prefix("$ ").unwrap();

            if let Some(directory) = command.strip_prefix("cd ") {
                commands.push(Command::Cd(String::from(directory)));
                continue;
            }

            assert_eq!(command, "ls");
            let mut entries = Vec::new();

            while let Some(entry) = iter.peek() {
                if entry.starts_with("$ ") {
                    break;
                }

                if let Some(directory) = entry.strip_prefix("dir ") {
                    entries.push(Entry::Directory(String::from(directory)));
                } else {
                    let (size, file) = entry.split_once(' ').unwrap();
                    entries.push(Entry::File(i64::fro(size), String::from(file)));
                }

                iter.next();
            }

            commands.push(Command::Ls(entries));
        }

        Self(commands)
    }
}

impl Solution for NoSpaceLeftOnDevice {
    fn one(self) -> i64 {
        self.flatten()
            .values()
            .copied()
            .filter(|size| *size <= 100000)
            .sum()
    }

    fn two(self) -> i64 {
        let sizes = self.flatten();
        let total = sizes[Path::new("/")];

        sizes
            .values()
            .copied()
            .filter(|size| 70000000 - (total - *size) >= 30000000)
            .min()
            .unwrap()
    }
}

impl NoSpaceLeftOnDevice {
    fn flatten(self) -> HashMap<PathBuf, i64> {
        let mut root = Path::new("/").to_path_buf();
        let mut tree = HashMap::new();

        for command in self.0 {
            match command {
                Command::Cd(path) if path == ".." => {
                    root.pop();
                }
                Command::Cd(path) => root.push(path),
                Command::Ls(entries) => {
                    tree.insert(root.clone(), entries);
                }
            }
        }

        let mut sizes = HashMap::new();
        root.push("/");
        Self::recurse(&mut root, &tree, &mut sizes);
        sizes
    }

    // Recursively fill out directory sizes
    fn recurse(
        root: &mut PathBuf,
        tree: &HashMap<PathBuf, Vec<Entry>>,
        sizes: &mut HashMap<PathBuf, i64>,
    ) -> i64 {
        let size = tree[root]
            .iter()
            .map(|entry| match entry {
                Entry::File(size, _) => *size,
                Entry::Directory(dir) => {
                    root.push(dir);
                    let size = Self::recurse(root, tree, sizes);
                    root.pop();
                    size
                }
            })
            .sum();

        sizes.insert(root.clone(), size);
        size
    }
}
