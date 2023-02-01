use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Display;
use std::fs::read_to_string;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum CaveType {
    Start,
    End,
    Small,
    Big,
}

#[derive(Debug, Clone)]
pub struct Cave {
    id: String,
    ctype: CaveType,
    connections: Vec<Rc<RefCell<Cave>>>,
}

impl PartialEq for Cave {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Cave {
    fn new(id: &str) -> Rc<RefCell<Cave>> {
        Rc::new(RefCell::new(Cave {
            id: id.to_string(),
            ctype: Cave::get_cave_type(id),
            connections: Vec::new(),
        }))
    }

    fn get_cave_type(id: &str) -> CaveType {
        if id == "start" {
            CaveType::Start
        } else if id == "end" {
            CaveType::End
        } else if id.chars().all(char::is_uppercase) {
            CaveType::Big
        } else {
            CaveType::Small
        }
    }

    fn print(cave_system: Rc<RefCell<Cave>>) {
        let mut to_print: VecDeque<Rc<RefCell<Cave>>> = VecDeque::new();
        let mut visited: CaveList = Vec::new();
        let mut cave = Rc::clone(&cave_system);
        to_print.push_back(Rc::clone(&cave));
        visited.push(Rc::clone(&cave));
        while !to_print.is_empty() {
            cave = to_print.pop_front().unwrap();
            println!("{}", cave.borrow());
            for c in cave.borrow().connections.iter() {
                if !visited.contains(c) {
                    to_print.push_back(Rc::clone(c));
                }
            }
        }
    }
}

type CaveList = Vec<Rc<RefCell<Cave>>>;

fn print_cave_list(cave_list: &CaveList) {
    for cave in cave_list {
        print!("{} ", cave.borrow());
    }
    println!();
}

fn parse_input(input: &str) -> Rc<RefCell<Cave>> {
    let lines = input.split_terminator("\n").collect::<Vec<&str>>();

    let mut caves: HashMap<&str, Rc<RefCell<Cave>>> = HashMap::new();
    for line in lines {
        let cave_ids: Vec<_> = line.split("-").collect();
        let id_1 = cave_ids[0];
        let id_2 = cave_ids[1];

        if let None = caves.get(&id_1) {
            caves.insert(id_1, Cave::new(id_1));
        }

        if let None = caves.get(&id_2) {
            caves.insert(id_2, Cave::new(id_2));
        }

        let cave_1 = caves.get(&id_1).unwrap();
        let cave_2 = caves.get(&id_2).unwrap();

        cave_1.borrow_mut().connections.push(Rc::clone(&cave_2));
        cave_2.borrow_mut().connections.push(Rc::clone(&cave_1));
    }

    Rc::clone(caves.get("start").unwrap())
}

mod part_1 {
    use super::*;

    fn can_be_visited(cave: Rc<RefCell<Cave>>, visited: &CaveList) -> bool {
        if !visited.contains(&cave) {
            return true;
        }
        if cave.borrow().ctype == CaveType::Big {
            return true;
        }
        return false;
    }

    pub fn visit_cave(
        cave: Rc<RefCell<Cave>>,
        mut visited: CaveList,
        mut curr_path: CaveList,
        paths: &mut Vec<CaveList>,
    ) {
        if cave.borrow().ctype == CaveType::End {
            curr_path.push(Rc::clone(&cave));
            paths.push(curr_path.clone());
            // print!("===> ");
            // print_cave_list(&curr_path);
            return;
        }

        if !visited.contains(&cave) {
            visited.push(Rc::clone(&cave));
        }

        curr_path.push(Rc::clone(&cave));

        for c in cave.borrow().connections.iter() {
            if !can_be_visited(Rc::clone(c), &visited) {
                continue;
            }
            visit_cave(Rc::clone(c), visited.clone(), curr_path.clone(), paths)
        }
    }

    pub fn get_cave_paths(cave_system: Rc<RefCell<Cave>>) -> Vec<CaveList> {
        let visited: CaveList = Vec::new();
        let curr_path: CaveList = Vec::new();
        let mut paths: Vec<CaveList> = Vec::new();
        visit_cave(cave_system, visited, curr_path, &mut paths);
        paths
    }

    pub fn count_paths(cave_system: &Rc<RefCell<Cave>>) -> usize {
        let paths = get_cave_paths(Rc::clone(cave_system));
        paths.len()
    }
}

mod part_2 {
    use super::*;

