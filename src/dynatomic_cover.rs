use crate::abstract_cycles::{AbstractPoint, AbstractPointClass, ShiftedCycle};
use crate::lamination::Lamination;
use crate::types::{Angle, Period};
use std::collections::HashSet;

mod cells;
use cells::{Edge, PrimitiveFace, SatelliteFace, Wake};
use num::Integer;

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
pub struct DynatomicCover
{
    pub period: Period,
    pub degree: Period,
    pub crit_period: Period,
    max_angle: Angle,
    ray_sets: Vec<(Angle, Angle)>,
    pub cycles_with_shifts: Vec<Option<ShiftedCycle>>,
    pub point_classes: Vec<Option<AbstractPointClass>>,
    pub vertices: Vec<ShiftedCycle>,
    pub wakes: Vec<Wake>,
    pub edges: Vec<Edge>,
    pub primitive_faces: Vec<PrimitiveFace>,
    pub satellite_faces: Vec<SatelliteFace>,
    visited_face_ids: HashSet<AbstractPointClass>,
}

impl DynatomicCover
{
    pub fn new(period: Period, degree: Period, crit_period: Period) -> Self
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
            cycles_with_shifts,
            point_classes,
            vertices: Vec::new(),
            wakes: Vec::new(),
            edges: Vec::new(),
            primitive_faces: Vec::new(),
            satellite_faces: Vec::new(),
            visited_face_ids: HashSet::new(),
        };
        curve.run();
        curve
    }

    fn compute_ray_sets(&mut self)
    {
        let lamination = Lamination::new(self.period, self.degree, self.crit_period);
        for angles in lamination.arcs_of_period(self.period, true)
        {
            self.ray_sets.push((
                (angles.0 * (self.max_angle.0 as i64)).to_integer().into(),
                (angles.1 * (self.max_angle.0 as i64)).to_integer().into(),
            ));
        }
        self.ray_sets.sort();
    }

    fn compute_cycles(&mut self)
    {
        for theta in 0..self.max_angle.into()
        {
            let theta_usize = usize::try_from(theta).unwrap();
            if self.cycles_with_shifts[theta_usize].is_some()
            {
                continue;
            }

            let point = AbstractPoint::new(theta.into(), self.period);

            let orbit = self.orbit(theta.into());
            if orbit.len() == self.period as usize
            {
                let (rep_idx, cycle_rep) = orbit.iter().enumerate().min_by_key(|x| x.1).unwrap();
                let cycle_rep = AbstractPoint::new(*cycle_rep, self.period);

                orbit
                    .iter()
                    .map(|x| usize::try_from(*x).unwrap())
                    .enumerate()
                    .for_each(|(i, x)| {
                        let shift = ((i as i64) - (rep_idx as i64)).rem_euclid(self.period);
                        let shifted_cycle = ShiftedCycle {
                            rep: cycle_rep,
                            shift,
                        };
                        self.cycles_with_shifts[x] = Some(shifted_cycle);
                    });
                // FIXME: should this be moved into the loop?
                self.point_classes[theta_usize] = Some(point.into());
            }
        }
    }

    fn compute_vertices(&mut self)
    {
        // Vertices, labeled by abstract point
        self.vertices = self
            .cycles_with_shifts
            .iter()
            .filter_map(|&v| v)
            .collect::<Vec<_>>();
    }

    fn compute_edges(&mut self)
    {
        // Leaves of lamination, labeled by shifted cycle
        self.wakes = self
            .ray_sets
            .iter()
            .map(|(theta0, theta1)| {
                let cycle0 = self.cycles_with_shifts[usize::try_from(*theta0).unwrap()].unwrap();
                let cycle1 = self.cycles_with_shifts[usize::try_from(*theta1).unwrap()].unwrap();
                Wake {
                    theta0: cycle0.into(),
                    theta1: cycle1.into(),
                }
            })
            .collect();

        self.edges = self
            .wakes
            .iter()
            .flat_map(|w| {
                (0..self.period).map(|i| Edge {
                    start: w.theta0.rotate(i),
                    end: w.theta1.rotate(i),
                })
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

    pub fn euler_characteristic(&self) -> isize
    {
        self.num_vertices() as isize - self.num_edges() as isize + self.num_faces() as isize
    }

    pub fn num_vertices(&self) -> usize
    {
        self.vertices.len()
    }

    pub fn num_edges(&self) -> usize
    {
        self.edges.len()
    }

    pub fn num_faces(&self) -> usize
    {
        self.primitive_faces.len() + self.satellite_faces.len()
    }

    pub fn genus(&self) -> isize
    {
        1 - self.euler_characteristic() / 2
    }

    pub fn face_sizes(&self) -> Vec<usize>
    {
        let primitive_sizes = self.primitive_faces.iter().map(|f| f.vertices.len());

        let satellite_sizes = self.satellite_faces.iter().map(|f| f.vertices.len());

        primitive_sizes.chain(satellite_sizes).collect()
    }

    pub fn num_odd_faces(&self) -> usize
    {
        self.face_sizes().iter().filter(|&s| s % 2 == 1).count()
    }

    pub fn orbit(&self, angle: Angle) -> Vec<Angle>
    {
        get_orbit(angle, self.max_angle, self.period, self.degree)
    }

    fn compute_faces(&mut self)
    {
        self.visited_face_ids.clear();

        self.primitive_faces = self
            .vertices
            .clone()
            .iter()
            .filter_map(|scycle| self._traverse_face(*scycle))
            .collect();

        self.satellite_faces = self
            .wakes
            .iter()
            .filter(|e| e.is_satellite())
            .flat_map(|e| {
                let shift = e.theta1.relative_shift(e.theta0);
                let num_faces = shift.gcd(&self.period);
                let face_period = self.period / num_faces;
                (0..num_faces).map(move |i| {
                    let base_point = e.theta0.with_shift(0).rotate(i);
                    SatelliteFace {
                        label: base_point,
                        vertices: (0..face_period)
                            .map(|j| base_point.rotate(j * num_faces))
                            .collect(),
                    }
                })
            })
            .collect();
    }

    fn _traverse_face(&mut self, starting_point: ShiftedCycle) -> Option<PrimitiveFace>
    {
        if self
            .visited_face_ids
            .contains(&starting_point.to_point_class())
        {
            return None;
        }

        let mut node = starting_point;
        let mut nodes = Vec::new();
        nodes.push(node);

        let mut face_degree = 1;

        loop
        {
            for edge in &self.wakes
            {
                let (a, b) = (edge.theta0, edge.theta1);
                if node.matches(a)
                {
                    // This handles the satellite case as well as half of the primitive cases
                    let rel_shift = node.relative_shift(a);
                    node = b.rotate(rel_shift);
                    nodes.push(node);
                }
                else if node.matches(b)
                {
                    let rel_shift = node.relative_shift(b);
                    node = a.rotate(rel_shift);
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
                return Some(PrimitiveFace {
                    label: starting_point.into(),
                    vertices: nodes,
                    degree: face_degree,
                });
            }
            else
            {
                self.visited_face_ids.insert(node.to_point_class());
            }

            face_degree += 1;
        }
    }

    pub fn summarize(&self, indent: usize, binary: bool)
    {
        let indent_str = " ".repeat(indent);

        if binary
        {
            println!("\n{} vertices:", self.vertices.len());
            for v in &self.vertices
            {
                println!("{}{:b}", indent_str, v.to_point());
            }

            println!("\n{} edges:", self.edges.len());
            for edge in &self.edges
            {
                println!("{}{:b}", indent_str, edge);
            }

            println!("\n{} primitive faces:", self.primitive_faces.len());
            for face in self.primitive_faces.iter()
            {
                println!("{}{:b}", indent_str, face);
            }

            println!("\n{} satellite faces:", self.satellite_faces.len());
            for face in self.satellite_faces.iter()
            {
                println!("{}{:b}", indent_str, face);
            }
        }
        else
        {
            println!("\n{} vertices:", self.vertices.len());
            for v in &self.vertices
            {
                println!("{}{}", indent_str, v.to_point());
            }

            println!("\n{} edges:", self.edges.len());
            for edge in &self.edges
            {
                println!("{}{}", indent_str, edge);
            }

            println!("\n{} primitive faces:", self.primitive_faces.len());
            for face in self.primitive_faces.iter()
            {
                println!("{}{}", indent_str, face);
            }

            println!("\n{} satellite faces:", self.satellite_faces.len());
            for face in self.satellite_faces.iter()
            {
                println!("{}{}", indent_str, face);
            }
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
