use crate::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use itertools::Itertools;

#[derive(FromStr, PartialEq, Debug)]
enum Command {
    #[display("$ cd {0}")]
    ChangeDir(String),
    #[display("$ ls")]
    Ls,
}

#[derive(FromStr, PartialEq, Debug)]
enum Listing {
    #[display("dir {0}")]
    Directory(String),
    #[display("{0} {1}")]
    File(usize, String),
}

// #[derive(PartialEq, Debug)]
// struct Entry {
//     children: Vec<Entry>,
//     path: String
// }
//
// impl Entry {
//     fn new(path: &str) -> Self {
//         Entry {
//             children: HashMap::new(),
//             path: path.to_string(),
//         }
//     }
// }

#[derive(Debug)]
pub struct Filesystem {
    folders: HashMap<PathBuf, Vec<usize>>,
}

impl Filesystem {
    fn get_folder_sizes(&self) -> HashMap<String, usize> {
        
        let mut scores = HashMap::new();
        
        for (path1, scores1) in self.folders.clone() { 
            for (path2, scores2) in self.folders.clone() {
                if path2.starts_with(&path1) {
                    scores.entry(path1.to_string_lossy().to_string()).and_modify(|y| *y += scores2.iter().sum::<usize>()).or_insert(scores2.iter().sum());
                }
            }
        };
        
        scores
        
    
    }
}

impl FromStr for Filesystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut folders = HashMap::new();
        let mut sizes = vec![];
        let mut current_folder = PathBuf::new();
        let mut scanning = false;
        for line in s.lines().skip(1) {
            if let Ok(command) = line.parse::<Command>() {
                // println!("{:?}", command);
                match command {
                    Command::ChangeDir(path) => {
                        if !folders.contains_key(&current_folder) {
                            folders.insert(
                                current_folder.clone(),
                                sizes.clone(),
                            );
                        }
                        // println!("folder = {:?}, folders = {:?}", current_folder, &folders);
                        sizes.clear();
                        scanning = false;
                        if path == ".." {
                            
                            current_folder.pop();
                            // println!("popping.. {:?}", current_folder);
                        } else {
                            current_folder.push(&PathBuf::from(path));
                        }
                    }
                    Command::Ls => scanning = true,
                }
            } else if let Ok(listing) = line.parse::<Listing>() {
                // println!("{:?}", listing);

                match listing {
                    Listing::Directory(_) => {}
                    Listing::File(size, _) => sizes.push(size),
                }
            } else {
                unreachable!()
            }
        }
        if !folders.contains_key(&current_folder) {
            folders.insert(
                current_folder,
                sizes.clone(),
            );
        }
        Ok(Filesystem { folders: folders })
    }
}

/// https://adventofcode.com/2022/day/7
#[aoc_generator(day7)]
pub fn generator(input: &str) -> Filesystem {
    input.parse().unwrap()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Filesystem) -> usize {
    input.get_folder_sizes().iter().map(|(a,b)| *b).filter(|x| *x  < 100000).sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Filesystem) -> i64 {
    let total: i64 = 70000000;
    let needed: i64 = 30000000;
    let used: i64 = *input.get_folder_sizes().get("").unwrap() as i64;
    let unused = total - used;
    let need_to_free = needed - unused;
    let (_, size) = input.get_folder_sizes().iter().map(|(a,b)| (a.clone(), *b as i64)).filter(|(a,b)| *b > need_to_free ).sorted_by_key(|(a,b)| *b).next().unwrap();
    size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "$ cd /
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
7214296 k";
        assert_eq!(95437, solve_part1(&generator(provided)));
        assert_eq!(
            1444896,
            solve_part1(&generator(include_str!("../input/2022/day7.txt")))
        );
    }

    #[test]
    fn it_works_part2() {
    let provided = "$ cd /
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
7214296 k";
    assert_eq!(24933642, solve_part2(&generator(provided)));
        assert_eq!(
            404395,
            solve_part2(&generator(include_str!("../input/2022/day7.txt")))
        );
    }
}
