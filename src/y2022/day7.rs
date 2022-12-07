use std::fmt::Display;

use id_tree::{InsertBehavior, Node, NodeId, Tree};

use crate::Solution;

pub struct Day7;

impl Solution for Day7 {
    fn q1(&self, data: &str) -> String {
        const SIZE_LIMIT: u64 = 100000;

        let commands = parse1(data);
        let tree: Tree<TreeNode> = commands.collect();

        tree.traverse_pre_order(tree.root_node_id().unwrap())
            .unwrap()
            .filter(|node| node.data().size <= SIZE_LIMIT)
            .filter(|node| !node.children().is_empty())
            .map(|node| node.data().size)
            .sum::<u64>()
            .to_string()
    }

    fn q2(&self, data: &str) -> String {
        const DISK_CAPACITY: u64 = 70_000_000;
        const WANTED_SPACE: u64 = 30_000_000;

        let commands = parse1(data);
        let tree: Tree<TreeNode> = commands.collect();

        let unused_space =
            DISK_CAPACITY - tree.get(tree.root_node_id().unwrap()).unwrap().data().size;
        let space_to_clear = WANTED_SPACE - unused_space;

        tree.traverse_pre_order(tree.root_node_id().unwrap())
            .unwrap()
            .filter(|node| node.data().size >= space_to_clear)
            .filter(|node| !node.children().is_empty())
            .map(|node| node.data().size)
            .min()
            .unwrap()
            .to_string()
    }
}

fn parse1(data: &str) -> impl Iterator<Item = Command<'_>> + '_ {
    data.split("$ ").flat_map(|s| Command::parse(s).ok())
}

enum Command<'a> {
    CdRoot,
    CdAbove,
    CdInto(&'a str),
    Ls(Vec<FileLs<'a>>),
}

impl Display for Command<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::CdRoot => write!(f, "$ cd /"),
            Command::CdAbove => write!(f, "$ cd .."),
            Command::CdInto(dir) => write!(f, "$ cd {dir}"),
            Command::Ls(files) => {
                write!(f, "$ ls")?;
                for file in files {
                    write!(f, "\n{file}")?;
                }
                Ok(())
            }
        }
    }
}

enum FileLs<'a> {
    Dir(&'a str),
    File(u32, &'a str),
}

impl Display for FileLs<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileLs::Dir(dir) => write!(f, "dir {dir}"),
            FileLs::File(size, file) => write!(f, "{size} {file}"),
        }
    }
}

impl<'a> Command<'a> {
    fn parse(s: &'a str) -> Result<Command<'a>, ()> {
        let mut lines = s.trim().split('\n');

        let line1 = lines.next().ok_or(())?.trim();
        Ok(match line1 {
            "ls" => parse_ls(lines)?,
            "cd /" => Self::CdRoot,
            "cd .." => Self::CdAbove,

            // cd xxx
            _ => Self::CdInto(line1.split_once(' ').ok_or(())?.1),
        })
    }
}

fn parse_ls<'a>(lines: impl Iterator<Item = &'a str>) -> Result<Command<'a>, ()> {
    Ok(Command::Ls(
        lines
            .flat_map(|line| line.trim().split_once(' '))
            .flat_map(|(a, b)| -> Result<FileLs, ()> {
                Ok(if a == "dir" {
                    FileLs::Dir(b)
                } else {
                    FileLs::File(a.parse().map_err(|_| ())?, b)
                })
            })
            .collect(),
    ))
}

#[derive(Debug)]
struct TreeNode {
    name: String,
    size: u64,
}

impl<'a> FromIterator<Command<'a>> for Tree<TreeNode> {
    fn from_iter<T: IntoIterator<Item = Command<'a>>>(commands: T) -> Self {
        let mut tree = Tree::new();
        let root_id = tree
            .insert(
                Node::new(TreeNode {
                    name: "/".to_string(),
                    size: 0,
                }),
                InsertBehavior::AsRoot,
            )
            .unwrap();

        let mut cur_node = root_id.clone();
        for cmd in commands {
            match cmd {
                Command::CdRoot => cur_node = root_id.clone(),
                Command::CdAbove => {
                    cur_node = tree
                        .get(&cur_node)
                        .unwrap()
                        .parent()
                        .unwrap_or(&root_id)
                        .clone()
                }
                Command::CdInto(dir) => cur_node = get_or_create(&mut tree, &cur_node, dir, 0),
                Command::Ls(files) => {
                    for file in files {
                        match file {
                            FileLs::Dir(dir) => {
                                get_or_create(&mut tree, &cur_node, dir, 0);
                            }
                            FileLs::File(size, name) => {
                                get_or_create(&mut tree, &cur_node, name, size as _);
                            }
                        }
                    }
                }
            }
        }

        update_sizes(&mut tree);
        tree
    }
}

fn update_sizes(tree: &mut Tree<TreeNode>) {
    for node_id in tree
        .traverse_post_order_ids(tree.root_node_id().unwrap())
        .unwrap()
    {
        let node_size = tree
            .children(&node_id)
            .unwrap()
            .map(|child| child.data().size)
            .sum();
        if node_size > 0 {
            let node = tree.get_mut(&node_id).unwrap();
            node.data_mut().size = node_size;
        }
    }
}

fn get_or_create(tree: &mut Tree<TreeNode>, cur_node: &NodeId, name: &str, size: u64) -> NodeId {
    tree.children_ids(cur_node)
        .unwrap()
        .find(|node_id| tree.get(node_id).unwrap().data().name == name)
        .cloned()
        .unwrap_or_else(|| {
            tree.insert(
                Node::new(TreeNode {
                    name: name.to_string(),
                    size,
                }),
                InsertBehavior::UnderNode(cur_node),
            )
            .unwrap()
        })
}

#[cfg(test)]
mod test {
    use crate::Solution;

    const DATA: &str = "
        $ cd /
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
        7214296 k
    ";

    #[test]
    fn example1() {
        assert_eq!("95437", super::Day7 {}.q1(DATA));
    }

    #[test]
    fn example2() {
        assert_eq!("24933642", super::Day7 {}.q2(DATA));
    }
}
