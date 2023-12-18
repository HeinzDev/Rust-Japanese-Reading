use std::collections::HashMap;
use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead};

fn load_words() -> (HashMap<usize, String>, HashMap<usize, String>) {
    let mut japanese = HashMap::new();
    let mut romaji = HashMap::new();

    if let Ok(lines) = read_lines("words.txt") {
        for (index, line) in lines.enumerate() {
            if let Ok(entry) = line {
                let mut parts = entry.split(',');
                if let (Some(japanese_word), Some(romaji_word)) = (parts.next(), parts.next()) {
                    japanese.insert(index + 1, japanese_word.trim().to_string());
                    romaji.insert(index + 1, romaji_word.trim().to_string());
                }
            }
        }
    }

    (japanese, romaji)
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut input = String::new();

    let (japanese, romaji) = load_words();
    let word_index = rand::thread_rng().gen_range(1..=japanese.len());
    let mut jap_word = String::new();
    let mut rom_word = String::new();

    match japanese.get(&word_index) {
        Some(value) => jap_word = value.to_string(),
        None => println!("err"),
    }

    match romaji.get(&word_index) {
        Some(value) => rom_word = value.to_string(),
        None => println!("err"),
    }

    println!("What does [{}] it mean?", jap_word);
    io::stdin().read_line(&mut input).expect("asdsa");

    if input.trim().to_lowercase() == rom_word.trim().to_lowercase() {
        println!("");
        println!("perfect!!");
    } else {
        println!("");
        println!("Wrong.");
        println!("(the word is: '{}')", rom_word);
    }
}
