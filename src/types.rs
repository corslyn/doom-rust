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
    pub things: Vec<Thing>,
    pub linedefs: Vec<Linedef>,
    // pub sidedefs: Vec<Sidedef>,
    pub vertexes: Vec<Vertex>,
    pub segments: Vec<Segment>,
    pub subsectors: Vec<Subsector>,
    pub nodes: Vec<Node>,
    // pub sectors: Vec<Sector>,
    // pub reject: Vec<u8>,
    // pub blockmap: Vec<u8>,
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

#[derive(Debug)]
pub struct Node {
    pub x_start: i16,
    pub y_start: i16,
    pub dx_start: i16,
    pub dy_start: i16,
    pub r_box: BBox,
    pub l_box: BBox,
    pub r_child: u16,
    pub l_child: u16,
}

pub struct Subsector {
    pub seg_count: i16,
    pub first_seg: i16,
}

pub struct Segment {
    pub start: i16,
    pub end: i16,
    pub angle: i16,
    pub linedef_num: i16,
    pub direction: i16,
    pub offset: i16,
}

pub struct Thing {
    pub x: i16,
    pub y: i16,
    pub angle: u16,
    pub thing_type: u16,
    pub flags: u16,
}

/// Bounding box
#[derive(Debug)]
pub struct BBox {
    pub top: i16,
    pub bottom: i16,
    pub left: i16,
    pub right: i16,
}

pub struct Player {
    pub thing: u16,
    pub angle: u16,
    pub pos: (i16, i16),
}
