use crate::{
    abstract_cycles::{AbstractCycle, AbstractCycleClass},
    types::Period,
};

#[derive(Debug, PartialEq)]
pub struct Face
{
    pub label: AbstractCycleClass,
    pub vertices: Vec<AbstractCycle>,
    pub degree: Period,
}

impl Face
{
    pub fn edges(&self) -> Vec<(AbstractCycle, AbstractCycle)>
    {
        let mut edges = Vec::new();
        for i in 0..self.vertices.len()
        {
            edges.push((
                self.vertices[i],
                self.vertices[(i + 1) % self.vertices.len()],
            ));
        }
        edges
    }

    pub fn len(&self) -> usize
    {
        self.vertices.len()
    }
}

impl std::fmt::Display for Face
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let vertices_as_strings: Vec<String> =
            self.vertices.iter().map(|v| v.to_string()).collect();
        write!(
            f,
            "{} = ({}); deg = {}",
            self.label,
            vertices_as_strings.join(", "),
            self.degree
        )
    }
}
impl std::fmt::Binary for Face
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let vertices_as_strings: Vec<String> =
            self.vertices.iter().map(|v| format!("{:b}", v)).collect();
        write!(
            f,
            "{:b} = ({}); deg = {}",
            self.label,
            vertices_as_strings.join(", "),
            self.degree
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Wake
{
    pub theta0: AbstractCycle,
    pub theta1: AbstractCycle,
}

impl Wake
{
    pub fn is_real(&self) -> bool
    {
        self.theta0.compute_cycle_class() == self.theta1.compute_cycle_class()
    }

    pub fn is_satellite(&self) -> bool
    {
        self.theta0 == self.theta1
    }
}

impl std::fmt::Display for Wake
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "Wake({}, {})", self.theta0, self.theta1,)
    }
}

#[derive(Debug, PartialEq)]
pub struct Edge
{
    pub start: AbstractCycle,
    pub end: AbstractCycle,
}

impl Edge
{
    pub fn is_real(&self) -> bool
    {
        self.start.compute_cycle_class() == self.end.compute_cycle_class()
    }

    pub fn is_parabolic(&self) -> bool
    {
        self.start == self.end
    }
}

impl std::fmt::Display for Edge
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{} -- {}", self.start, self.end)
    }
}

impl std::fmt::Binary for Edge
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{:b} -- {:b}", self.start, self.end)
    }
}
