//! Main executable for the EZWordle2 package.

use ezwordle2::{
    Wordlist,
    MatchString,
    Match::*,
};

fn main() {
    let mut wordlist = Wordlist::new();

    wordlist = wordlist.filter(
        &("raise".try_into().unwrap()),
        &MatchString ([
            Gray,
            Yellow,
            Gray,
            Gray,
            Gray,
        ]),
    );

    wordlist = wordlist.filter(
        &("float".try_into().unwrap()),
        &MatchString ([
            Gray,
            Gray,
            Green,
            Yellow,
            Green,
        ]),
    );

    wordlist = wordlist.filter(
        &("about".try_into().unwrap()),
        &MatchString ([
            Green,
            Gray,
            Green,
            Gray,
            Green,
        ]),
    );

    let guess = wordlist.guess();

    println!("I guess {}", guess);
}