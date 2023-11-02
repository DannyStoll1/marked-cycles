use crate::abstract_cycles::{AbstractCycle, AbstractCycleClass, AbstractPoint};
use crate::global_state::{set_period, MAX_ANGLE, PERIOD};
use crate::common::{get_orbit, cells};
use crate::lamination::Lamination;
use crate::types::{IntAngle, Period};
use std::collections::{HashMap, HashSet};

type Vertex = AbstractCycle;
type Edge = cells::Edge<Vertex>;
type Face = cells::Face<Vertex, AbstractCycleClass>;

use self::cells::Wake;

#[derive(Debug, PartialEq)]
pub struct MarkedCycleCoverBuilder
{
    pub period: Period,
    pub crit_period: Period,
    adjacency_map: HashMap<AbstractCycle, Vec<(AbstractCycle, IntAngle)>>,
}

impl MarkedCycleCoverBuilder
{
    #[must_use]
    pub fn new(period: Period, crit_period: Period) -> Self
    {
        Self {
            period,
            crit_period,
            adjacency_map: HashMap::new(),
        }
    }

    #[must_use]
    pub fn build(&mut self) -> MarkedCycleCover
    {
        set_period(self.period);
        let cycles = self.cycles();
        let vertices = self.vertices(&cycles);
        let edges = self.edges(&cycles);
        let faces = self.faces(&vertices);

        MarkedCycleCover {
            crit_period: self.crit_period,
            vertices,
            edges,
            faces,
        }
    }

    #[inline]
    fn orbit(&self, angle: IntAngle) -> Vec<IntAngle>
    {
        get_orbit(angle)
    }

    fn cycles(&self) -> Vec<Option<AbstractCycle>>
    {
        let mut cycles = vec![None; usize::try_from(MAX_ANGLE.get()).unwrap()];
        for theta in 0..MAX_ANGLE.get().into() {
            let theta_usize = usize::try_from(theta).unwrap();
            if cycles[theta_usize].is_some() {
                continue;
            }

            let orbit = self.orbit(theta.into());
            if orbit.len() == PERIOD.get() as usize {
                let cycle_rep = orbit.iter().min().unwrap();
                let cycle_rep = AbstractPoint::new(*cycle_rep);

                orbit
                    .iter()
                    .map(|x| usize::try_from(*x).unwrap())
                    .for_each(|x| {
                        let cycle = AbstractCycle { rep: cycle_rep };
                        cycles[x] = Some(cycle);
                    });
            }
        }
        if PERIOD.get() == 1 {
            let alpha_fp = AbstractPoint::new(IntAngle(1));
            cycles.push(Some(AbstractCycle { rep: alpha_fp }));
        }
        cycles
    }

    fn vertices(&self, cycles: &[Option<AbstractCycle>]) -> Vec<AbstractCycle>
    {
        // Vertices, labeled by abstract point
        let mut vertices = cycles.iter().filter_map(|&v| v).collect::<Vec<_>>();
        vertices.sort_unstable_by_key(|x| x.rep);
        vertices.dedup();
        vertices
    }

    fn edges(&mut self, cycles: &[Option<AbstractCycle>]) -> Vec<Edge>
    {
        Lamination::new()
            .with_crit_period(self.crit_period)
            .into_arcs_of_period(PERIOD.get())
            .into_iter()
            .filter_map(|(theta0, theta1)| {
                let angle0 = MAX_ANGLE.get().scale_by_ratio(&theta0);
                let angle1 = MAX_ANGLE.get().scale_by_ratio(&theta1);

                let k0 = usize::try_from(angle0).ok()?;
                let k1 = usize::try_from(angle1).ok()?;

                let cyc0 = cycles[k0]?;
                let cyc1 = cycles[k1]?;

                if cyc0 == cyc1 {
                    return None;
                }

                let tag = angle0.max(angle1);
                self.adjacency_map
                    .entry(cyc0)
                    .or_insert_with(Vec::new)
                    .push((cyc1, tag));
                self.adjacency_map
                    .entry(cyc1)
                    .or_insert_with(Vec::new)
                    .push((cyc0, tag));

                Some(Edge {
                    start: cyc0,
                    end: cyc1,
                    wake: Wake { angle0, angle1 },
                })
            })
            .collect()
    }

