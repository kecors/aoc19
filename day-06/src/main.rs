use std::collections::{HashMap, VecDeque};
use std::io::{stdin, Read};

#[derive(Debug)]
struct Solver<'a> {
    child_parents: HashMap<&'a str, &'a str>,
    parent_children: HashMap<&'a str, Vec<&'a str>>,
    orbit_counts: HashMap<&'a str, u32>,
}

impl<'a> Solver<'a> {
    fn new(input: &str) -> Solver {
        let mut child_parents = HashMap::new();
        let mut parent_children = HashMap::new();

        input.lines().for_each(|line| {
            let objects: Vec<&str> = line.split(')').collect();
            child_parents.insert(objects[1], objects[0]);
            let o = parent_children.entry(objects[0]).or_insert(Vec::new());
            o.push(objects[1]);
        });

        let orbit_counts = HashMap::new();

        Solver {
            child_parents,
            parent_children,
            orbit_counts,
        }
    }

    fn compute_orbit_counts(&mut self) {
        self.orbit_counts.insert("COM", 0);

        let mut orbiters: VecDeque<&str> = VecDeque::new();
        if let Some(children) = self.parent_children.get("COM") {
            for child in children.iter() {
                orbiters.push_back(child);
            }
        }

        while let Some(orbiter) = orbiters.pop_front() {
            if let Some(parent_orbit_count) = self.get_parent_orbit_count(orbiter) {
                self.orbit_counts.insert(orbiter, parent_orbit_count + 1);
            }

            if let Some(children) = self.parent_children.get(&orbiter) {
                for child in children.iter() {
                    orbiters.push_back(child);
                }
            }
        }
    }

    fn get_parent_orbit_count(&self, orbiter: &str) -> Option<u32> {
        let parent = self.child_parents.get(&orbiter)?;
        self.orbit_counts.get(parent).copied()
    }

    fn sum_orbit_counts(&self) -> u32 {
        self.orbit_counts.values().sum()
    }

    fn calculate_transfers(&self, orbiter_1: &str, orbiter_2: &str) -> u32 {
        let path_1 = self.get_path(orbiter_1);
        let path_2 = self.get_path(orbiter_2);

        let mut j = 0;
        loop {
            if path_1[j] != path_2[j] {
                break;
            }
            j += 1;
        }

        (path_1.len() - 1 - j + path_2.len() - 1 - j) as u32
    }

    fn get_path(&self, mut orbiter: &'a str) -> Vec<&str> {
        let mut path = vec![orbiter];

        while let Some(parent) = self.child_parents.get(orbiter) {
            path.push(parent);
            orbiter = parent;
        }

        path.into_iter().rev().collect()
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut solver = Solver::new(&input);

    solver.compute_orbit_counts();

    println!(
        "Part 1: the total number of orbits is {}",
        solver.sum_orbit_counts()
    );

    println!(
        "Part 2: the minimum tranfers requires is {}",
        solver.calculate_transfers("YOU", "SAN")
    );
}
