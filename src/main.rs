use std::fs;
use std::io;

enum Word {
    Exist(String),
    DoenstExist,
}

fn search_word(dictionary: &String, word: &str) -> Word
{
    let word_length = word.len();
    for line in dictionary.lines(){
        if line.len() <= word_length
        {
            continue;
        }

        let first_letter = line.chars()
            .next().expect("Code not filtered properly, line length is 0");
        let searched_first_letter = line.chars()
            .next().expect("Code not filtered properly, search word length is 0");

        if first_letter != searched_first_letter{
            continue;
        }

        //Safely get and compare line word
        let line_word = line.get(0..word_length);
        match line_word {
            None => (),
            Some(w) => {
                if w == word
                {
                    let line_length = line.len();
                    let info_text = line.get(word_length + 2..line_length);
                    match info_text{
                        None => return Word::Exist(String::from
                            ("word exists but no information about it could be found")),
                        Some(txt) => return Word::Exist(String::from(txt)),
                    }
                }
            },
        }
    }
    Word::DoenstExist
}

fn filter_dictionary(path : &str)
{
    //Any dictionary name refers to the dictionary in text string
    let dictionary = fs::read_to_string(path).expect("Could not find dictionary file");

    let mut optimized_dictionary = String::new();
    for line in dictionary.lines(){
        if line.len() == 0
        {
            continue;
        }
        optimized_dictionary.push_str(line);
        optimized_dictionary.push_str("\n");
    }

    fs::write("res/optimized_dictionary.txt", optimized_dictionary)
        .expect("Failed to create new dictionary file");
}

fn main() -> std::io::Result<()> {
    let mut input_reader = String::new();
    let dictionary = fs::read_to_string("res/optimized_dictionary.txt")
        .expect("Could not find dictionary file");
    let dictionary = dictionary.trim().to_string().to_lowercase();
    println!("Type any word to get information about it\nType -exit to quit the program");
    loop {
        input_reader.clear();
        io::stdin()
            .read_line(&mut input_reader)
            .expect("Failed to read input");
        match input_reader.trim() {
            "-exit" => break,
            _ => {
                let input = input_reader.trim().to_string();
                //Checks if the user has given no input
                if input.len() == 0 {
                    println!("!No valid input");
                    continue;
                }
                //Checks if the user has given an invalid command
                if input.chars().next().expect("Failed to filter input when reading input") == '-' {
                    println!("!Invalid command");
                    continue;
                }
                let word = input.to_lowercase();
                let result = search_word(&dictionary, word.as_str());
                match result {
                    Word::DoenstExist => println!("The word doesn't exists"),
                    Word::Exist(info_text) => println!("{}", info_text),
                }
            },
        }
    }
    Ok(())
}
