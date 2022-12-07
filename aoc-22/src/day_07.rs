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
    Dir(String),
    File(i64, String),
}

impl Fro for NoSpaceLeftOnDevice {
    fn fro(input: &str) -> Self {
        let mut iter = input.trim().lines().peekable();

        let mut commands = Vec::new();
        while let Some(next) = iter.next() {
            if next.starts_with("$ ") {
                let next = next.trim_start_matches("$ ");

                if next.starts_with("cd ") {
                    let directory = next.trim_start_matches("cd ").trim().to_owned();
                    commands.push(Command::Cd(directory));
                } else if next.starts_with("ls") {
                    let mut ls = Vec::new();

                    while let Some(a) = iter.peek() {
                        if a.starts_with("$ ") {
                            break;
                        }

                        if a.starts_with("dir ") {
                            ls.push(Entry::Dir(a.trim_start_matches("dir ").trim().to_owned()));
                        } else {
                            let (a, b) = a.split_once(' ').unwrap();
                            ls.push(Entry::File(i64::fro(a), b.to_owned()));
                        }

                        iter.next();
                    }

                    commands.push(Command::Ls(ls));
                }
            }
        }

        Self(commands)
    }
}

impl Solution for NoSpaceLeftOnDevice {
    fn one(self) -> i64 {
        let mut prefix = Path::new("/").to_path_buf();
        let mut tree = HashMap::new();
        let mut sizes = HashMap::new();

        for command in self.0 {
            match &command {
                Command::Cd(path) if path == ".." => {
                    prefix.pop();
                }
                Command::Cd(path) if path == "/" => prefix = PathBuf::from("/"),
                Command::Cd(path) => prefix.push(path),
                Command::Ls(entries) => {
                    tree.entry(prefix.clone())
                        .or_insert_with(Vec::new)
                        .extend(entries.iter().cloned());
                }
            }
        }

        recurse(&tree, &mut Path::new("/").to_path_buf(), &mut sizes);
        sizes
            .iter()
            .filter_map(|(_, size)| if *size <= 100000 { Some(*size) } else { None })
            .sum()
    }

    fn two(self) -> i64 {
        let mut prefix = Path::new("/").to_path_buf();
        let mut tree = HashMap::new();
        let mut sizes = HashMap::new();

        for command in self.0 {
            match &command {
                Command::Cd(path) if path == ".." => {
                    prefix.pop();
                }
                Command::Cd(path) if path == "/" => prefix = PathBuf::from("/"),
                Command::Cd(path) => prefix.push(path),
                Command::Ls(entries) => {
                    tree.entry(prefix.clone())
                        .or_insert_with(Vec::new)
                        .extend(entries.iter().cloned());
                }
            }
        }

        recurse(&tree, &mut Path::new("/").to_path_buf(), &mut sizes);

        let total = sizes[Path::new("/")];

        sizes
            .values()
            .copied()
            .filter(|size| 70000000 - (total - *size) >= 30000000)
            .min()
            .unwrap()
    }
}

fn recurse(
    tree: &HashMap<PathBuf, Vec<Entry>>,
    root: &mut PathBuf,
    sizes: &mut HashMap<PathBuf, i64>,
) -> i64 {
    let sum = tree
        .get(root)
        .unwrap()
        .iter()
        .map(|entry| match entry {
            Entry::File(size, _) => *size,
            Entry::Dir(dir) => {
                root.push(dir);
                let size = recurse(tree, root, sizes);
                root.pop();
                size
            }
        })
        .sum();

    sizes.insert(root.clone(), sum);
    sum
}
