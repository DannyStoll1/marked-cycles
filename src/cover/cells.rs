#[derive(Debug, PartialEq)]
pub struct Face {
    pub vertices: Vec<u32>,
    pub degree: u32,
}

impl Face {
    pub fn edges(&self) -> Vec<(u32, u32)> {
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
    pub angles: (u32, u32),
    pub endpoints: (u32, u32),
    pub period: u32,
    pub degree: u32,
}

impl Edge {
    pub fn is_real(&self) -> bool {
        self.angles.0 + self.angles.1 == self.degree.pow(self.period as u32) - 1
    }

    pub fn to_string(&self) -> String {
        let n = self.period as usize;
        let m = (self.degree.pow(self.period as u32)).to_string().len();
        let t = self.angles.0;
        let a = self.endpoints.0;
        let b = self.endpoints.1;
        format!("{:>n$b} = {:>m$} -- {:<m$}", t, a, b, n = n, m = m)
    }
}
