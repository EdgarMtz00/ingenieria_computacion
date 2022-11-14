use crate::symbols::Token;

pub struct Node {
    pub token: Token,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(token: Token) -> Node {
        Node {
            token,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn print(&self, level: usize) {
        let mut indent = String::new();
        for _ in 0..level {
            indent.push_str("  ");
        }
        println!("{}{:?}", indent, self.token);
        for child in &self.children {
            child.print(level + 1);
        }
    }
}