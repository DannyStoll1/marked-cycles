use crate::{
    abstract_cycles::{AbstractPointClass, ShiftedCycle},
    types::{Angle, Period},
};

#[derive(Debug, PartialEq)]
pub struct PrimitiveFace
{
    pub label: AbstractPointClass,
    pub vertices: Vec<ShiftedCycle>,
    pub degree: Period,
}

impl PrimitiveFace
{
    pub fn edges(&self) -> Vec<(ShiftedCycle, ShiftedCycle)>
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

impl std::fmt::Display for PrimitiveFace
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let vertices_as_strings: Vec<String> = self
            .vertices
            .iter()
            .map(|v| v.to_point().to_string())
            .collect();
        write!(
            f,
            "{} = ({}); deg = {}",
            self.label,
            vertices_as_strings.join(", "),
            self.degree
        )
    }
}
impl std::fmt::Binary for PrimitiveFace
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let vertices_as_strings: Vec<String> = self
            .vertices
            .iter()
            .map(|v| format!("{:b}", v.to_point()))
            .collect();
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
pub struct SatelliteFace
{
    pub label: ShiftedCycle,
    pub vertices: Vec<ShiftedCycle>,
}

impl SatelliteFace
{
    pub fn edges(&self) -> Vec<(ShiftedCycle, ShiftedCycle)>
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

impl std::fmt::Display for SatelliteFace
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let vertices_as_strings: Vec<String> = self
            .vertices
            .iter()
            .map(|v| v.to_point().to_string())
            .collect();
        write!(f, "{} = ({})", self.label, vertices_as_strings.join(", "))
    }
}

impl std::fmt::Binary for SatelliteFace
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let vertices_as_strings: Vec<String> = self
            .vertices
            .iter()
            .map(|v| format!("{:b}", v.to_point()))
            .collect();
        write!(
            f,
            "{:b} = ({})",
            self.label,
            vertices_as_strings.join(", ")
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Wake
{
    pub theta0: ShiftedCycle,
    pub theta1: ShiftedCycle,
}

impl Wake
{
    pub fn is_real(&self) -> bool
    {
        self.theta0.to_point_class() == self.theta1.to_point_class()
    }

    pub fn is_satellite(&self) -> bool
    {
        self.theta0.matches(self.theta1)
    }
}

impl std::fmt::Display for Wake
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(
            f,
            "Wake({}, {})",
            self.theta0.to_point(),
            self.theta1.to_point()
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Edge
{
    pub start: ShiftedCycle,
    pub end: ShiftedCycle,
}

impl Edge
{
    pub fn is_real(&self) -> bool
    {
        self.start.to_point_class() == self.end.to_point_class()
    }

    pub fn is_parabolic(&self) -> bool
    {
        self.start.matches(self.end)
    }
}

impl std::fmt::Display for Edge
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{} -- {}", self.start.to_point(), self.end.to_point())
    }
}

impl std::fmt::Binary for Edge
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(
            f,
            "{:b} -- {:b}",
            self.start.to_point(),
            self.end.to_point()
        )
    }
}
