use chrono::{Date, Utc};

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

/// Header enum for encapsulating the various types of headers.
///
/// # Header types
///
/// - [HeaderKind::Encoding]: Wraps an [EncodeKind]
/// - [HeaderKind::DumpVersion]: The version of KiCAD's file mechanism.
/// Supports `4` officially.
/// - [HeaderKind::DumpName]: The name of the save/dump file passed through.
/// - [HeaderKind::LastUpdated]: The last date where the save file was
/// modified.
#[derive(Debug, PartialEq)]
pub enum HeaderKind {
    Encoding(EncodeKind),
    DumpVersion(u8),
    DumpName(String),
    LastUpdated(Date<Utc>),
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
/// - [Component::name]: Thought to be the name and extra infomation of a
/// component structured inside of the schema format as `[name]:[desc]`.
#[derive(Debug, PartialEq)]
pub struct Component {
    pub name: (String, String),
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
/// - [KiCADSchema::headers]: The metadata contained inside of the save file.
/// - [KiCADSchema::body]: The primary structure of the save file ([Component],
/// [Wire], etc). You can find more details for this in [HeaderKind].
#[derive(Debug, PartialEq)]
pub struct KiCADSchema {
    pub headers: Vec<HeaderKind>,
    pub body: Vec<SchemaNode>,
}
