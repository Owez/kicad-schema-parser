use chrono::prelude::*;

/// Co-ordinates of an item inside of the stucture file. All items apart from
/// metadata should have this.
#[derive(Debug, PartialEq)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

/// The main node for the schema, containing the general structural infomation.
#[derive(Debug, PartialEq)]
pub enum SchemaNode {
    Connection(Connection),
    Component(Component),
    Wire(Wire),
}

/// A "Connection" inside of the schema file.
///
/// # Format Example
///
/// - `Connection ~ 6125 1400`
/// - `Connection ~ 6025 1400`
/// - `Connection ~ 5925 1400`
#[derive(Debug, PartialEq)]
pub struct Connection {
    pub pos: Coords,
}

/// A "Wire Wire" inside of the schema file.
///
/// # Format Example
///
/// ```none
/// Wire Wire Line
///	    10500 4600 10000 4600
/// Wire Wire Line
///	    10500 4600 10500 4000
/// Wire Wire Line
/// 	10400 4000 10400 4400
/// ```
#[derive(Debug, PartialEq)]
pub struct Wire {
    pub start: Coords,
    pub end: Coords,
}

/// A "Component" inside of the schema file. This appears to be the main
/// structures of the dump.
///
/// # [Component] Structure
///
/// - [Component::catagory]: Thought to be the name and extra infomation of a
/// component structured inside of the schema format as `[name]:[desc]`.
/// - [Component::pos]: The [Coords] position of the component.
#[derive(Debug, PartialEq)]
pub struct Component {
    pub name: (String, String),
    // pub pos: Coords,
}

/// The encoding type for the dump file.
#[derive(Debug, PartialEq)]
pub enum EncodeKind {
    UTF8,
}

/// Frontend structure for the parsed KiCAD dump file. This includes the
/// metadata and all structures making up the file.
///
/// # [KiCADSchema] Structure
///
/// - [KiCADSchema::encoded]: The enocde type of the file. This should usually
/// be [EncodeKind::UTF8] unless on some non-standard locale.
/// - [KiCADSchema::title]: The given title of the dump file.
/// - [KiCADSchema::date_updated]: Last modified date. This uses [chrono] to
/// give a user-friendly date representation.
/// - [KiCADSchema::body]: Main structure of passed schematic file. Please see
/// [SchemaNode] for more detail.
/// - [KiCADSchema::version]: Schematic version (given by file author).
/// - [KiCADSchema::schema_version]: The KiCAD dump file version. This
/// currently is not counted, just extra metadata.
#[derive(Debug, PartialEq)]
pub struct KiCADSchema {
    pub schema_version: u8,
    pub encoded: EncodeKind,
    pub title: String,
    pub date_updated: DateTime<Utc>,
    pub body: Vec<SchemaNode>,
    pub version: String,
}
