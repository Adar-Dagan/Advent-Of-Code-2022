//Tried to implement with a more functional approach

use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::HashMap;
use std::rc::Rc;

struct Dir {
    parent: Option<Rc<RefCell<Dir>>>,
    subdirs: HashMap<String, Rc<RefCell<Dir>>>,
    size: i32,
}

fn main() {
    const CONTENTS: &str = include_str!("day7.input");

    let root = Rc::new(RefCell::new(Dir {
        parent: None,
        subdirs: HashMap::new(),
        size: 0,
    }));

    let mut current_dir = root.clone();

    for terminal_line in CONTENTS.lines() {
        let mut words = terminal_line.split_whitespace();
        let first_word = words.next().unwrap();
        let second_word = words.next().unwrap();

        match first_word {
            "$" if second_word.eq("cd") => {
                let dir_name = words.next().unwrap();
                current_dir = get_dir(current_dir.borrow_mut(), dir_name, root.clone());
            }
            "dir" => {
                add_subdir(
                    &mut current_dir.borrow_mut().subdirs,
                    &current_dir,
                    second_word,
                );
            }
            size if size.parse::<i32>().is_ok() => {
                current_dir.borrow_mut().size += size.parse::<i32>().unwrap();
            }
            _ => {}
        }
    }

    calculate_dirs_total_sizes(root.borrow_mut());
    let total = sum_of_dir_sizes_under10000(root.borrow_mut());

    println!("{}", total);
}

fn get_dir(current: RefMut<'_, Dir>, dir_name: &str, root: Rc<RefCell<Dir>>) -> Rc<RefCell<Dir>> {
    if dir_name.eq("..") {
        return current.parent.as_ref().expect("parent not found").clone();
    } else if dir_name.eq("/") {
        return root.clone();
    }

    current
        .subdirs
        .get(dir_name)
        .expect("dir not found")
        .clone()
}

fn add_subdir(
    subdirs: &mut HashMap<String, Rc<RefCell<Dir>>>,
    parent: &Rc<RefCell<Dir>>,
    second_word: &str,
) {
    subdirs.insert(
        second_word.to_string(),
        Rc::new(RefCell::new(Dir {
            parent: Some(parent.clone()),
            subdirs: HashMap::new(),
            size: 0,
        })),
    );
}

fn calculate_dirs_total_sizes(mut dir: RefMut<'_, Dir>) -> i32 {
    let mut total = dir.size;
    for subdir in dir.subdirs.values() {
        total += calculate_dirs_total_sizes(subdir.borrow_mut());
    }
    dir.size = total;
    total
}

fn sum_of_dir_sizes_under10000(borrow_mut: RefMut<'_, Dir>) -> i32 {
    let mut total = if borrow_mut.size < 100000 {
        borrow_mut.size
    } else {
        0
    };
    for subdir in borrow_mut.subdirs.values() {
        total += sum_of_dir_sizes_under10000(subdir.borrow_mut());
    }
    total
}
