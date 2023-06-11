use std::io;

fn main() {
    //Get a string from the user:
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");


    //Convert the string to pig latin
    let pig_latin = convert_to_pig_latin(&input);
    println!("Pig latin: {}", pig_latin);
}

fn convert_to_pig_latin(input: &str) -> String {
    //Split the string into words
    let words = input.split_whitespace();

    //Convert each word to pig latin
    let mut pig_latin_words = Vec::new();
    for word in words {
        let pig_latin_word = pig_latin(word);
        pig_latin_words.push(pig_latin_word);
    }

    //Join the words back together
    let pig_latin = pig_latin_words.join(" ");
    pig_latin
}

fn pig_latin(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    if vowels.contains(&word.chars().next().unwrap()) {
        format!("{}-hay", word)
    } else {
        let split_index = word.chars().position(|c| vowels.contains(&c)).unwrap_or_else(|| word.len());
        let (start, rest) = word.split_at(split_index);
        format!("{}-{}ay", rest, start)
    }
}

