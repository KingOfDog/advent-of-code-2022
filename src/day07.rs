/* WHAT HAVE I DONE!? */

use std::{
    cell::RefCell,
    collections::{HashMap, LinkedList},
    fmt::Error,
    rc::Rc,
    str::FromStr,
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Parsed = Rc<RefCell<FileTree>>;

#[derive(Debug)]
struct Command {
    name: CommandType,
    args: Vec<String>,
    output: Vec<String>,
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(|s| s.to_string()).collect_vec();
        let input = lines
            .first()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect_vec();
        Ok(Command {
            name: input.first().unwrap().parse()?,
            args: input[1..].to_vec(),
            output: lines[1..].to_vec(),
        })
    }
}

#[derive(Debug)]
enum CommandType {
    ChangeDir,
    ListFiles,
}

impl FromStr for CommandType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cd" => Ok(CommandType::ChangeDir),
            "ls" => Ok(CommandType::ListFiles),
            _ => Err(Error),
        }
    }
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Parsed {
    let input: Vec<Command> = input
        .split("$ ")
        .skip(1)
        .map(|c| c.parse().unwrap())
        .collect_vec();

    let directories = Rc::new(RefCell::new(FileTree::new()));
    let mut current_path = LinkedList::new();
    current_path.push_back(Rc::clone(&directories));

    input.iter().for_each(|command| match command.name {
        CommandType::ChangeDir => {
            let path = command.args.first().unwrap();
            if path == ".." {
                current_path.pop_back();
            } else {
                let children = Rc::new(RefCell::new(FileTree::new()));
                let entry = FileSystemEntry::Directory {
                    children: Rc::clone(&children),
                };
                current_path
                    .back()
                    .unwrap()
                    .as_ref()
                    .borrow_mut()
                    .insert(path.to_owned(), entry);

                current_path.push_back(Rc::clone(&children));
            }
        }

        CommandType::ListFiles => {
            let files = command
                .output
                .iter()
                .filter(|line| !line.starts_with("dir"))
                .map(|line| {
                    let parts = line.split(" ").collect_vec();
                    let name: String = parts[1].parse().unwrap();
                    (
                        name.clone(),
                        FileSystemEntry::File {
                            size: parts[0].parse().unwrap(),
                        },
                    )
                })
                .collect_vec();
            current_path
                .back()
                .unwrap()
                .as_ref()
                .borrow_mut()
                .extend(files)
        }
    });

    directories
}

type FileTree = HashMap<String, FileSystemEntry>;

#[derive(Debug, Clone)]
enum FileSystemEntry {
    Directory { children: Rc<RefCell<FileTree>> },
    File { size: usize },
}

impl FileSystemEntry {
    fn size(&self) -> usize {
        match &self {
            FileSystemEntry::Directory { children, .. } => {
                children.borrow().values().map(|v| v.size()).sum()
            }
            FileSystemEntry::File { size, .. } => *size,
        }
    }

    fn value(&self) -> usize {
        match &self {
            FileSystemEntry::Directory { children, .. } => {
                if self.size() <= 100000 {
                    self.size()
                        + children
                            .borrow()
                            .values()
                            .filter(|item| matches!(item, FileSystemEntry::Directory { .. }))
                            .map(|item| item.value())
                            .sum::<usize>()
                } else {
                    children.borrow().values().map(|item| item.value()).sum()
                }
            }
            FileSystemEntry::File { .. } => 0,
        }
    }
}

#[aoc(day7, part1)]
fn part1(input: &Parsed) -> usize {
    input.borrow().values().map(|item| item.value()).sum()
}

#[aoc(day7, part2)]
fn part2(input: &Parsed) -> usize {
    let binding = input.borrow();
    let root = binding.get("/").unwrap();
    let root_size = root.size();
    let required_space = 30000000 - (70000000 - root_size);

    find_dir_with_min(input.to_owned(), required_space)
        .unwrap()
        .size()
}

fn find_dir_with_min(file_tree: Rc<RefCell<FileTree>>, min_size: usize) -> Option<FileSystemEntry> {
    file_tree
        .borrow()
        .values()
        .filter_map(|d| match d {
            FileSystemEntry::Directory { children, .. } => {
                let size = d.size();
                if size < min_size {
                    return None;
                }
                let min_subdir =
                    find_dir_with_min(children.to_owned(), min_size).unwrap_or(d.to_owned());
                if min_subdir.size() < size {
                    Some(min_subdir)
                } else {
                    Some(d.to_owned())
                }
            }
            _ => None,
        })
        .min_by_key(|item| item.size())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 95437);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 24933642);
    }
}