    fn can_be_visited(
        cave: Rc<RefCell<Cave>>,
        visited: &CaveList,
        twiced_small: &Option<Rc<RefCell<Cave>>>,
    ) -> bool {
        if !visited.contains(&cave) {
            return true;
        }
        if cave.borrow().ctype == CaveType::Big {
            return true;
        }
        if cave.borrow().ctype == CaveType::Small {
            if *twiced_small != None {
                return false;
            }
            return true;
        }
        return false;
    }

    pub fn visit_cave(
        cave: Rc<RefCell<Cave>>,
        mut visited: CaveList,
        mut twiced_small: Option<Rc<RefCell<Cave>>>,
        mut curr_path: CaveList,
        paths: &mut Vec<CaveList>,
    ) {
        if cave.borrow().ctype == CaveType::End {
            curr_path.push(Rc::clone(&cave));
            paths.push(curr_path.clone());
            // print!("===> ");
            // print_cave_list(&curr_path);
            return;
        }

        if !visited.contains(&cave) {
            visited.push(Rc::clone(&cave));
        } else if cave.borrow().ctype == CaveType::Small {
            if twiced_small == None {
                twiced_small = Some(Rc::clone(&cave));
            }
        }

        curr_path.push(Rc::clone(&cave));

        for c in cave.borrow().connections.iter() {
            if !can_be_visited(Rc::clone(c), &visited, &twiced_small) {
                continue;
            }
            visit_cave(
                Rc::clone(c),
                visited.clone(),
                twiced_small.clone(),
                curr_path.clone(),
                paths,
            )
        }
    }

    pub fn get_cave_paths(cave_system: Rc<RefCell<Cave>>) -> Vec<CaveList> {
        let visited: CaveList = Vec::new();
        let twiced_small: Option<Rc<RefCell<Cave>>> = None;
        let curr_path: CaveList = Vec::new();
        let mut paths: Vec<CaveList> = Vec::new();
        visit_cave(cave_system, visited, twiced_small, curr_path, &mut paths);
        paths
    }

    pub fn count_paths(cave_system: &Rc<RefCell<Cave>>) -> usize {
        let paths = get_cave_paths(Rc::clone(cave_system));
        paths.len()
    }
}

fn main() {
    let cave_system = parse_input(&read_to_string("data/day-12.txt").unwrap());

    println!("== PART 1");
    let path_count = part_1::count_paths(&cave_system);
    println!(
        "Number of paths visiting small caves at most once: {}",
        path_count
    );
    println!();
    println!("== PART 2");
    let path_count = part_2::count_paths(&cave_system);
    println!(
        "Number of paths visiting one small cave twice and the others only once: {}",
        path_count
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    #[test]
    fn cave_system_should_start_with_start_cave() {
        let cave_system = parse_input(INPUT);
        assert_eq!(cave_system.borrow().ctype, CaveType::Start);
    }

    #[test]
    fn start_node_should_have_big_a_and_small_b_as_children() {
        let cave_system = parse_input(INPUT);
        let children = &cave_system.borrow().connections;
        assert_eq!(children.len(), 2);

        let child_a = Rc::clone(&children[0]);
        assert_eq!(child_a.borrow().ctype, CaveType::Big);
        assert_eq!(child_a.borrow().id, "A");

        let child_b = Rc::clone(&children[1]);
        assert_eq!(child_b.borrow().ctype, CaveType::Small);
        assert_eq!(child_b.borrow().id, "b");
    }

    #[test]
    fn visited_contains_using_rc_refcell() {
        let mut visited: CaveList = Vec::new();
        let n_start = Cave::new("start");
        let n_a = Cave::new("A");
        assert_eq!(visited.contains(&Rc::clone(&n_start)), false);
        visited.push(Rc::clone(&n_start));
        visited.push(Rc::clone(&n_a));
        assert_eq!(visited.contains(&Rc::clone(&n_start)), true);
        assert_eq!(visited.contains(&Rc::clone(&n_a)), true);
    }

    #[test]
    fn cave_system_with_just_start_and_end() {
        let cave_system = parse_input("start-end");
        Cave::print(Rc::clone(&cave_system));
        let paths = part_1::get_cave_paths(Rc::clone(&cave_system));
        // println!("PATHS");
        // print_cave_list(&paths[0]);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].len(), 2);
    }

    #[test]
    fn cave_system_from_the_example() {
        let cave_system = parse_input(INPUT);
        let paths = part_1::get_cave_paths(Rc::clone(&cave_system));
        assert_eq!(paths.len(), 10);
    }
}
