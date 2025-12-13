#![allow(dead_code)]
use std::collections::HashMap;

type NodeId = usize;

struct Trie {
    nodes: Vec<Node>,
}

#[derive(Default)]
struct Node {
    edges: HashMap<char, NodeId>,
    terminal: bool,
}

impl Trie {
    fn new() -> Self {
        let root = Node::default();
        let nodes = vec![root];
        Self { nodes }
    }

    fn add_node(&mut self) -> NodeId {
        let id = self.nodes.len();
        self.nodes.push(Node::default());
        id
    }

    fn add_edge(&mut self, c: char, src: NodeId, dst: NodeId) {
        if let Some(node) = self.nodes.get_mut(src) {
            node.edges.insert(c, dst);
        };
    }

    pub fn insert(&mut self, word: &str) {
        let mut id = 0;
        for c in word.chars() {
            match self.nodes[id].edges.get(&c) {
                Some(next) => {
                    id = *next;
                }
                None => {
                    let next = self.add_node();
                    self.add_edge(c, id, next);
                    id = next;
                }
            }
        }
        if let Some(node) = self.nodes.get_mut(id) {
            node.terminal = true;
        }
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut id = 0;
        for c in word.chars() {
            match self.nodes[id].edges.get(&c) {
                Some(next) => {
                    id = *next;
                }
                None => {
                    return false;
                }
            }
        }
        self.nodes[id].terminal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trie_basics() {
        let mut trie = Trie::new();
        trie.insert("cat");
        trie.insert("cap");
        trie.insert("cape");
        trie.insert("caper");

        assert!(trie.contains("cat"));
        assert!(trie.contains("cap"));
        assert!(trie.contains("cape"));
        assert!(trie.contains("caper"));
        assert!(!trie.contains("car"));
        assert!(!trie.contains("catch"));
        assert!(!trie.contains("calm"));
        assert!(!trie.contains("dog"));
    }
}

fn main() {
    println!("Hello, world!");
}
