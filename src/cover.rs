use std::cmp::min;
use std::collections::{HashMap, HashSet};

mod cells;
use cells::{Face, Edge};

use crate::lamination::Lamination;

#[derive(Debug, PartialEq)]
pub struct MarkedMultCover {
    period: u32,
    degree: u32,
    per2: bool,
    max_angle: u32,
    ray_sets: Vec<(u32, u32)>,
    cycles: HashMap<u32, u32>,
    cycle_classes: HashMap<u32, u32>,
    cycle_pairs: Vec<((u32, u32), Vec<u32>)>,
    vertices: Vec<u32>,
    edges: Vec<Edge>,
    faces: HashMap<u32, Face>,
    _visited_cycles: HashSet<u32>,
}

impl MarkedMultCover {
    pub fn new(period: u32, degree: u32, per2: bool) -> Self {
        let max_angle = degree.pow(period as u32) - 1;

        let mut ray_sets = Vec::new();

        let mut cycles = HashMap::new();
        let mut cycle_classes = HashMap::new();
        let mut cycle_pairs = Vec::new();
        let mut vertices = Vec::new();
        let mut edges = Vec::new();

        Self {
            period,
            degree,
            per2,
            max_angle,
            ray_sets,
            cycles,
            cycle_classes,
            cycle_pairs,
            vertices,
            edges,
            faces: HashMap::new(),
            _visited_cycles: HashSet::new(),
        }
    }

    fn _compute_ray_sets(&mut self) {
        let lamination = Lamination::new(self.period, self.degree, self.per2);
        for angles in lamination.arcs_of_period(self.period, true) {
            self.ray_sets.push((
                (angles.0 * (self.max_angle as i32)).to_integer() as u32,
                (angles.1 * (self.max_angle as i32)).to_integer() as u32,
            ));
        }
        self.ray_sets.sort();
    }

    fn _compute_cycles(&mut self) {
        for angle in 0..self.max_angle {
            let orbit = self.orbit(angle);
            if orbit.len() == self.period as usize {
                let min_angle_in_cycle = orbit.iter().min().unwrap();
                let dual_angle = min(*min_angle_in_cycle, self.max_angle - *min_angle_in_cycle);
                self.cycles.insert(angle, *min_angle_in_cycle);
                self.cycle_classes.insert(angle, dual_angle);
            }
        }
    }

    fn _compute_cycle_pairs(&mut self) {
        self.cycle_pairs = self
            .ray_sets
            .iter()
            .map(|&(a, b)| {
                let cycles = vec![self.cycles[&a], self.cycles[&b]];
                ((a, b), cycles)
            })
            .collect();
    }

    fn _compute_vertices(&mut self) {
        // Vertices, labeled by minimum cycle representative
        self.vertices = self.cycles.values().map(|v| *v).collect::<Vec<_>>();
        self.vertices.sort_unstable();
        self.vertices.dedup();
    }

    fn _compute_edges(&mut self) {
        // Primitive leaves of lamination,
        // labeled by minimum cycle representative
        self.edges = self
            .cycle_pairs
            .iter()
            .filter_map(|(ref t, vec)| {
                if let [a, b] = vec.as_slice() {
                    if a != b {
                        let angles = (t.0, t.1);
                        Some(Edge {
                            angles,
                            endpoints: (*a, *b),
                            period: self.period,
                            degree: self.degree,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
    }

    pub fn run(&mut self) {
        self._compute_ray_sets();
        self._compute_cycles();
        self._compute_cycle_pairs();
        self._compute_vertices();
        self._compute_edges();
        self._compute_faces();
    }

    fn euler_characteristic(&self) -> isize {
        self.vertices.len() as isize - self.edges.len() as isize + self.faces.len() as isize
    }

    fn genus(&self) -> isize {
        1 - self.euler_characteristic() / 2
    }

    fn face_sizes(&self) -> Vec<usize> {
        self.faces.values().map(|f| f.vertices.len()).collect()
    }

    fn num_odd_faces(&self) -> usize {
        self.face_sizes().iter().filter(|&s| s % 2 == 1).count()
    }

    fn orbit(&self, angle: u32) -> HashSet<u32> {
        get_orbit(angle, self.max_angle, self.period, self.degree)
    }

    fn _compute_faces(&mut self) {
        self.faces.clear();
        self._visited_cycles.clear();

        for cycle in self.cycles.values().cloned().collect::<HashSet<_>>() {
            if !self._visited_cycles.contains(&cycle) {
                let face = self._traverse_face(cycle);
                self.faces.insert(cycle, face);
            }
        }
    }

    fn _traverse_face(&mut self, starting_angle: u32) -> Face {
        let mut node = starting_angle;
        let mut nodes = Vec::new();
        nodes.push(node);

        let mut face_degree = 1;

        loop {
            for edge in &self.edges {
                let (a, b) = edge.endpoints;
                if node == a {
                    node = b;
                    nodes.push(node);
                } else if node == b {
                    node = a;
                    nodes.push(node);
                }
            }

            if node == starting_angle {
                if nodes.len() > 1 {
                    nodes.pop();
                }
                return Face {
                    vertices: nodes,
                    degree: face_degree,
                };
            } else {
                self._visited_cycles.insert(self.cycles[&node]);
            }

            face_degree += 1;
        }
    }

    pub fn summarize(&self, indent: usize) {
        let indent_str = " ".repeat(indent);

        println!("\n{} vertices:", self.vertices.len());
        let n = self.period;
        let m = (self.degree.pow(n as u32)).to_string().len();
        for v in &self.vertices {
            println!("{}{}", indent_str, v);
        }

        println!("\n{} edges:", self.edges.len());
        for edge in &self.edges {
            println!("{}{:}", indent_str, edge.to_string());
        }

        println!("\n{} faces:", self.faces.len());
        for (p, face) in self.faces.iter().enumerate() {
            println!("{}[{:0>n$b}] = {:?}", indent_str, p, face, n = n as usize);
        }

        println!("\nFace sizes:");
        println!("{}{:?}", indent_str, self.face_sizes());

        println!(
            "\nSmallest face: {}",
            self.face_sizes().iter().min().unwrap()
        );
        println!(
            "\nLargest face: {}",
            self.face_sizes().iter().max().unwrap()
        );
        println!("\nGenus is {}", self.genus());
    }
}

fn get_orbit(angle: u32, max_angle: u32, period: u32, degree: u32) -> HashSet<u32> {
    let mut orbit = HashSet::new();
    let mut theta = angle.clone();

    for _ in 0..period {
        orbit.insert(theta);
        theta = theta * degree % max_angle;
    }

    orbit
}

