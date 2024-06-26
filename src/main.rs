use std::io;
use random_word::Lang;

fn main() {
    let mut stage= 0;
    let stages = [
    ["", "", "", "", "", "" , ""], 
    //["", "", "", "", "", " /" , "/  "], 
    ["", "", "", "", "", r#" /\"# , r#"/  \"#],
    ["", " ||", " ||", " ||", " ||", r#" /\"# , r#"/  \"#], 
    [" ___________", " ||", " ||", " ||", " ||", r#" /\"# , r#"/  \"#], 
    [" ___________", " || /", " ||/", " ||", " ||", r#" /\"# , r#"/  \"#], 
    //[" ___________", " || /      |", " ||/", " ||", " ||", r#" /\"# , r#"/  \"#], 
    [" ___________", " || /      |", " ||/       O", " ||", " ||", r#" /\"# , r#"/  \"#], 
    [" ___________", " || /      |", " ||/       O", " ||        |", " ||", r#" /\"# , r#"/  \"#], 
    //[" ___________", " || /      |", " ||/       O", " ||       /|", " ||", r#" /\"# , r#"/  \"#], 
    [" ___________", " || /      |", " ||/       O", r#" ||       /|\"#, " ||", r#" /\"# , r#"/  \"#], 
    //[" ___________", " || /      |", " ||/       O", r#" ||       /|\"#, " ||       / ", r#" /\"# , r#"/  \"#], 
    [" ___________", " || /      |", " ||/       O", r#" ||       /|\"#, r#" ||       / \"#, r#" /\"# , r#"/  \"#]
    ];

    let hidden_word = random_word::gen(Lang::En);
    let mut revealed_word = "".to_owned();
    let mut guessed_letters = "".to_owned();
    revealed_word = create_revealed_word(hidden_word, revealed_word);

    loop {
        if revealed_word == hidden_word {
            print_string(&revealed_word);
            println!("");
            println!("YOU WON, THE WORD WAS {hidden_word}");
            break;
        }
        for instance in stages[stage] {
            println!("{}", instance);
        }
        if stage == 12 {
            println!("YOU KILLED KENNY, YOU BASTARD!!!");
            break;
        }
        print_string(&revealed_word);
        println!("");
        print_string(&guessed_letters);
        println!("");

        loop {
            let mut guess = String::new();
            let mut isunique = true;
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            
            let guess: char = match guess.trim().parse() {
                Ok(char) => char,
                Err(_) => continue,
            };
            for letter in guessed_letters.chars(){
                if guess == letter{
                    println!("Letter already guessed, try again.");
                    isunique = false;
                }
            }

            if isunique {
                if letternotin(guess, hidden_word) {
                    stage += 1;
                }
                guessed_letters.push(guess);
                break;
            }
        }

        revealed_word = reveal_letter(guessed_letters.clone(), hidden_word);

    }
}

fn create_revealed_word(hidden_word: &str, mut revealed_word: String)-> String{
    for _ in hidden_word.chars(){
        revealed_word.push('_');
    }
    return revealed_word;
}

fn reveal_letter(guessed_letters: String, hidden_word: &str) -> String {
    let mut revealed_word = "".to_owned();
    for letter in hidden_word.chars(){
        let mut count = 0;

        for guessedletter in guessed_letters.chars(){
            if letter == guessedletter {
                revealed_word.push(letter);
                continue;
            } 
            count += 1;          
        }
        if count == guessed_letters.len(){
            revealed_word.push('_');
        }
    }
    return revealed_word;
}

fn print_string(word: &str) {
    for letter in word.chars(){
        print!("{letter} ");
    }
}

fn letternotin(letter: char, hidden_word: &str) -> bool {
    for character in hidden_word.chars(){
        if character == letter {
            return false;
        }
    }
    return true;
}