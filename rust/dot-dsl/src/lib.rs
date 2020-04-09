pub mod graph {
    use std::collections::HashMap;
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct Node {
                pub item: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(item: &str) -> Self {
                    Node {
                        item: item.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Node {
                        attrs: attrs
                            .iter()
                            .map(|(k, v)| (String::from(*k), String::from(*v)))
                            .collect(),
                        ..self
                    }
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|v| v.as_str())
                }
            }
        }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Edge {
                        attrs: attrs
                            .iter()
                            .map(|(k, v)| (String::from(*k), String::from(*v)))
                            .collect(),
                        ..self
                    }
                }
            }
        }
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(self, nodes: &[graph_items::node::Node]) -> Self {
            Graph {
                nodes: nodes.to_vec(),
                ..self
            }
        }

        pub fn with_edges(self, edges: &[graph_items::edge::Edge]) -> Self {
            Graph {
                edges: edges.to_vec(),
                ..self
            }
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            Graph {
                attrs: attrs
                    .iter()
                    .map(|(k, v)| (String::from(*k), String::from(*v)))
                    .collect(),
                ..self
            }
        }

        pub fn get_node(&self, item: &str) -> Option<&graph_items::node::Node> {
            self.nodes.iter().find(|node| node.item == item)
        }
    }
}
