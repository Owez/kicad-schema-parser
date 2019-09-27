//! # About
//!
//! This is a work-in-progress parser for the KiCAD schematic save files, to be
//! used in an auto-generation framework for embedded devices.
//!
//! This project uses the [lalrpop](https://github.com/lalrpop/lalrpop) library
//! for parsing with custom structures as it's primary source of outputs.
//!
//! # Examples
//!
//! Basic schematic with 1 header:
//!
//! ```rust
//! use kicad_schema_parser::{
//!     parse_kicad_str,
//!     KiCadSchema,
//!     HeaderKind,
//!     Wire,
//!     WireType,
//!     Coords,
//!     SchemaNode
//! };
//!
//! fn main() {
//!     let wire_exp = Wire {
//!         ty: WireType::Wire,
//!         start: Coords { x: 0, y: 0 },
//!         end: Coords { x: 1, y: 1 }
//!     };
//!
//!     let expected = KiCadSchema {
//!         headers: vec![
//!             HeaderKind::DumpName("MySchema".to_string()),
//!         ],
//!         body: vec![
//!             SchemaNode::Wire(wire_exp)
//!         ]
//!     };
//!
//!     let input_str = "Title \"MySchema\"\nWire Wire Line\n\t0 0 1 1";
//!
//!     assert_eq!(parse_kicad_str(input_str), expected);
//! }
//! ```

#[macro_use]
extern crate lalrpop_util;

mod parser;

pub use parser::ast::*; // Shorthand for all structures and enums contained within

/// Parses an inputted &[str] into a formatted
/// [Node format](parser::ast::SchemaNode) or panics.
///
/// NOTE: This will soon be updated to allow for a uniform result (currently in
/// testing phases).
pub fn parse_kicad_str(input: &str) -> KiCadSchema {
    parser::lalrpop_parser::GrammarParser::new()
        .parse(input)
        .unwrap()
}
