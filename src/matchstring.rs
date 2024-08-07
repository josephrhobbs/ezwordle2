//! Matchstring abstraction for the EZWordle2 package.

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
}