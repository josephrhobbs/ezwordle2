//! Matchstring abstraction for the EZWordle2 package.

use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
/// A match element.
pub enum Match {
    /// Correct letter and position.
    Green,

    /// Correct letter, wrong position.
    Yellow,

    /// Incorrect letter.
    Gray,
}

impl TryFrom<char> for Match {
    type Error = fmt::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use Match::*;
        match c {
            '.' => Ok (Green),
            '/' => Ok (Yellow),
            'x' => Ok (Gray),
            _ => Err (fmt::Error)
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
/// A matchstring, consisting of five match elements.
pub struct MatchString (pub [Match; 5]);

impl MatchString {
    /// Generate a list of all possible matchstrings.
    /// 
    /// # Parameters
    /// None.
    ///
    /// # Returns
    /// A `Vec<MatchString>` containing every possible matchstring.
    pub fn all() -> Vec<Self> {
        Self::generate(5)
    }

    /// Generate a list of all possible matchstrings
    /// of a provided length.
    /// 
    /// # Parameters
    /// - `n` (`usize`): the specified length
    /// 
    /// # Returns
    /// A `Vec<MatchString>` containing every possible matchstring
    /// of the provided length.
    fn generate(n: usize) -> Vec<Self> {
        use Match::*;

        if n == 0 {
            // Base case: n is zero, return a vector with one item
            vec![
                MatchString ([Gray, Gray, Gray, Gray, Gray]),
            ]
        } else if n <= 5 {
            // Construct vector of next size up
            let mut next = Vec::new();

            for ms in Self::generate(n - 1) {
                let mut green = ms;
                let mut yellow = ms;
                let mut gray = ms;

                green.0[n - 1] = Green;
                yellow.0[n - 1] = Yellow;
                gray.0[n - 1] = Gray;

                next.push(green);
                next.push(yellow);
                next.push(gray);
            }

            next
        } else {
            // We never call this method with n > 5
            unreachable!()
        }
    }

    /// Check if this matchstring is winning.
    /// 
    /// # Parameters
    /// None.
    /// 
    /// # Returns
    /// A `bool` indicating whether or not the
    /// player has won.
    pub fn check(&self) -> bool {
        *self == Self ([Match::Green; 5])
    }
}

impl TryFrom<String> for MatchString {
    type Error = fmt::Error;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.len() == 5 {
            // Initialize an empty matchstring
            let mut matchstring = Self ([Match::Gray; 5]);
            for (i, chr) in string.chars().enumerate() {
                matchstring.0[i] = chr.try_into()?;
            }

            Ok (matchstring)
        } else {
            Err (fmt::Error)
        }
    }
}