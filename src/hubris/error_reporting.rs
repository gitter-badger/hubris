use typeck::{TyCtxt};
use super::ast::{Span, SourceMap};

use std::io::prelude::*;
use term::{Terminal, color, Result as TResult};

pub trait ErrorContext<O: Write> {
    // I don't particularly like these names - JR
    fn get_source_map(&self) -> &SourceMap;
    fn get_terminal(&mut self) -> &mut Box<Terminal<Output=O> + Send>;

    fn span_error(&mut self,
                  span: Span,
                  message: String) -> TResult<()> {
        let (line_no, col_no) = self.get_source_map().position(span).unwrap();
        let (line_with_padding, marker) = self.get_source_map().underline_span(span).unwrap();

        let filename_str = format!("{}:{}:{}: {}:{} ",
                               self.get_source_map().file_name,
                               line_no,
                               col_no,
                               line_no,
                               col_no);

        try!(write!(self.get_terminal(), "{}", filename_str));

        try!(self.get_terminal().fg(color::RED));
        try!(write!(self.get_terminal(), "error: "));
        try!(self.get_terminal().reset());
        try!(writeln!(self.get_terminal(), "{}", message));

        let file_str_simple =
            format!("{}:{}:{}: ",
            self.get_source_map().file_name,
            line_no,
            col_no);

        try!(write!(self.get_terminal(), "{} {}", file_str_simple, line_with_padding));

        let mut marker_padding = "".to_string();

        for _ in 0..file_str_simple.len() {
            marker_padding.push(' ');
        }

        try!(write!(self.get_terminal(), "{}", marker_padding));
        try!(self.get_terminal().fg(color::RED));
        try!(writeln!(self.get_terminal(), "{}", marker));
        try!(self.get_terminal().reset());

        Ok(())
    }
}

pub trait Report<O: Write, E: ErrorContext<O>> {
    fn report(self, cx: &mut E) -> TResult<()>;
}
