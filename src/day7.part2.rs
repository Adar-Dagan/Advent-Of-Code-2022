use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;

struct Dir {
    parent: Weak<RefCell<Dir>>,
    subdirs: HashMap<String, Rc<RefCell<Dir>>>,
    size: i32,
}

impl Dir {
    fn update_size(&mut self, size: i32) {
        self.size += size;
        if let Some(parent) = self.parent.upgrade() {
            parent.borrow_mut().update_size(size);
        }
    }

    fn get_subdir_under_100000(&self, vec: &mut Vec<i32>) {
        for subdir in self.subdirs.values().map(|x| x.borrow()) {
            subdir.get_subdir_under_100000(vec);
            if subdir.size < 100000 {
                vec.push(subdir.size);
            }
        }
    }

    fn smallest_over_size(&self, size: i32) -> i32 {
        let mut smallest = i32::MAX;
        for subdir in self.subdirs.values().map(|x| x.borrow()) {
            let subdir_smallest = subdir.smallest_over_size(size);
            if subdir_smallest < smallest {
                smallest = subdir_smallest;
            }
        }
        if self.size > size && self.size < smallest {
            smallest = self.size;
        }
        smallest
    }
}

struct DirTree {
    root: Rc<RefCell<Dir>>,
    current_dir: Weak<RefCell<Dir>>,
}

impl DirTree {
    pub fn new() -> Self {
        let root = DirTree::new_node(Weak::new());

        DirTree {
            root: root.clone(),
            current_dir: Rc::downgrade(&root),
        }
    }

    fn new_node(parent: Weak<RefCell<Dir>>) -> Rc<RefCell<Dir>> {
        Rc::new(RefCell::new(Dir {
            parent,
            subdirs: HashMap::new(),
            size: 0,
        }))
    }

    fn get_current(&self) -> Rc<RefCell<Dir>> {
        self.current_dir.upgrade().unwrap()
    }

    fn goto_dir(&mut self, dir_name: &str) {
        self.current_dir = match dir_name {
            ".." => self.get_current().borrow().parent.clone(),
            "/" => Rc::downgrade(&self.root),
            _ => Rc::downgrade(
                self.get_current()
                    .borrow()
                    .subdirs
                    .get(dir_name)
                    .expect("dir not found"),
            ),
        }
    }

    fn add_subdir(&self, new_dir_name: &str) {
        self.get_current().borrow_mut().subdirs.insert(
            new_dir_name.to_string(),
            DirTree::new_node(self.current_dir.clone()),
        );
    }

    fn add_size(&self, size: i32) {
        self.get_current().borrow_mut().update_size(size)
    }

    fn sum_of_dir_sizes_under_100000(&self) -> i32 {
        let mut vec = Vec::new();
        self.root.borrow().get_subdir_under_100000(&mut vec);
        vec.iter().sum()
    }

    fn smallest_over_size(&self, size: i32) -> i32 {
        self.root.borrow().smallest_over_size(size)
    }
}

fn main() {
    const CONTENTS: &str = include_str!("day7.input");

    let mut dir_tree = DirTree::new();

    for terminal_line in CONTENTS.lines() {
        let mut words = terminal_line.split_whitespace();
        let first_word = words.next().unwrap();
        let second_word = words.next().unwrap();

        match first_word {
            "$" if second_word.eq("cd") => {
                let dir_name = words.next().unwrap();
                dir_tree.goto_dir(dir_name);
            }
            "dir" => {
                dir_tree.add_subdir(second_word);
            }
            size if size.parse::<i32>().is_ok() => {
                dir_tree.add_size(size.parse::<i32>().unwrap());
            }
            _ => {}
        }
    }

    let total = dir_tree.smallest_over_size(30000000 - (70000000 - dir_tree.root.borrow().size));

    println!("{}", total);
}
