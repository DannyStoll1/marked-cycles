use crate::common::cells::{AugmentedVertex as Aug, Edge, Face};
use lazy_static::lazy_static;
use regex::Regex;
use std::{f32::consts::PI, fmt::Display};

lazy_static! {
    static ref RE_DEL: Regex = Regex::new(r"^\((.*)\)$").expect("Invalid regex");
    static ref RE_ABR: Regex = Regex::new(r"^<(.*)>$").expect("Invalid regex");
}

pub struct TikzRenderer<V, F>
{
    commands: Vec<String>,
    edges: Vec<Edge<V>>,
    faces: Vec<Face<Aug<V>, F>>,
}
impl<V, F> TikzRenderer<V, F>
where
    V: Display,
    F: Display,
{
    const EDGE_LENGTH: f32 = 1.46;

    // pub fn new(edges: Vec<Edge<V>>, faces: Vec<Face<V, F>>) -> Self
    #[must_use]
    pub fn new(faces: Vec<Face<Aug<V>, F>>) -> Self
    {
        let commands = vec![
            r"\begin{tikzpicture}".to_owned(),
            r"    \def\edgelength{1.8cm}".to_owned(),
        ];
        Self {
            commands,
            edges: Vec::new(),
            faces,
        }
    }

    fn draw_face(&mut self, face: &Face<Aug<V>, F>)
    {
        let n = face.len();

        let half_angle = PI / (n as f32);
        let radius = Self::EDGE_LENGTH / (2.0 * half_angle.sin());
        let offset_x = radius * half_angle.cos();

        self.commands.push("\n".to_owned());
        self.commands
            .push(format!(r"    \def\baseangle{{180/{n}}}"));
        self.commands
            .push(format!(r"    \def\anchorx{{{offset_x}}}"));
        self.commands.push(String::new());

        let face_str = face.label.to_string();
        let face_idx = RE_ABR.replace_all(&face_str, r"$1").to_string();
        let face_label = format!(r"$\abr{{{face_idx}}}$");
        let face_id = format!(r"(face{face_idx})");

        self.commands.push(format!(
            r"    \node {face_id} at (\anchorx, 0) {{{face_label}}};"
        ));

        let label = format!("{}", face.vertices[0].vertex);
        let label = RE_DEL.replace_all(&label, r"$\del{$1}$").to_string();
        self.commands.push(format!(
            r"    \node (node-{face_idx}-0) at (${face_id}+(\baseangle:{radius})$) {{{label}}};",
        ));

        for (i, node) in face.vertices.iter().enumerate().skip(1) {
            let label = node.vertex.to_string();
            let label = RE_DEL.replace_all(&label, r"$\del{$1}$").to_string();
            self.commands.push(format!(
                // r"    \node (node-{face_idx}-{i}) at ($(node-{face_idx}-{prev})+({{\baseangle - 90 - {i}*\anglestep}}:)$) {{{label}}};",
                r"    \node (node-{face_idx}-{i}) at ($(node-{face_idx}-{prev})+({angle} + \baseangle:{dist})$) {{{label}}};",
                angle = (-90. + (i as f32).mul_add(-360., 180.) / (n as f32)).rem_euclid(360.),
                dist = Self::EDGE_LENGTH,
                prev = i-1
            ));
        }

        // draw the edges between the nodes
        for i in 0..n {
            let next = (i + 1) % n;

            let data = face.vertices[i].data;

            if data.neg_edge() {
                self.commands.push(format!(
                    r"    \draw[double,double distance=2pt] (node-{face_idx}-{i}) -- (node-{face_idx}-{next});"
                ));
            } else {
                self.commands.push(format!(
                    r"    \draw (node-{face_idx}-{i}) -- (node-{face_idx}-{next});"
                ));
            }

            if data.pos_vertex() {
                self.commands.push(format!(
                    r"    \draw[dashed] (node-{face_idx}-{i}) -- {face_id};"
                ));
            }

            if data.neg_vertex() {
                self.commands.push(format!(
                    r"    \draw[dotted] (node-{face_idx}-{i}) -- {face_id};"
                ));
            }
        }
    }

    #[must_use]
    pub fn draw_largest_face(mut self) -> String
    {
        let max_size = self.faces.iter().map(Face::len).max().unwrap_or_default();
        let faces = std::mem::take(&mut self.faces);
        for f in &faces {
            if f.len() == max_size {
                self.draw_face(f);
                break;
            }
        }
        self.commands.push(r"\end{tikzpicture}".to_owned());
        self.commands.join("\n")
    }

    #[must_use]
    pub fn draw_smallest_face(mut self) -> String
    {
        let min_size = self.faces.iter().map(Face::len).min().unwrap_or_default();
        let faces = std::mem::take(&mut self.faces);
        for f in &faces {
            if f.len() == min_size {
                self.draw_face(f);
                break;
            }
        }
        self.commands.push(r"\end{tikzpicture}".to_owned());
        self.commands.join("\n")
    }

    #[must_use]
    pub fn generate(mut self) -> String
    {
        let faces = std::mem::take(&mut self.faces);
        for f in &faces {
            self.draw_face(f);
        }
        self.commands.push(r"\end{tikzpicture}".to_owned());
        self.commands.join("\n")
    }

    // fn draw_edge(&mut self, edge: Edge<V>) {
    //     todo!()
    // }
    //
    // fn layout_face(&mut self, face: Face<V, F>) {
    //     todo!()
    // }
}
