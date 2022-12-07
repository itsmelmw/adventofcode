// https://adventofcode.com/2022/day/7

struct FileSystem {
    files: Vec<Node>,
}

impl FileSystem {
    fn new() -> Self {
        return FileSystem {
            files: vec![Node::new(0, None)],
        };
    }

    fn add_dir(&mut self, parent: usize) -> usize {
        let index = self.files.len();
        let node = Node::new(index, Some(parent));

        self.files[parent].dirs.push(index);
        self.files.push(node);

        return index;
    }

    fn add_file(&mut self, dir: usize, size: usize) {
        let mut node = &mut self.files[dir];
        node.size += size;

        if let Some(index) = node.parent {
            self.add_file(index, size);
        }
    }

    fn get_parent(&mut self, node: usize) -> usize {
        return self.files[self.files[node].parent.unwrap()].index;
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
        return Node {
            index: index,
            size: 0,
            dirs: Vec::new(),
            parent: parent,
        };
    }
}

fn parse(input: &str) -> FileSystem {
    let mut fs = FileSystem::new();
    let mut cwd = 0;
    for line in input.split("\n") {
        match line.split(" ").collect::<Vec<&str>>()[..] {
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
    return fs;
}

fn solve1(parsed: &FileSystem) -> String {
    return parsed
        .files
        .iter()
        .map(|dir| if dir.size <= 100000 { dir.size } else { 0 })
        .sum::<usize>()
        .to_string();
}

fn solve2(parsed: &FileSystem) -> String {
    let space = 30000000 - (70000000 - parsed.files[0].size);
    return parsed
        .files
        .iter()
        .map(|dir| dir.size)
        .filter(|size| size >= &space)
        .min()
        .unwrap()
        .to_string();
}

pub fn solve(input: &str) -> (String, String) {
    let parsed = parse(input);
    return (solve1(&parsed), solve2(&parsed));
}
