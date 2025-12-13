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

    fn insert(&mut self, word: &str) {
        let mut id = 0;
        for c in word.chars() {
            match self.nodes[id].edges.get(&c) {
                Some(next) => id = *next,
                None => {
                    let next = self.nodes.len();
                    self.nodes.push(Node::default());
                    self.nodes[id].edges.insert(c, next);
                    id = next;
                }
            }
        }
        self.nodes[id].terminal = true;
    }

    fn contains(&self, word: &str) -> bool {
        let mut id = 0;
        for c in word.chars() {
            match self.nodes[id].edges.get(&c) {
                Some(next) => id = *next,
                None => return false,
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
