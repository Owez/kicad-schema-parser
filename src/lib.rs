#[macro_use]
extern crate lalrpop_util;

mod parser;

pub use parser::ast::*; // Shorthand for all structures and enums contained within

/// Parses an inputted &[str] into a formatted
/// [Node format](parser::ast::SchemaNode) or panics.
///
/// NOTE: This will soon be updated to allow for a uniform result (currently in
/// testing phases).
pub fn parse_kicad_str(input: &str) -> Vec<SchemaNode> {
    parser::lalrpop_parser::GrammarParser::new()
        .parse(input)
        .unwrap()
}
