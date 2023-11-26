use crate::global_state::{MAX_ANGLE, PERIOD};
use crate::types::IntAngle;

#[must_use]
#[inline]
pub fn get_orbit(angle: IntAngle) -> Vec<IntAngle>
{
    let mut orbit = Vec::with_capacity(PERIOD.get() as usize);

    orbit.push(angle);
    let mut theta = angle * 2 % MAX_ANGLE.get();

    while theta != angle {
        orbit.push(theta);
        theta = theta * 2 % MAX_ANGLE.get();
    }

    orbit
}

pub mod cells
{
    use crate::{
        abstract_cycles::AbstractPoint,
        global_state::{MAX_ANGLE, PERIOD},
        types::{IntAngle, Period},
    };

    #[derive(Debug, PartialEq, Eq)]
    pub struct Face<V, F>
    {
        pub label: F,
        pub vertices: Vec<V>,
        pub degree: Period,
    }

    impl<V, F> Face<V, F>
    {
        pub fn edges(&self) -> Vec<(V, V)>
        where
            V: Copy,
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

        #[inline]
        pub const fn is_reflexive(&self) -> bool
        {
            self.degree == 1
        }

        #[inline]
        pub fn is_empty(&self) -> bool
        {
            self.vertices.is_empty()
        }

        #[inline]
        pub fn len(&self) -> usize
        {
            self.vertices.len()
        }
    }

    impl<V, F> std::fmt::Display for Face<V, F>
    where
        V: std::fmt::Display,
        F: std::fmt::Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        {
            let vertices_as_strings: Vec<String> =
                self.vertices.iter().map(ToString::to_string).collect();
            write!(
                f,
                "{} = ({}); deg = {}",
                self.label,
                vertices_as_strings.join(" "),
                self.degree
            )
        }
    }
    impl<V, F> std::fmt::Binary for Face<V, F>
    where
        V: std::fmt::Binary,
        F: std::fmt::Binary,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        {
            let vertices_as_strings: Vec<String> =
                self.vertices.iter().map(|v| format!("{v:b}")).collect();
            write!(
                f,
                "{:b} = ({}); deg = {}",
                self.label,
                vertices_as_strings.join(" "),
                self.degree
            )
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Wake
    {
        pub angle0: IntAngle,
        pub angle1: IntAngle,
    }

    impl Wake
    {
        #[must_use]
        pub fn is_real(&self) -> bool
        {
            self.angle0 + self.angle1 == MAX_ANGLE.get()
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
                write!(f, "{:0>width$b} <-> {:0>width$b}", self.angle0, self.angle1)
            } else {
                write!(f, "{:b} <-> {:b}", self.angle0, self.angle1)
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Edge<V>
    {
        pub start: V,
        pub end: V,
        pub wake: Wake,
    }

    impl<V> Edge<V>
    {
        #[inline]
        pub fn is_real(&self) -> bool
        {
            self.wake.is_real()
        }

        #[inline]
        fn connector(&self) -> &str
        {
            if self.is_real() {
                "==="
            } else {
                "---"
            }
        }
    }

    impl<V> std::fmt::Display for Edge<V>
    where
        V: std::fmt::Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        {
            let ks = AbstractPoint::new(self.wake.angle0).kneading_sequence();
            let connector = self.connector();
            write!(
                f,
                "{:>digits$} {connector} {:<digits$} \twake: {:digits$} \tKS = {ks:>period$}",
                self.start,
                self.end,
                self.wake,
                digits = (PERIOD.get() / 3 + 1) as usize,
                period = PERIOD.get() as usize
            )
        }
    }

    impl<V> std::fmt::Binary for Edge<V>
    where
        V: std::fmt::Binary,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        {
            let ks = AbstractPoint::new(self.wake.angle0).kneading_sequence();
            write!(
                f,
                "{:b} -- {:b}   wake = {wake:period$b}   KS = {ks:>period$}",
                self.start,
                self.end,
                wake = self.wake,
                period = PERIOD.get() as usize
            )
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum HalfPlane
    {
        Upper,
        Lower,
        #[default]
        PosReal,
        NegReal,
    }

    impl From<IntAngle> for HalfPlane
    {
        fn from(angle: IntAngle) -> Self
        {
            use std::cmp::Ordering::*;
            match (angle * 2).cmp(&MAX_ANGLE.get()) {
                Less => Self::Upper,
                Equal => Self::NegReal,
                Greater => Self::Lower,
            }
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum VertexData
    {
        PosReal,
        NegReal,
        PosNeg,
        NegPos,
        NegEdge,
        NegEdgePos,
        #[default]
        NonReal,
    }

    impl VertexData
    {
        pub const fn neg_edge(&self) -> bool
        {
            matches!(self, Self::NegEdge | Self::NegEdgePos)
        }
        pub const fn pos_vertex(&self) -> bool
        {
            matches!(
                self,
                Self::PosReal | Self::PosNeg | Self::NegPos | Self::NegEdgePos
            )
        }
        pub const fn neg_vertex(&self) -> bool
        {
            matches!(self, Self::NegReal | Self::PosNeg | Self::NegPos)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct AugmentedVertex<V>
    {
        pub vertex: V,
        pub data: VertexData,
    }

    impl<V> std::fmt::Display for AugmentedVertex<V>
    where
        V: std::fmt::Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        {
            use VertexData::*;
            match self.data {
                NonReal => self.vertex.fmt(f),
                PosReal => write!(f, "+{}", self.vertex),
                NegReal => write!(f, "-{}", self.vertex),
                PosNeg => write!(f, "+-{}", self.vertex),
                NegPos => write!(f, "-+{}", self.vertex),
                NegEdge => write!(f, "{} ===", self.vertex),
                NegEdgePos => write!(f, "+{} ===", self.vertex),
            }
        }
    }

    impl<V> std::fmt::Binary for AugmentedVertex<V>
    where
        V: std::fmt::Binary,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        {
            use VertexData::*;
            match self.data {
                NonReal => self.vertex.fmt(f),
                PosReal => write!(f, "+{:b}", self.vertex),
                NegReal => write!(f, "-{:b}", self.vertex),
                PosNeg => write!(f, "+-{:b}", self.vertex),
                NegPos => write!(f, "-+{:b}", self.vertex),
                NegEdge => write!(f, "{:b} ===", self.vertex),
                NegEdgePos => write!(f, "+{:b} ===", self.vertex),
            }
        }
    }
}
