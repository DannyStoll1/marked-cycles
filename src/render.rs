fn generate_ngon_svg(point_labels: Vec<&str>, r: f32) -> String
{
    let n = point_labels.len();
    let center = (r + 10.0, r + 10.0); // Add some margin
    let mut paths = Vec::new();

    for i in 0..n {
        let angle = 2.0 * std::f32::consts::PI * (i as f32) / (n as f32);
        let x = center.0 + r * angle.cos();
        let y = center.1 + r * angle.sin();

        // For simplicity, add lines to the paths; in practice, you may want to form a single polygon path
        let next_angle = 2.0 * std::f32::consts::PI * ((i + 1) % n as usize) as f32 / (n as f32);
        let x_next = center.0 + r * next_angle.cos();
        let y_next = center.1 + r * next_angle.sin();

        paths.push(format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" style=\"stroke:black;stroke-width:2\" />", x, y, x_next, y_next));

        // Add text labels
        let label_x = center.0 + (r + 10.0) * angle.cos(); // Adjust the 10.0 to position the labels
        let label_y = center.1 + (r + 10.0) * angle.sin(); // Adjust the 10.0 to position the labels
        paths.push(format!(
            "<text x=\"{}\" y=\"{}\" style=\"font-family:Arial;font-size:10px;\">{}</text>",
            label_x, label_y, point_labels[i]
        ));
    }

    // Combine everything into an SVG
    format!(
        "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">{}</svg>",
        2.0 * (r + 20.0),
        2.0 * (r + 20.0),
        paths.join("")
    )
}
