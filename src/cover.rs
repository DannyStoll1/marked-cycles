use std::cmp::min;
use std::collections::{HashMap, HashSet};

mod cells;
use cells::{Face, Edge};

use crate::lamination::Lamination;
use crate::types::Period;

#[derive(Debug, PartialEq)]
pub struct MarkedMultCover {
    period: Period,
    degree: Period,
    crit_period: Period,
    max_angle: Period,
    ray_sets: Vec<(Period, Period)>,
    cycles: HashMap<Period, Period>,
    cycle_classes: HashMap<Period, Period>,
    cycle_pairs: Vec<((Period, Period), Vec<Period>)>,
    vertices: Vec<Period>,
    edges: Vec<Edge>,
    faces: HashMap<Period, Face>,
    _visited_cycles: HashSet<Period>,
}

impl MarkedMultCover {
    pub fn new(period: Period, degree: Period, crit_period: Period) -> Self {
        let max_angle = degree.pow(period.try_into().unwrap()) - 1;

        let ray_sets = Vec::new();

        let cycles = HashMap::new();
        let cycle_classes = HashMap::new();
        let cycle_pairs = Vec::new();
        let vertices = Vec::new();
        let edges = Vec::new();

        let mut curve = Self {
            period,
            degree,
            crit_period,
            max_angle,
            ray_sets,
            cycles,
            cycle_classes,
            cycle_pairs,
            vertices,
            edges,
            faces: HashMap::new(),
            _visited_cycles: HashSet::new(),
        };
        curve.run();
        curve
    }

    fn _compute_ray_sets(&mut self) {
        let lamination = Lamination::new(self.period, self.degree, self.crit_period);
        for angles in lamination.arcs_of_period(self.period, true) {
            self.ray_sets.push((
                (angles.0 * self.max_angle).to_integer() as Period,
                (angles.1 * self.max_angle).to_integer() as Period,
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

    pub fn euler_characteristic(&self) -> isize {
        self.vertices.len() as isize - self.edges.len() as isize + self.faces.len() as isize
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn num_faces(&self) -> usize {
        self.faces.len()
    }

    pub fn genus(&self) -> isize {
        1 - self.euler_characteristic() / 2
    }

    pub fn face_sizes(&self) -> Vec<usize> {
        self.faces.values().map(|f| f.vertices.len()).collect()
    }

    pub fn num_odd_faces(&self) -> usize {
        self.face_sizes().iter().filter(|&s| s % 2 == 1).count()
    }

    pub fn orbit(&self, angle: Period) -> HashSet<Period> {
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

    fn _traverse_face(&mut self, starting_angle: Period) -> Face {
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
        // let m = (self.degree.pow(n as Period)).to_string().len();
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

fn get_orbit(angle: Period, max_angle: Period, period: Period, degree: Period) -> HashSet<Period> {
    let mut orbit = HashSet::new();
    let mut theta = angle.clone();

    for _ in 0..period {
        orbit.insert(theta);
        theta = theta * degree % max_angle;
    }

    orbit
}