    fn faces(&self, vertices: &[AbstractCycle]) -> Vec<Face>
    {
        let mut visited = HashSet::new();
        vertices
            .iter()
            .cloned()
            .filter_map(|cyc| {
                if visited.contains(&cyc) {
                    return None;
                }
                Some(self.traverse_face(cyc, &mut visited))
            })
            .collect()
    }

    fn traverse_face(
        &self,
        starting_point: AbstractCycle,
        visited: &mut HashSet<AbstractCycle>,
    ) -> Face
    {
        // cycle that is currently marked
        let mut node: AbstractCycle = starting_point;

        // angle of the current parameter
        let mut curr_angle = IntAngle(0);

        let mut nodes = Vec::new();

        let mut face_degree = 1;

        while let Some((next_node, next_angle)) = self.get_next_vertex_and_angle(node, curr_angle) {
            // If we are crossing the real axis
            if curr_angle >= next_angle {
                if node == starting_point {
                    break;
                }
                visited.insert(node);
                face_degree += 1;
            }

            nodes.push(node);
            node = next_node;

            curr_angle = next_angle;
        }

        if nodes.is_empty() {
            nodes.push(node);
        }

        let face_id = AbstractCycleClass::new(starting_point);

        return Face {
            label: face_id,
            vertices: nodes,
            degree: face_degree,
        };
    }

    fn get_next_vertex_and_angle(
        &self,
        node: AbstractCycle,
        curr_angle: IntAngle,
    ) -> Option<(AbstractCycle, IntAngle)>
    {
        self.adjacency_map
            .get(&node)?
            .iter()
            .min_by_key(|(_, ang)| (ang.0 - curr_angle.0 - 1).rem_euclid(MAX_ANGLE.get().0))
            .cloned()
    }
}

#[derive(Debug, PartialEq)]
pub struct MarkedCycleCover
{
    pub crit_period: Period,
    pub vertices: Vec<AbstractCycle>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
}

impl MarkedCycleCover
{
    #[must_use]
    pub fn new(period: Period, crit_period: Period) -> Self
    {
        MarkedCycleCoverBuilder::new(period, crit_period).build()
    }

    #[must_use]
    pub fn euler_characteristic(&self) -> i64
    {
        self.num_vertices() as i64 - self.num_edges() as i64 + self.num_faces() as i64
    }

    #[must_use]
    pub fn num_vertices(&self) -> usize
    {
        self.vertices.len()
    }

    #[must_use]
    pub fn num_edges(&self) -> usize
    {
        self.edges.len()
    }

    #[must_use]
    pub fn num_faces(&self) -> usize
    {
        self.faces.len()
    }

    #[must_use]
    pub fn genus(&self) -> i64
    {
        1 - self.euler_characteristic() / 2
    }

    pub fn face_sizes(&self) -> impl Iterator<Item = usize> + '_
    {
        self.faces.iter().map(Face::len)
    }

    #[must_use]
    pub fn num_odd_faces(&self) -> usize
    {
        self.face_sizes().filter(|&s| s % 2 == 1).count()
    }

    pub fn summarize(&self, indent: usize, binary: bool)
    {
        let indent_str = " ".repeat(indent);
        macro_rules! print_elements {
            ($title: expr, $iter: expr, $count: expr) => {
                if $count > crate::MAX_DISPLAY_ITEMS {
                    println!("\n{} {}", $count, $title);
                } else {
                    println!("\n{} {}:", $count, $title);
                    for elem in $iter {
                        if binary {
                            println!("{indent_str}{elem:b}");
                        } else {
                            println!("{indent_str}{elem}");
                        }
                    }
                }
            };
        }

        print_elements!("vertices", &self.vertices, self.vertices.len());
        print_elements!("edges", &self.edges, self.edges.len());
        print_elements!("faces", &self.faces, self.faces.len());

        if self.faces.len() < crate::MAX_DISPLAY_ITEMS {
            println!("\nFace sizes:");
            println!("{}{:?}", indent_str, self.face_sizes().collect::<Vec<_>>());
        }

        println!("\nSmallest face: {}", self.face_sizes().min().unwrap());
        println!("\nLargest face: {}", self.face_sizes().max().unwrap());
        println!("\nGenus is {}", self.genus());
    }
}
