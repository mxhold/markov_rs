extern crate rand;
use rand::{weak_rng, Rng, XorShiftRng, SeedableRng};
use std::env;
use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer);

    let args: Vec<String> = env::args().collect();

    let max_words: u32 = args[1].parse().unwrap();
    let output: String = generate(buffer.trim(), max_words, [0, 0, 0, 0]);

    println!("{}", output);
}

fn parse(string: &str) -> HashMap<(&str, &str), Vec<&str>> {
    let mut table: HashMap<(&str, &str), Vec<&str>> = HashMap::new();

    let words: Vec<&str> = string.split(" ").collect();
    let grouped_words = words.windows(3);

    for word_group in grouped_words {
        let prefix = (word_group[0], word_group[1]);
        let suffix = word_group[2];

        let new_suffix = match table.get(&prefix) {
            Some(existing_words) => {
                let mut new_words = existing_words.clone();
                new_words.push(suffix);
                new_words
            },
            None => vec![suffix],
        };

        table.insert(prefix, new_suffix);
    }

    table
}

fn generate(string: &str, max_words: u32, seed: [u32; 4]) -> String {
    let table = parse(string);

    let mut rng: XorShiftRng = if &seed == &[0, 0, 0, 0] {
        weak_rng()
    } else {
        SeedableRng::from_seed(seed)
    };

    let mut possible_prefixes: Vec<&(&str, &str)> = table.keys().collect();
    possible_prefixes.sort();
    let prefix: &(&str, &str) = rng.choose(&possible_prefixes).unwrap();
    let &(mut word1, mut word2) = prefix;

    let mut result = word1.to_string() + " " + word2;

    for _ in 1..(max_words - 1) {
        match table.get(&(word1, word2)) {
            Some(suffixes) => {
                word1 = word2;
                word2 = rng.choose(&suffixes).unwrap();
                result = result + " " + word2;
            },
            None => {
                break;
            },
        }
    }

    result
}

#[test]
fn test_parse() {
    let table = parse("I like cake. I like pie.");

    assert_eq!(table.get(&("I", "like")), Some(&vec!["cake.", "pie."]));
    assert_eq!(table.get(&("like", "cake.")), Some(&vec!["I"]));
    assert_eq!(table.get(&("cake.", "I")), Some(&vec!["like"]));
}

#[test]
fn test_generate() {
    let result = generate("I like cake. I like pie.", 6, [13, 84, 433, 33]);

    assert_eq!(result, "I like cake. I like cake.");
}
