//! Word abstraction for the EZWordle2 package.

use std::fmt;

use crate::{
    Match,
    MatchString,
    Wordlist
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
/// A Wordle word.
pub struct Word (pub [char; 5]);

impl Word {
    /// Compare this word against a potential secret,
    /// generating the corresponding matchstring.
    /// 
    /// # Parameters
    /// - `secret` (`&Word`): the (potential) secret word
    /// to compare against
    /// 
    /// # Returns
    /// A `MatchString` corresponding to the corresponding
    /// matchstring.
    pub fn check(&self, secret: &Self) -> MatchString {
        let mut matchstring = [Match::Gray; 5];

        // "Reserve" all the correctly matching ones first
        for (i, check_letter) in self.0.iter().enumerate() {
            if *check_letter == secret.0[i] {
                matchstring[i] = Match::Green;
            }
        }

        // Go back and color yellow
        for (i, check_letter) in self.0.iter().enumerate() {
            // Don't recolor anything that's green
            if matchstring[i] == Match::Green {
                continue;
            }

            for (j, secret_letter) in secret.0.iter().enumerate() {
                // If the secret contains the letter to be checked,
                // and the check letter is not already marked green,
                // then it should be marked yellow
                if secret_letter == check_letter
                    && matchstring[j] != Match::Green
                {
                    matchstring[i] = Match::Yellow;
                }
            }
        }

        MatchString (matchstring)
    }

    /// Check if this word contains a given character.
    /// 
    /// # Parameters
    /// - `c` (`&char`): the character to check
    /// 
    /// # Returns
    /// A `bool` corresponding to whether this word
    /// contains the given character.
    pub fn contains(&self, c: &char) -> bool {
        for val in &self.0 {
            if val == c {
                return true;
            }
        }

        return false;
    }

    /// Compute the information entropy of this word,
    /// in bits, given a wordlist.
    ///
    /// # Parameters
    /// - `wordlist` (`&Wordlist`): a list of words
    ///
    /// # Returns
    /// An `f64` containing the amount of information entropy.
    pub fn entropy(&self, wordlist: &Wordlist) -> f64 {
        let mut entropy = 0.0;

        let all_matchstrings = MatchString::all();

        // Iterate over every possible matchstring
        for matchstring in all_matchstrings {
            entropy += self.contribution(wordlist, &matchstring);
        }

        entropy
    }

    /// Compute the contribution that the possible
    /// matchstring makes to the overall entropy of this
    /// word.
    ///
    /// # Parameters
    /// - `wordlist` (`&Wordlist`): a list of words
    /// - `matchstring` (`&MatchString`): a matchstring
    ///
    /// # Returns
    /// An `f64` representing the entropy contribution of
    /// this matchstring.
    pub fn contribution(&self, wordlist: &Wordlist, matchstring: &MatchString) -> f64 {
        // Construct a filtered wordlist in which every word obeys
        // the constraint imposed by this word's matchstring
        let filtered_wordlist = wordlist.filter(self, matchstring);

        // Compute the length of this wordlist
        let filtered_len = filtered_wordlist.len();

        // Compute the length of the original list
        let original_len = wordlist.len();

        // If we have any zeroes here, just return zero or we
        // might get some weird math here pretty soon...
        if filtered_len == 0.0 || original_len == 0.0 {
            return 0.0;
        }

        // Compute the probability that this matchstring will occur
        let p = filtered_len / original_len;

        // Return the Shannon entropy contribution based on the
        // probability that this matchstring will occur
        -p * p.log2()
    }
}

impl TryFrom<&str> for Word {
    type Error = fmt::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() == 5 {
            let mut ascii = s.chars();

            Ok (Self ([
                ascii.next().unwrap(),
                ascii.next().unwrap(),
                ascii.next().unwrap(),
                ascii.next().unwrap(),
                ascii.next().unwrap(),
            ]))
        } else {
            Err (fmt::Error)
        }
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
        )
    }
}