use clap::Parser;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, prelude::BufRead},
    path::{Path, PathBuf},
};

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

    fn read<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let reader = BufReader::new(File::open(&path)?);
        let mut trie = Self::new();
        for word in reader.lines() {
            trie.insert(&word?);
        }
        Ok(trie)
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

fn phone_number(s: &str) -> Result<String, String> {
    if s.chars().all(|d| d.is_ascii_digit()) {
        Ok(s.to_string())
    } else {
        Err("must be a string of digits 0-9".to_string())
    }
}

/// Searches for phone words that can be spelled from a given phone number
#[derive(clap::Parser)]
struct Args {
    /// Path to dictionary file, one word per line
    #[arg(short, long, default_value = "/usr/share/dict/words")]
    dict: PathBuf,

    /// Phone number to match, any string of digits 0-9
    #[arg(value_parser = phone_number)]
    digits: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let trie = Trie::read(&args.dict)
        .map_err(|err| format!("failed to read {}: {}", args.dict.display(), err))?;

    let digits: Vec<_> = args.digits.chars().collect();

    let mut pending = vec![(String::new(), Trie::ROOT, 0)];

    while let Some((phone_word, node, d)) = pending.pop() {
        if d == digits.len() {
            if trie.is_terminal(node) {
                println!("{}", phone_word);
            }
            continue;
        }
        let digit = digits[d];
        for &letter in phone_letters(digit) {
            if let Some(&next_node) = trie.next(node, letter) {
                let mut next_phone_word = phone_word.clone();
                next_phone_word.push(letter);
                pending.push((next_phone_word, next_node, d + 1));
            }
            if trie.is_terminal(node)
                && let Some(&next_node) = trie.next(Trie::ROOT, letter)
            {
                let mut next_phone_word = phone_word.clone();
                next_phone_word.push(' ');
                next_phone_word.push(letter);
                pending.push((next_phone_word, next_node, d + 1));
            }
        }
    }

    Ok(())
}
