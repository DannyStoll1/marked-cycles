use crate::global_state::{MAX_ANGLE, PERIOD};
use crate::types::IntAngle;

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
        global_state::PERIOD,
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
    impl<V, F> std::fmt::Binary for Face<V, F>
    where
        V: std::fmt::Binary,
        F: std::fmt::Binary,
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

    #[derive(Debug, Clone, PartialEq, Eq)]
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

    #[derive(Debug, PartialEq, Eq)]
    pub struct Edge<V>
    {
        pub start: V,
        pub end: V,
        pub wake: Wake,
    }

    impl<V> Edge<V> {}

    impl<V> std::fmt::Display for Edge<V>
    where
        V: std::fmt::Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        {
            let ks = AbstractPoint::new(self.wake.angle0).kneading_sequence();
            write!(
                f,
                "{:>digits$} -- {:<digits$} \twake: {:digits$} \tKS = {ks:>period$}",
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
                "{:b} -- {:b}   wake = {wake:b}   KS = {ks:>period$}",
                self.start,
                self.end,
                wake = self.wake,
                period = PERIOD.get() as usize
            )
        }
    }
}
