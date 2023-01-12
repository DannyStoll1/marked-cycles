#[derive(Debug, PartialEq)]
struct Tessellation {
    faces: Vec<Face>,
    edges: Vec<(u32, u32)>,
    vertices: Vec<(f64, f64)>,
}

impl Tessellation {
    fn euler_characteristic(&self) -> isize {
        let chi =
            self.vertices.len() as isize - self.edges.len() as isize + self.faces.len() as isize;

        // It had better be even!
        assert_eq!(chi % 2, 0);

        chi
    }

    fn genus(&self) -> isize {
        1 - self.euler_characteristic() / 2
    }

    fn face_sizes(&self) -> Vec<usize> {
        self.faces.iter().map(|f| f.vertices.len()).collect()
    }

    fn num_odd_faces(&self) -> usize {
        self.face_sizes().iter().filter(|&s| s % 2 == 1).count()
    }
}
