//! Wordlist management functionality for the EZWordle2 package.

use crate::{
    MatchString,
    Word,
};

use indicatif::ProgressIterator;

/// All the words.
pub const ALLWORDS: &str = include_str!("../words.txt");

#[derive(Clone, Debug)]
/// Wordle wordlist.
pub struct Wordlist {
    /// All the words.
    pub words: Vec<Word>,
}

impl Wordlist {
    /// Construct a new wordlist.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A `Wordlist` containing all the words.
    pub fn new() -> Self {
        let words = ALLWORDS.split("\n")
            .filter(|w| !w.is_empty())
            .map(|w| w.try_into().unwrap())
            .collect::<Vec<Word>>();

        Self {
            words
        }
    }

    /// Compute the length of this wordlist.
    /// 
    /// _Note_: this return an `f64` as this is best for entropy
    /// calculations.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// The length of this wordlist as an `f64`.
    pub fn len(&self) -> f64 {
        self.words.len() as f64
    }

    /// Filter the wordlist by applying a word and its corresponding matchstring.
    /// 
    /// # Parameters
    /// - `word` (`&Word`): the word applied
    /// - `matchstring` (`&MatchString`): the resultant 
    /// matchstring
    /// 
    /// # Returns
    /// A `Wordlist` filtered by this word and its 
    /// corresponding matchstring.
    pub fn filter(&self, word: &Word, matchstring: &MatchString) -> Self {
        let words = self.words.iter()
            .filter(|w| word.check(*w) == *matchstring)
            .map(|w| *w)
            .collect::<Vec<Word>>();

        Self {
            words,
        }
    }

    /// Check if the wordlist contains the given word.
    /// 
    /// # Parameters
    /// - `word` (`&Word`): the word to be checked
    /// 
    /// # Returns
    /// A boolean value, indicating whether the word is contained in the wordlist.
    pub fn contains(&self, word: &Word) -> bool {
        self.words.contains(word)
    }

    /// Compute the word with the highest information entropy.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A `Word` with the highest information entropy.
    pub fn guess(&self) -> Word {
        let mut sorted_words = Vec::new();

        for word in self.words.iter().progress() {
            sorted_words.push((word, word.entropy(self)));
        }

        sorted_words.sort_by(|x, y| y.1.partial_cmp(&x.1).unwrap());

        *sorted_words[0].0
    }
}