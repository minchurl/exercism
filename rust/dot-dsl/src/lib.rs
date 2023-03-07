macro_rules! impl_attrs {
    () => {
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for &(key, value) in attrs {
                self.attrs.insert(key.to_string(), value.to_string());
            }
            self
        }

        pub fn attr(&self, name: &str) -> Option<&str>{
            self.attrs.get(name).map(|x| x.as_str())
        }
    };
}

pub mod graph {
    use std::collections::HashMap;
    use graph_items::{node::Node, edge::Edge};

    #[derive(Clone, PartialEq, Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>, 
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(), 
                edges: Vec::new(), 
                attrs: HashMap::new(), 
            }
        }
        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Graph {
            self.nodes.extend_from_slice(nodes);
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Graph {
            self.edges.extend_from_slice(edges);
            self
        }

        impl_attrs!();

        pub fn node(&self, node: &str) -> Result<&Node, &str> {
            match self.nodes.iter().find(|&x| x.name.eq(node)) {
                None => Err("not exist"), 
                Some(x) => Ok(x), 
            }
        }
    }

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;
            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                pub name: String, 
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(), 
                        attrs: HashMap::new(),
                    }
                }
                impl_attrs!();
            }
        }

        pub mod edge {
            use std::collections::HashMap;
            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                pub node_name: (String, String), 
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(node_a: &str, node_b: &str) -> Self {
                    Edge {
                        node_name: (node_a.to_string(), node_b.to_string()), 
                        attrs: HashMap::new(),
                    }
                }
                impl_attrs!();
            }
        }

    }

}
