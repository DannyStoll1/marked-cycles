use crate::{
    abstract_cycles::{AbstractCycle, AbstractCycleClass, AbstractPoint},
    types::{IntAngle, Period}, global_state::PERIOD,
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
        for i in 0..self.vertices.len() {
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
    pub angle0: IntAngle,
    pub angle1: IntAngle,
}

impl Wake
{
    pub fn is_satellite(&self) -> bool
    {
        self.angle0 == self.angle1
    }
}

impl std::fmt::Display for Wake
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        if let Some(width) = f.width() {
            write!(f, "{:>width$} <-> {:<width$}", self.angle0, self.angle1)
        } else {
            write!(f, "{} <-> {}", self.angle0, self.angle1)
        }
    }
}

impl std::fmt::Binary for Wake
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        if let Some(width) = f.width() {
            write!(f, "{:>width$b} <-> {:<width$b}", self.angle0, self.angle1)
        } else {
            write!(f, "{:b} <-> {:b}", self.angle0, self.angle1)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Edge
{
    pub start: AbstractCycle,
    pub end: AbstractCycle,
    pub wake: Wake,
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
        let ks = AbstractPoint::new(self.wake.angle0).kneading_sequence();
        write!(
            f,
            "{:>digits$} -- {:<digits$}   wake = {:digits$}   KS = {ks:>period$}",
            self.start,
            self.end,
            self.wake,
            digits = (PERIOD.get() / 3) as usize,
            period = PERIOD.get() as usize
        )
    }
}

impl std::fmt::Binary for Edge
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let ks = AbstractPoint::new(self.wake.angle0).kneading_sequence();
        write!(
            f,
            "{:b} -- {:b}   wake = {wake:b}   KS = {ks:>period$}",
            self.start,
            self.end,
            wake = self.wake,
            period = PERIOD.get() as usize
        )
    }
}
