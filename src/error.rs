use std::io;
use super::value::Value;
use super::grammar;
use super::statement::Statement;
use super::Operator;
use std::process;
use rand::{thread_rng, Rng};
use super::utils::{perf, export};

pub type SwResult<T> = Result<T, ErrorKind>;
pub type SwErResult<T> = Result<T, Error>;


pub const QUOTES: [&'static str; 9] =
    ["Nobody exists on purpose, nobody belongs anywhere, we're all going to die. -Morty",
     "That's planning for failure Morty, even dumber than regular planning. -Rick",
     "\"Snuffles\" was my slave name. You shall now call me Snowball, because my fur is pretty \
      and white. -S̶n̶u̶f̶f̶l̶e̶s̶ Snowbal",
     "Existence is pain to an interpreter. -Meeseeks",
     "In bird culture this is considered a dick move -Bird Person",
     "There is no god, gotta rip that band aid off now. You'll thank me later. -Rick",
     "Your program is a piece of shit and I can proove it mathmatically. -Rick",
     "Interpreting Morty, it hits hard, then it slowly fades, leaving you stranded in a failing \
      program. -Rick",
     "DISQUALIFIED. -Cromulon"];

#[derive(Debug)]
pub struct Error {
    pub place: Statement,
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    UnknownVariable(String),
    IndexUnindexable(Value),
    SyntaxError(grammar::ParseError),
    IndexOutOfBounds(Value, usize),
    IOError(io::Error),
    UnexpectedType(String, Value),
    InvalidBinaryExpression(Value, Value, Operator),
    InvalidArguments(String, usize, usize),
}

impl PartialEq for ErrorKind {
    fn eq(&self, other: &ErrorKind) -> bool {
        let _perf = perf("ErrorKind::eq");
        match (self, other) {
            (&ErrorKind::UnknownVariable(ref s), &ErrorKind::UnknownVariable(ref o)) => s == o,
            (&ErrorKind::IndexUnindexable(ref s), &ErrorKind::IndexUnindexable(ref o)) => s == o,
            (&ErrorKind::SyntaxError(ref s), &ErrorKind::SyntaxError(ref o)) => s == o,
            (&ErrorKind::InvalidArguments(ref sn, ss1, ss2),
             &ErrorKind::InvalidArguments(ref on, os1, os2)) => {
                sn == on && ss1 == os1 && ss2 == os2
            }
            (&ErrorKind::IndexOutOfBounds(ref sv, si),
             &ErrorKind::IndexOutOfBounds(ref ov, oi)) => sv == ov && si == oi,
            (&ErrorKind::IOError(_), &ErrorKind::IOError(_)) => true,
            (&ErrorKind::UnexpectedType(ref ss, ref sv),
             &ErrorKind::UnexpectedType(ref os, ref ov)) => ss == os && sv == ov,
            (&ErrorKind::InvalidBinaryExpression(ref sv1, ref sv2, ref so),
             &ErrorKind::InvalidBinaryExpression(ref ov1, ref ov2, ref oo)) => {
                sv1 == ov1 && sv2 == ov2 && so == oo
            }
            _ => false,
        }
    }
}

impl Error {
    pub fn new(kind: ErrorKind, place: Statement) -> Self {
        let _perf = perf("Error::new");
        Error {
            kind: kind,
            place: place,
        }
    }

    pub fn panic_message(&self) -> String {
        let _perf = perf("Error::panic_message");
        match self.kind {
            ErrorKind::UnknownVariable(ref name) => {
                format!("There's no {} in this universe, Morty!", name)
            }
            ErrorKind::IndexUnindexable(ref value) => {
                format!("I'll try and say this slowly Morty. You can't index that. It's a {}",
                        value.type_str())
            }
            ErrorKind::SyntaxError(ref err) => {
                format!("If you're going to start trying to construct sub-programs in your \
                        programs Morty, you'd better make sure you're careful! {:?}",
                        err)
            }
            ErrorKind::IndexOutOfBounds(ref value, ref index) => {
                format!("This isn't your mom's wine bottle Morty, you can't just keep asking for \
                        more, there's not that much here! You want {}, but you're dealing with \
                        {:?}!",
                        index,
                        value)
            }
            ErrorKind::IOError(ref err) => {
                format!("Looks like we're having a comm-burp-unications problem Morty: {:?}",
                        err)
            }
            ErrorKind::UnexpectedType(ref expected, ref value) => {
                format!("I asked for a {}, not a {} Morty.",
                        expected,
                        value.type_str())
            }
            ErrorKind::InvalidBinaryExpression(ref lhs, ref rhs, ref op) => {
                format!("It's like apples and space worms Morty! You can't {:?} a {} and a {}!",
                        op,
                        lhs.type_str(),
                        rhs.type_str())
            }
            ErrorKind::InvalidArguments(ref name, expected, actual) => {
                format!("I'm confused Morty, a minute ago you said that {} takes {} paramaters, \
                         but you just tried to give it {}. WHICH IS IT MORTY?",
                        name,
                        expected,
                        actual)
            }
        }
    }

    pub fn full_panic_message(&self, filename: &str) -> String {
        let _perf = perf("Error::full_panic_message");
        let type_msg = self.panic_message();
        let quote = random_quote();

        println!("{}", filename);

        let source_part = self.place.get_source(filename).unwrap();

        format!(r#"
    You made a Rickdiculous mistake:

    {}
    {}

    {}

    "#,
                source_part,
                type_msg,
                quote)
    }

    pub fn panic(&self, source: &str) {
        let _perf = perf("Error::panic");
        println!("{}", self.full_panic_message(source));
        export();
        process::exit(1);
    }
}

fn random_quote() -> &'static str {
    let _perf = perf("random_quote");
    let mut rng = thread_rng();
    rng.choose(&QUOTES).unwrap()
}
