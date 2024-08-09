//! Main executable for the EZWordle2 package.

use std::io::{
    self,
    Write,
};

use ezwordle2::{
    Word,
    Wordlist,
    MatchString,
};

fn main() {
    // Initialize EZWordle backend
    println!("\nInitializing...");

    // Construct wordlist
    let mut wordlist = Wordlist::new();

    // Store the current best guess
    let mut guess: String = "raise".to_string();

    // Give the first guess & get feedback
    if update(&mut wordlist, &guess) {
        println!("\nGreat job!\n");
        return;
    }

    for _ in 0..5 {
        // Compute the next best guess
        let guess_word = &wordlist.guess();
        guess = guess_word.to_string();
        
        // Give the next guess & get feedback
        if update(&mut wordlist, &guess) {
            println!("\nGreat job!\n");
            return;
        }
    }
}

/// Update the wordlist based on a given guess, returning the victory condition.
fn update(wordlist: &mut Wordlist, guess: &str) -> bool {
    println!("\nI guess {}\n", guess.to_uppercase());

    // Get user guess
    let mut user_guess = get_guess();
    
    while {
        if let Ok (word) = TryInto::<Word>::try_into(user_guess.as_str()) {
            !wordlist.contains(&word)
        } else {
            true
        }
    } {
        // Get user guess again
        user_guess = get_guess();
    }

    // Convert this into a word
    let user_word: Word = user_guess.as_str().try_into().unwrap();

    // Get user matchstring
    let mut user_matchstring = get_matchstring();

    while TryInto::<MatchString>::try_into(user_matchstring.to_owned()).is_err() {
        // Get user guess again
        user_matchstring = get_matchstring();
    }

    // Convert this into a matchstring
    let user_matchstring: MatchString = user_matchstring.try_into().unwrap();

    // Update the wordlist
    *wordlist = wordlist.filter(&user_word, &user_matchstring);

    user_matchstring.check()
}

/// Get the user's guess
fn get_guess() -> String {
    // Get user guess
    let mut user_guess = String::new();

    // Ask the user
    print!("What do you guess? >> ");

    // Flush the output
    io::stdout().flush().unwrap();

    // Read in the input line
    io::stdin().read_line(&mut user_guess).expect("Could not read input");

    user_guess.truncate(5);

    user_guess.to_lowercase()
}

/// Get the user's matchstring
fn get_matchstring() -> String {
    // Get user guess
    let mut user_guess = String::new();

    // Ask the user
    print!("How did you do? >> ");

    // Flush the output
    io::stdout().flush().unwrap();

    // Read in the input line
    io::stdin().read_line(&mut user_guess).expect("Could not read input");

    user_guess.truncate(5);

    user_guess
}