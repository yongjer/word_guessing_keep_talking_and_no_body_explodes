use std::collections::HashSet;
use std::io;

fn main() {
    let word_set: HashSet<&str> = [
        "about", "after", "again", "below", "could", "every", "first", "found", "great", "house",
        "large", "learn", "never", "other", "place", "plant", "point", "right", "small", "sound",
        "spell", "still", "study", "their", "there", "these", "thing", "think", "three", "water",
        "where", "which", "world", "would", "write",
    ]
    .iter()
    .cloned()
    .collect();

    println!("Enter two strings separated by a space:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let inputs: Vec<&str> = input.trim().split_whitespace().collect();
    if inputs.len() != 2 {
        println!("Please enter exactly two strings separated by a space.");
        return;
    }

    let first_string = inputs[0].to_lowercase();
    let second_string = inputs[1].to_lowercase();

    let possible_words: Vec<&str> = word_set
        .iter()
        .filter(|&word| {
            first_string.contains(word.chars().next().unwrap())
                && second_string.contains(word.chars().last().unwrap())
        })
        .cloned()
        .collect();

    if possible_words.is_empty() {
        println!("No words found matching the criteria.");
    } else {
        println!("Possible words:");
        for word in possible_words {
            println!("{}", word);
        }
    }
}
