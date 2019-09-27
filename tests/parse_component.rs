use kicad_schema_parser::{parse_kicad_str, Connection, Coords, KiCadSchema, SchemaNode};

/// Tests a basic parse of [Connection].
#[test]
fn parse_component() {
    let parsed = parse_kicad_str("Connection ~ 3023 3042");
    let expected = KiCadSchema {
        headers: Vec::new(),
        body: vec![SchemaNode::Connection(Connection {
            pos: Coords { x: 3023, y: 3042 },
        })],
    };

    assert_eq!(parsed, expected);
}
