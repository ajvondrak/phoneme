use std::{collections::HashMap, fs::File, io::BufReader, io::prelude::BufRead, path::Path};

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
    const ROOT: NodeId = 0;

    fn new() -> Self {
        let root = Node::default();
        let nodes = vec![root];
        Self { nodes }
    }

    fn next(&self, id: NodeId, c: char) -> Option<&NodeId> {
        self.nodes.get(id)?.edges.get(&c)
    }

    fn is_terminal(&self, id: NodeId) -> bool {
        self.nodes.get(id).is_some_and(|node| node.terminal)
    }

    fn insert(&mut self, word: &str) {
        let mut id = Self::ROOT;
        for c in word.chars() {
            match self.next(id, c) {
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
}

fn phone_letters(digit: char) -> &'static [char] {
    match digit {
        '2' => &['a', 'b', 'c'],
        '3' => &['d', 'e', 'f'],
        '4' => &['g', 'h', 'i'],
        '5' => &['j', 'k', 'l'],
        '6' => &['m', 'n', 'o'],
        '7' => &['p', 'q', 'r', 's'],
        '8' => &['t', 'u', 'v'],
        '9' => &['w', 'x', 'y', 'z'],
        _ => &[],
    }
}

fn main() {
    let path = Path::new("/usr/share/dict/words"); // TODO: optionally get this from argv
    let file = match File::open(path) {
        Err(err) => panic!("couldn't open {}: {}", path.display(), err),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    let mut words = Trie::new();
    for word in reader.lines().map_while(Result::ok) {
        if word.len() > 1 && word.chars().all(|c| c.is_alphabetic() && c.is_lowercase()) {
            words.insert(&word);
        };
    }

    let phone_number = "8737878"; // TODO: get this from argv

    let digits: Vec<_> = phone_number.chars().collect();
    let mut stack = vec![(String::new(), Trie::ROOT, 0)];

    while let Some((phone_word, node, d)) = stack.pop() {
        if d == digits.len() {
            if words.is_terminal(node) {
                println!("{}", phone_word);
            }
            continue;
        }
        let digit = digits[d];
        for &letter in phone_letters(digit) {
            if let Some(&next_node) = words.next(node, letter) {
                let mut next_phone_word = phone_word.clone();
                next_phone_word.push(letter);
                stack.push((next_phone_word, next_node, d + 1));
            }
            if words.is_terminal(node)
                && let Some(&next_node) = words.next(Trie::ROOT, letter)
            {
                let mut next_phone_word = phone_word.clone();
                next_phone_word.push(' ');
                next_phone_word.push(letter);
                stack.push((next_phone_word, next_node, d + 1));
            }
        }
    }
}
