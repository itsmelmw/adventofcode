// https://adventofcode.com/2022/day/7

use aoc_utils::solutions::{InputDir, Part, Solution};

struct FileSystem {
    files: Vec<Node>,
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            files: vec![Node::new(0, None)],
        }
    }

    fn add_dir(&mut self, parent: usize) -> usize {
        let index = self.files.len();
        let node = Node::new(index, Some(parent));

        self.files[parent].dirs.push(index);
        self.files.push(node);

        index
    }

    fn add_file(&mut self, dir: usize, size: usize) {
        let node = &mut self.files[dir];
        node.size += size;

        if let Some(index) = node.parent {
            self.add_file(index, size);
        }
    }

    fn get_parent(&mut self, node: usize) -> usize {
        self.files[self.files[node].parent.unwrap()].index
    }
}

struct Node {
    index: usize,
    size: usize,
    dirs: Vec<usize>,
    parent: Option<usize>,
}

impl Node {
    fn new(index: usize, parent: Option<usize>) -> Self {
        Node {
            index,
            size: 0,
            dirs: Vec::new(),
            parent,
        }
    }
}

pub struct Day07 {
    fs: FileSystem,
}

impl<'i> Solution<'i> for Day07 {
    fn title(&self) -> &str {
        "No Space Left On Device"
    }
    fn parse(input: &'i str) -> Self {
        let mut fs = FileSystem::new();
        let mut cwd = 0;
        for line in input.split('\n') {
            match line.split(' ').collect::<Vec<&str>>()[..] {
                // Commands
                ["$", "cd", "/"] => (),
                ["$", "cd", ".."] => cwd = fs.get_parent(cwd),
                ["$", "cd", _] => cwd = fs.add_dir(cwd), //fs.add(cwd, None),
                ["$", "ls"] => (),
                // Nodes
                ["dir", _] => (),
                [size, _] => fs.add_file(cwd, size.parse::<usize>().unwrap()),
                _ => (),
            }
        }
        Self { fs }
    }
    fn solve_part_1(&self) -> String {
        self.fs
            .files
            .iter()
            .map(|dir| if dir.size <= 100000 { dir.size } else { 0 })
            .sum::<usize>()
            .to_string()
    }
    fn solve_part_2(&self) -> String {
        let space = 30000000 - (70000000 - self.fs.files[0].size);
        self.fs
            .files
            .iter()
            .map(|dir| dir.size)
            .filter(|size| size >= &space)
            .min()
            .unwrap()
            .to_string()
    }
    fn answer(&self, input: &InputDir, part: &Part) -> Option<&str> {
        match (input.name().as_str(), part) {
            ("Example", Part::One) => Some("95437"),
            ("Example", Part::Two) => Some("24933642"),
            ("Puzzle", Part::One) => Some("1118405"),
            ("Puzzle", Part::Two) => Some("12545514"),
            _ => unreachable!(),
        }
    }
}
