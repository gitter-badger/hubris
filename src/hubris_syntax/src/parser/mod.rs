use std::path::Path;
use std::fs::File;
use std::io;
use std::io::Read;

// A way to verify the parser is not producing dummy spans
// in debug mode, need to wrap this with cfg enable at some point.
mod dummy_span_debug;
// The LALRPOP parser for our language.
mod hubris;
mod source_map;

use lalrpop_util::ParseError;
pub use self::source_map::SourceMap;
pub use super::tok;
use self::dummy_span_debug::*;

pub struct Parser {
    pub source_map: SourceMap,
}

// TODO: bring sourcemap along with this?
#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    InvalidToken {
        location: usize,
    },
    UnrecognizedToken {
        location: (usize, usize),
        token: String,
        expected: Vec<String>,
    },
    UnexpectedEOF {
        expected: Vec<String>,
    },
    UserError {
        error: tok::Error,
    },
    ExtraTokens {
        location: (usize, usize),
        token: String
    }
}

impl Parser {
    pub fn parse(&self) -> Result<super::ast::Module, Error> {
        let tokenizer = tok::Tokenizer::new(&self.source_map.source[..], 0);
        let module = try!(hubris::parse_Module(&self.source_map.source[..], tokenizer)
                              .map_err(Parser::translate_error));
        ensure_no_dummy_spans(&module);
        Ok(module)
    }

    pub fn parse_term(&self) -> Result<super::ast::Term, Error> {
        let tokenizer = tok::Tokenizer::new(&self.source_map.source[..], 0);
        hubris::parse_Term(&self.source_map.source[..], tokenizer)
            .map_err(Parser::translate_error)
    }

    pub fn translate_error<'input>(error: ParseError<usize, tok::Tok<'input>, tok::Error>) -> Error {
        match error {
            ParseError::InvalidToken { location } =>
                Error::InvalidToken { location: location },
            ParseError::UnrecognizedToken { token, expected } => {
                match token {
                    None => Error::UnexpectedEOF { expected: expected },
                    Some((start, token, end)) => {
                        Error::UnrecognizedToken {
                            location: (start, end),
                            token: format!("{:?}", token),
                            expected: expected,
                        }
                    }
                }
            }
            ParseError::User { error } => Error::UserError { error: error },
            ParseError::ExtraToken { token } => {
                let (start, t, end) = token;
                Error::ExtraTokens { location: (start, end), token: format!("{:?}", t) }
            }
        }
    }
}

pub fn from_file<T: AsRef<Path>>(path: T) -> io::Result<Parser> {
    let path = path.as_ref();

    let mut file = try!(File::open(path));
    let mut contents = String::new();

    try!(file.read_to_string(&mut contents));

    Ok(Parser {
        source_map: SourceMap::from_file(
            format!("{}", path.to_owned().display()),
            contents),
    })
}

pub fn from_string(contents: String) -> io::Result<Parser> {
    let source_map = SourceMap::from_source(contents);

    Ok(Parser { source_map: source_map })
}
