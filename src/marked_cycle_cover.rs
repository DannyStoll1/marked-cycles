use crate::abstract_cycles::{AbstractCycle, AbstractCycleClass, AbstractPoint};
use crate::lamination::Lamination;
use crate::types::{Angle, Period};
use std::collections::HashSet;

mod cells;
use cells::{Edge, Face, Wake};

fn get_orbit(angle: Angle, max_angle: Angle, period: Period, degree: Period) -> Vec<Angle>
{
    let mut orbit = Vec::with_capacity(period as usize);

    orbit.push(angle);
    let mut theta = angle * degree % max_angle;

    while theta != angle
    {
        orbit.push(theta);
        theta = theta * degree % max_angle;
    }

    orbit
}

#[derive(Debug, PartialEq)]
pub struct MarkedCycleCover
{
    pub period: Period,
    pub degree: Period,
    pub crit_period: Period,
    max_angle: Angle,
    ray_sets: Vec<(Angle, Angle)>,
    pub cycles: Vec<Option<AbstractCycle>>,
    pub cycle_classes: Vec<Option<AbstractCycleClass>>,
    pub vertices: Vec<AbstractCycle>,
    pub wakes: Vec<Wake>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
    visited_face_ids: HashSet<AbstractCycleClass>,
}

impl MarkedCycleCover
{
    #[must_use] pub fn new(period: Period, degree: Period, crit_period: Period) -> Self
    {
        let max_angle = Angle(degree.pow(period.try_into().unwrap()) - 1);

        let ray_sets = Vec::new();

        let cycles_with_shifts = vec![None; max_angle.try_into().unwrap()];
        let point_classes = vec![None; max_angle.try_into().unwrap()];

        let mut curve = Self {
            period,
            degree,
            crit_period,
            max_angle,
            ray_sets,
            cycles: cycles_with_shifts,
            cycle_classes: point_classes,
            vertices: Vec::new(),
            wakes: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
            visited_face_ids: HashSet::new(),
        };
        curve.run();
        curve
    }

    fn compute_ray_sets(&mut self)
    {
        Lamination::new()
            .with_crit_period(self.crit_period)
            .arcs_of_period(self.period)
            .iter()
            .for_each(|angles| {
                self.ray_sets.push((
                    (angles.0 * self.max_angle.0).to_integer().into(),
                    (angles.1 * self.max_angle.0).to_integer().into(),
                ));
            });
        self.ray_sets.sort();
    }

    fn compute_cycles(&mut self)
    {
        for theta in 0..self.max_angle.into()
        {
            let theta_usize = usize::try_from(theta).unwrap();
            if self.cycles[theta_usize].is_some()
            {
                continue;
            }

            let orbit = self.orbit(theta.into());
            if orbit.len() == self.period as usize
            {
                let cycle_rep = orbit.iter().min().unwrap();
                let cycle_rep = AbstractPoint::new(*cycle_rep, self.period);

                orbit
                    .iter()
                    .map(|x| usize::try_from(*x).unwrap())
                    .for_each(|x| {
                        let cycle = AbstractCycle { rep: cycle_rep };
                        self.cycles[x] = Some(cycle);
                        self.cycle_classes[x] = Some(cycle.into());
                    });
            }
        }
    }

    fn compute_vertices(&mut self)
    {
        // Vertices, labeled by abstract point
        self.vertices = self.cycles.iter().filter_map(|&v| v).collect::<Vec<_>>();
        self.vertices.sort_unstable_by_key(|x| x.rep);
        self.vertices.dedup();
    }

    fn compute_edges(&mut self)
    {
        // Leaves of lamination, labeled by shifted cycle
        self.wakes = self
            .ray_sets
            .iter()
            .map(|(theta0, theta1)| {
                let cycle0 = self.cycles[usize::try_from(*theta0).unwrap()].unwrap();
                let cycle1 = self.cycles[usize::try_from(*theta1).unwrap()].unwrap();
                Wake {
                    theta0: cycle0,
                    theta1: cycle1,
                }
            })
            .collect();

        self.edges = self
            .wakes
            .iter()
            .filter(|x| !x.is_satellite())
            .map(|w| Edge {
                start: w.theta0,
                end: w.theta1,
            })
            .collect()
    }

    pub fn run(&mut self)
    {
        self.compute_ray_sets();
        self.compute_cycles();
        self.compute_vertices();
        self.compute_edges();
        self.compute_faces();
    }

    #[must_use] pub fn euler_characteristic(&self) -> isize
    {
        self.num_vertices() as isize - self.num_edges() as isize + self.num_faces() as isize
    }

