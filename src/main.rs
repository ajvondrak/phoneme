#![allow(dead_code)]
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

    fn phone_words(&self, phone_number: &str) {
        let digits: Vec<_> = phone_number.chars().collect();
        let mut stack: Vec<(String, NodeId, usize)> = Vec::new();
        stack.push((String::new(), 0, 0));

        while let Some((phone_word, id, d)) = stack.pop() {
            if d == digits.len() {
                if self.nodes[id].terminal {
                    println!("{}", phone_word);
                }
                continue;
            }
            let digit = digits[d];
            for letter in t9_letters(digit) {
                if let Some(&next_id) = self.nodes[id].edges.get(letter) {
                    let mut next_phone_word = phone_word.clone();
                    next_phone_word.push(*letter);
                    stack.push((next_phone_word, next_id, d + 1));
                }
                if self.nodes[id].terminal
                    && let Some(&next_id) = self.nodes[0].edges.get(letter)
                {
                    let mut next_phone_word = phone_word.clone();
                    next_phone_word.push(' ');
                    next_phone_word.push(*letter);
                    stack.push((next_phone_word, next_id, d + 1));
                }
            }
        }
    }
}

static ONE: &[char] = &[];
static ABC: &[char] = &['a', 'b', 'c'];
static DEF: &[char] = &['d', 'e', 'f'];
static GHI: &[char] = &['g', 'h', 'i'];
static JKL: &[char] = &['j', 'k', 'l'];
static MNO: &[char] = &['m', 'n', 'o'];
static PQRS: &[char] = &['p', 'q', 'r', 's'];
static TUV: &[char] = &['t', 'u', 'v'];
static WXYZ: &[char] = &['w', 'x', 'y', 'z'];
static ZERO: &[char] = &[];

fn t9_letters(digit: char) -> &'static [char] {
    match digit {
        '1' => ONE,
        '2' => ABC,
        '3' => DEF,
        '4' => GHI,
        '5' => JKL,
        '6' => MNO,
        '7' => PQRS,
        '8' => TUV,
        '9' => WXYZ,
        '0' => ZERO,
        _ => panic!("not a digit: {:?}", digit),
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
    let path = Path::new("/usr/share/dict/words");
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    let mut trie = Trie::new();
    for word in reader.lines().map_while(Result::ok) {
        if word.len() > 1 && word.chars().all(|c| c.is_alphabetic() && c.is_lowercase()) {
            trie.insert(&word);
        };
    }

    trie.phone_words("8737878"); // "use rust", but also some others like "us erupt", "user tst"
}
