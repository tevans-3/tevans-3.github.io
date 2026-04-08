struct Node {
    id: String,
    content: String,
    shape: String,
}

pub struct Diagram {
    nodes: Vec<Node>,
    edges: Vec<(String, String)>,
    counter: usize,
}

impl Diagram {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            counter: 0,
        }
    }

    pub fn add(mut self, content: &str, shape: &str) -> Self {
        self.counter += 1;
        self.nodes.push(Node {
            id: format!("Shape{}", self.counter),
            content: content.to_string(),
            shape: shape.to_string(),
        });
        self
    }

    pub fn connect(mut self, from: &str, to: &str) -> Self {
        self.edges.push((from.to_string(), to.to_string()));
        self
    }

    fn tikz_style(shape: &str) -> &str {
        match shape {
            "diamond" => "draw, diamond, aspect=2, minimum width=2.5cm, minimum height=1cm",
            "rounded" => "draw, rounded corners, minimum width=2.5cm, minimum height=1cm",
            "circle" => "draw, circle, minimum size=1cm",
            "parallelogram" => {
                "draw, trapezium, trapezium left angle=70, trapezium right angle=110, minimum width=2.5cm, minimum height=1cm"
            }
            _ => "draw, rectangle, minimum width=2.5cm, minimum height=1cm",
        }
    }

    pub fn render(&self) -> String {
        let mut tikz = String::new();
        tikz.push_str("\\begin{tikzpicture}[node distance=2cm, auto]\n");

        for (i, node) in self.nodes.iter().enumerate() {
            let style = Self::tikz_style(&node.shape);
            if i == 0 {
                tikz.push_str(&format!(
                    "  \\node[{}] ({}) {{{}}};\n",
                    style, node.id, node.content
                ));
            } else {
                tikz.push_str(&format!(
                    "  \\node[{}, below of={}] ({}) {{{}}};\n",
                    style, self.nodes[i - 1].id, node.id, node.content
                ));
            }
        }

        for (from, to) in &self.edges {
            tikz.push_str(&format!("  \\draw[->] ({}) -- ({});\n", from, to));
        }

        tikz.push_str("\\end{tikzpicture}");
        tikz
    }
}