    #[must_use] pub fn num_vertices(&self) -> usize
    {
        self.vertices.len()
    }

    #[must_use] pub fn num_edges(&self) -> usize
    {
        self.edges.len()
    }

    #[must_use] pub fn num_faces(&self) -> usize
    {
        self.faces.len()
    }

    #[must_use] pub fn genus(&self) -> isize
    {
        1 - self.euler_characteristic() / 2
    }

    pub fn face_sizes(&self) -> impl Iterator<Item = usize> + '_
    {
        self.faces.iter().map(Face::len)
    }

    #[must_use] pub fn num_odd_faces(&self) -> usize
    {
        self.face_sizes().filter(|&s| s % 2 == 1).count()
    }

    #[must_use] pub fn orbit(&self, angle: Angle) -> Vec<Angle>
    {
        get_orbit(angle, self.max_angle, self.period, self.degree)
    }

    // Should only be called when cycle classes have already been computed
    fn get_cycle_class(&self, cycle: AbstractCycle) -> AbstractCycleClass
    {
        self.cycle_classes[usize::try_from(cycle.rep.angle).unwrap()].unwrap()
    }

    fn compute_faces(&mut self)
    {
        self.visited_face_ids.clear();

        self.faces = self
            .vertices
            .clone()
            .iter()
            .filter_map(|cyc| self.traverse_face(*cyc))
            .collect();
    }

    fn traverse_face(&mut self, starting_point: AbstractCycle) -> Option<Face>
    {
        let face_id = self.get_cycle_class(starting_point);
        if self.visited_face_ids.contains(&face_id)
        {
            return None;
        }

        let mut node = starting_point;
        let mut nodes = Vec::new();
        nodes.push(node);

        let mut face_degree = 1;

        loop
        {
            for Wake { theta0, theta1 } in &self.wakes
            {
                if node == *theta0
                {
                    node = *theta1;
                    nodes.push(node);
                }
                else if node == *theta1
                {
                    node = *theta0;
                    nodes.push(node);
                }
            }

            if node == starting_point
            {
                // Remove repeated starting vertex
                if nodes.len() > 1
                {
                    nodes.pop();
                }
                return Some(Face {
                    label: starting_point.into(),
                    vertices: nodes,
                    degree: face_degree,
                });
            }
            self.visited_face_ids.insert(self.get_cycle_class(node));

            face_degree += 1;
        }
    }

    pub fn summarize(&self, indent: usize, binary: bool)
    {
        let indent_str = " ".repeat(indent);

        println!("\n{} vertices:", self.vertices.len());
        if binary
        {
            for v in &self.vertices
            {
                println!("{}{:b}", indent_str, v);
            }

            println!("\n{} edges:", self.edges.len());
            for edge in &self.edges
            {
                println!("{}{:b}", indent_str, edge);
            }

            println!("\n{} faces:", self.faces.len());
            for face in self.faces.iter()
            {
                println!("{}{:b}", indent_str, face);
            }
        }
        else
        {
            for v in &self.vertices
            {
                println!("{}{}", indent_str, v);
            }

            println!("\n{} edges:", self.edges.len());
            for edge in &self.edges
            {
                println!("{}{}", indent_str, edge);
            }

            println!("\n{} faces:", self.faces.len());
            for face in self.faces.iter()
            {
                println!("{}{}", indent_str, face);
            }
        }

        println!("\nFace sizes:");
        println!("{}{:?}", indent_str, self.face_sizes().collect::<Vec<_>>());

        println!("\nSmallest face: {}", self.face_sizes().min().unwrap());
        println!("\nLargest face: {}", self.face_sizes().max().unwrap());
        println!("\nGenus is {}", self.genus());
    }
}

// impl From<Lamination> for MarkedCycleCover {
//     fn from(value: Lamination) -> Self {
//         let max_angle = Angle(degree.pow(period.try_into().unwrap()) - 1);
//
//         let ray_sets = Vec::new();
//
//         let cycles_with_shifts = vec![None; max_angle.try_into().unwrap()];
//         let point_classes = vec![None; max_angle.try_into().unwrap()];
//
//         let mut curve = Self {
//             period,
//             degree,
//             crit_period,
//             max_angle,
//             ray_sets,
//             cycles: cycles_with_shifts,
//             cycle_classes: point_classes,
//             vertices: Vec::new(),
//             wakes: Vec::new(),
//             edges: Vec::new(),
//             faces: Vec::new(),
//             visited_face_ids: HashSet::new(),
//         };
//         curve.run();
//         curve
//     }
// }
