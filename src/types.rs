#[derive(Debug)]
pub struct Wad {
    pub data: Vec<u8>,
}

pub struct WadHeader {
    /// Wad type: IWAD or PWAD
    pub wad_type: String,

    /// Amount of lumps
    pub numlumps: i32,

    /// Offset where the lumps begins
    pub infotableofs: i32,
}

#[derive(Debug)]

/// Represents one lump of the wad
pub struct Lump {
    pub filepos: i32,
    pub size: i32,
    pub name: String,
}

#[derive(Debug)]
/// Contains all the lumps of the wad
pub struct Directory {
    pub lumps: Vec<Lump>,
}

pub struct Map {
    pub name: String,
    // pub things: Vec<Thing>,
    pub linedefs: Vec<Linedef>,
    // pub sidedefs: <Vec<Sidedef>,
    pub vertexes: Vec<Vertex>,
    // pub segs: <Vec<Seg>,
    // pub ssectors: <Vec<SSubsector>,
    // pub nodes: <Vec<Node>,
    // pub sectors: <Vec<Sector>,
    // pub reject: <Vec<u8>,
    // pub blockmap: <Vec<u8>,
}

#[derive(Debug)]
pub struct Linedef {
    /// Starting vertex
    pub start: i16,

    /// Ending vertex
    pub end: i16,
    // flags: i16,
    // special: i16,
    // tag: i16,
    // front_sidedef: i16,
    // back_sidedef: i16,
}

#[derive(Debug)]
pub struct Vertex {
    pub x_position: i16,
    pub y_position: i16,
}
