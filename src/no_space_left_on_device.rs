use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

#[derive(Debug)]
pub enum CommandType {
    Cd,
    Ls,
}

#[derive(Debug)]
pub enum ItemType {
    File { size: i32 },
    Folder { name: String },
}

#[derive(Debug)]
pub enum IteratorItem {
    Command { command_type: CommandType, param: String },
    Info { item_type: ItemType },
}

pub struct CommandLineIterator {
    buf_reader: BufReader<File>,
}

impl Iterator for CommandLineIterator {
    type Item = IteratorItem;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::new();
        match self.buf_reader.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                let buf = buf.trim_end();
                if buf.starts_with('$') {
                    Some(parse_command(&buf[2..]))
                } else {
                    Some(parse_info(buf))
                }
            }
            Err(_e) => panic!(),
        }
    }
}

pub fn read_input(file_name: &str) -> CommandLineIterator {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    CommandLineIterator {
        buf_reader: reader,
    }
}

fn parse_info(line: &str) -> IteratorItem {
    let result = line.split(' ').collect::<Vec<_>>();

    match result[0].parse::<i32>() {
        Ok(size) => IteratorItem::Info { item_type: ItemType::File { size } },
        Err(_) => IteratorItem::Info { item_type: ItemType::Folder { name: String::from(result[1]) } },
    }
}

fn parse_command(line: &str) -> IteratorItem {
    let result = line.split(' ').collect::<Vec<_>>();
    let command_type: CommandType;
    let param: String;

    if result[0] == "ls" {
        command_type = CommandType::Ls;
        param = "".to_string();
    } else {
        command_type = CommandType::Cd;
        param = result[1].to_string();
    }

    IteratorItem::Command { command_type, param }
}

#[derive(Debug)]
pub struct TreeNode {
    value: ItemType,
    children: Vec<Rc<RefCell<TreeNode>>>,
    parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new() -> TreeNode {
        TreeNode {
            value: ItemType::Folder { name: r"\".to_string() },
            children: vec![],
            parent: None,
        }
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
        self.children.push(new_node);
    }
}

pub fn print(node: Rc<RefCell<TreeNode>>) {
    match &node.borrow().value {
        ItemType::File { size } => println!("file (size={size})"),
        ItemType::Folder { name } => {
            println!(">{name}");
            print_children(Rc::clone(&node), 2);
        }
    }
}

pub fn print_children(node: Rc<RefCell<TreeNode>>, indent: u32) {
    for child in RefCell::borrow(&node).children.iter() {
        let item_type: &ItemType = &RefCell::borrow(child).value;
        let ind = " ".repeat(indent as usize);

        if let ItemType::Folder { name } = &item_type {
            let size = calculate_item_size(Rc::clone(child));
            println!("{ind}>{name}  (size: {size})");

            print_children(Rc::clone(child), indent + 2);
        }
    }

    for child in RefCell::borrow(&node).children.iter() {
        let item_type: &ItemType = &RefCell::borrow(child).value;
        let ind = " ".repeat(indent as usize);

        if let ItemType::File { size } = &item_type {
            println!("{ind}file (size={size})");

            print_children(Rc::clone(child), indent + 2);
        }
    }
}

fn find_child_by_name(root: Rc<RefCell<TreeNode>>, folder_name: &str) -> Option<Rc<RefCell<TreeNode>>> {
    for node in RefCell::borrow(&root).children.iter() {
        let item_type: &ItemType = &RefCell::borrow(node).value;

        match item_type {
            ItemType::File { .. } => continue,
            ItemType::Folder { name } => {
                if *name == folder_name {
                    return Some(Rc::clone(node));
                }
                continue;
            }
        }
    }

    None
}

fn calculate_item_size(node: Rc<RefCell<TreeNode>>) -> i32 {
    let mut sum = 0;

    for node in RefCell::borrow(&node).children.iter() {
        let item_type: &ItemType = &RefCell::borrow(node).value;

        match item_type {
            ItemType::File { size } => {
                sum += size;
                continue;
            }
            ItemType::Folder { .. } => {
                sum += calculate_item_size(Rc::clone(node));
            }
        }
    }

    sum
}

fn calculate_all_sizes(node: Rc<RefCell<TreeNode>>) -> i32 {
    let mut total = 0;

    for node in RefCell::borrow(&node).children.iter() {
        let item_type: &ItemType = &RefCell::borrow(node).value;

        match item_type {
            ItemType::File { .. } => continue,
            ItemType::Folder { .. } => {
                let size = calculate_item_size(Rc::clone(node));

                if size < 100_000 {
                    total += size;
                }

                total += calculate_all_sizes(Rc::clone(node));
            }
        }
    }

    total
}

fn init_tree(file_name: &str) -> Rc<RefCell<TreeNode>> {
    let root = Rc::new(RefCell::new(TreeNode::new()));
    let mut current_folder = Rc::clone(&root);

    for item in read_input(file_name).skip(1) {
        match item {
            IteratorItem::Command { command_type, param } => {
                match command_type {
                    CommandType::Cd => {
                        if param == ".." {
                            let current_clone = Rc::clone(&current_folder);
                            current_folder = Rc::clone(RefCell::borrow(&current_clone).parent.as_ref().unwrap_or(&root));
                        } else {
                            current_folder = find_child_by_name(Rc::clone(&current_folder), &param).unwrap();
                        }
                    }
                    CommandType::Ls => {}
                }
            }
            IteratorItem::Info { item_type } => {
                match item_type {
                    ItemType::Folder { name } => {
                        let node = TreeNode {
                            value: ItemType::Folder { name },
                            parent: Some(Rc::clone(&current_folder)),
                            children: vec![],
                        };

                        RefCell::borrow_mut(&current_folder).add_child(Rc::new(RefCell::new(node)));
                    }
                    ItemType::File { size } => {
                        let node = TreeNode {
                            value: ItemType::File { size },
                            parent: Some(Rc::clone(&current_folder)),
                            children: vec![],
                        };

                        RefCell::borrow_mut(&current_folder).add_child(Rc::new(RefCell::new(node)));
                    }
                }
            }
        }
    }

    root
}

fn get_all_dir_sizes(node: Rc<RefCell<TreeNode>>, dir_sizes: &mut Vec<i32>) {
    for child in RefCell::borrow(&node).children.iter() {
        match &RefCell::borrow(child).value {
            ItemType::File { .. } => continue,
            ItemType::Folder { .. } => {
                dir_sizes.push(calculate_item_size(Rc::clone(child)));

                get_all_dir_sizes(Rc::clone(child), dir_sizes);
            }
        }
    }
}

pub fn no_space_left_on_device_part_1(file_name: &str) -> i32 {
    let root = init_tree(file_name);
    print(Rc::clone(&root));

    calculate_all_sizes(Rc::clone(&root))
}

pub fn no_space_left_on_device_part_2(file_name: &str) -> i32 {
    let max_space = 70_000_000;
    let required_space = 30_000_000;
    let root = init_tree(file_name);

    let missing_space = calculate_item_size(Rc::clone(&root)) - (max_space - required_space);
    let mut dir_sizes: Vec<i32> = Vec::new();

    get_all_dir_sizes(Rc::clone(&root), &mut dir_sizes);

    let mut closest_size = max_space;
    for dir_size in dir_sizes.iter() {
        let diff = dir_size - missing_space;
        if diff > 0 && closest_size - missing_space > dir_size - missing_space {
            closest_size = *dir_size;
        }
    }

    closest_size
}

