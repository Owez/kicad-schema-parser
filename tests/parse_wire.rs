use kicad_schema_parser::{parse_kicad_str, Coords, HeaderKind, KiCADSchema, SchemaNode, Wire};

/// Tests a basic parse of [Connection].
#[test]
fn parse_wire() {
    let parsed = parse_kicad_str("Wire Wire Line\n10500 4600 10000 4600");
    let expected = KiCADSchema {
        headers: Vec::new(),
        body: vec![SchemaNode::Wire(Wire {
            start: Coords { x: 10500, y: 4600 },
            end: Coords { x: 10000, y: 4600 },
        })],
    };

    assert_eq!(parsed, expected);
}
