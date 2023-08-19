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
    fn calculate_size(&mut self) -> i32 {
        for subdir in self.subdirs.values() {
            self.size += subdir.borrow_mut().calculate_size();
        }
        self.size
    }

    fn get_subdir_under_100000(&self, vec: &mut Vec<i32>) {
        for subdir in self.subdirs.values().map(|x| x.borrow()) {
            subdir.get_subdir_under_100000(vec);
            if subdir.size < 100000 {
                vec.push(subdir.size);
            }
        }
    }
}

struct DirTree {
    root: Rc<RefCell<Dir>>,
    current_dir: Rc<RefCell<Dir>>,
}

impl DirTree {
    pub fn new() -> Self {
        let root = Rc::new(RefCell::new(Dir {
            parent: Weak::new(),
            subdirs: HashMap::new(),
            size: 0,
        }));

        DirTree {
            root: root.clone(),
            current_dir: root.clone(),
        }
    }

    fn goto_dir(&mut self, dir_name: &str) {
        self.current_dir = match dir_name {
            ".." => self
                .current_dir
                .borrow()
                .parent
                .upgrade()
                .expect("parent not found")
                .clone(),
            "/" => self.root.clone(),
            _ => self
                .current_dir
                .borrow()
                .subdirs
                .get(dir_name)
                .expect("dir not found")
                .clone(),
        }
    }

    fn add_subdir(&self, new_dir_name: &str) {
        self.current_dir.borrow_mut().subdirs.insert(
            new_dir_name.to_string(),
            Rc::new(RefCell::new(Dir {
                parent: Rc::downgrade(&self.current_dir),
                subdirs: HashMap::new(),
                size: 0,
            })),
        );
    }

    fn add_size(&self, size: i32) {
        self.current_dir.borrow_mut().size += size;
    }

    fn calculate_sizes(&self) {
        self.root.borrow_mut().calculate_size();
    }

    fn sum_of_dir_sizes_under10000(&self) -> i32 {
        let mut vec = Vec::new();
        self.root.borrow().get_subdir_under_100000(&mut vec);
        vec.iter().sum()
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

    dir_tree.calculate_sizes();
    let total = dir_tree.sum_of_dir_sizes_under10000();

    println!("{}", total);
}
