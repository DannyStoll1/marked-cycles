use crate::types::Period;

#[derive(Debug, PartialEq)]
pub struct Face {
    pub vertices: Vec<Period>,
    pub degree: Period,
}

impl Face {
    pub fn edges(&self) -> Vec<(Period, Period)> {
        let mut edges = Vec::new();
        for i in 0..self.vertices.len() {
            edges.push((
                self.vertices[i],
                self.vertices[(i + 1) % self.vertices.len()],
            ));
        }
        edges
    }

    pub fn len(&self) -> usize {
        self.vertices.len()
    }
}

#[derive(Debug, PartialEq)]
pub struct Edge {
    pub angles: (Period, Period),
    pub endpoints: (Period, Period),
    pub period: Period,
    pub degree: Period,
}

impl Edge {
    pub fn is_real(&self) -> bool {
        self.angles.0 + self.angles.1 == self.degree.pow(self.period .try_into().unwrap()) - 1
    }

    pub fn to_string(&self) -> String {
        let n = self.period as usize;
        let m = (self.degree.pow(self.period .try_into().unwrap())).to_string().len();
        let t = self.angles.0;
        let a = self.endpoints.0;
        let b = self.endpoints.1;
        format!("{:>n$b} = {:>m$} -- {:<m$}", t, a, b, n = n, m = m)
    }
}
