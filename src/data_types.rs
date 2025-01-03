use crate::player::Player;

/// Represents a WAD file.
pub struct Wad {
    /// Raw data of the WAD file
    pub data: Vec<u8>,
}

#[derive(Debug)]
/// Represents the header of a WAD file.
pub struct WadHeader {
    /// Wad type: IWAD or PWAD
    pub wad_type: String,

    /// Amount of lumps
    pub directory_count: u32,

    /// Offset where the lumps begins
    pub directory_offset: u32,
}

pub struct Directory {
    /// Contains all the lumps of the WAD file
    pub lumps: Vec<Lump>,
}

#[derive(Debug)]
/// Represents one lump of the WAD file.
pub struct Lump {
    /// Offset of the lump
    pub lump_offset: u32,
    /// Size of the lump
    pub lump_size: u32,
    /// Name of the lump (8 bytes)
    pub lump_name: String,
}

/// Represents a map in the WAD file.
pub struct Map {
    /// Name of the map (E1M1 for Doom 1, MAP01 for Doom 2)
    pub map_name: String,

    /// Vertices of the map (points in the map)
    pub vertices: Vec<Vertex>,

    /// Linedefs of the map (lines between vertices)
    pub linedefs: Vec<Linedef>,

    pub things: Vec<Thing>,

    pub nodes: Vec<Node>,

    pub subsectors: Vec<Subsector>,

    pub segments: Vec<Segment>,

    /// Minimum x coordinate
    pub x_min: i16,
    /// Maximum x coordinate
    pub x_max: i16,
    /// Minimum y coordinate
    pub y_min: i16,
    /// Maximum y coordinate
    pub y_max: i16,

    pub scale_factor: f32,

    pub player: Player,

    pub render_y_size: u32,
}

/// Represents a vertex in the map.
#[derive(Debug, Clone)]
pub struct Vertex {
    pub x_position: i16,
    pub y_position: i16,
}

/// Represents a linedef in the map.
pub struct Linedef {
    /// Starting vertex id
    pub start_vertex: u16,
    /// Ending vertex id
    pub end_vertex: u16,
    /// Flags of the linedef
    pub flags: u16,
    /// Special type of the linedef
    pub linetype: u16,
    /// Tag of the linedef
    pub sector_tag: u16,
    /// Right sidedef id
    pub right_sidedef: u16,
    /// Left sidedef id
    pub left_sidedef: u16,
}
#[derive(Clone)]
pub struct Thing {
    pub x_position: i16,
    pub y_position: i16,
    pub angle: u16,
    pub thing_type: u16,
    pub flags: u16,
}

pub struct Node {
    pub x_partition: i16,
    pub y_partition: i16,
    pub change_x_partition: i16,
    pub change_y_partition: i16,

    pub right_bbox_top: i16,
    pub right_bbox_bottom: i16,
    pub right_bbox_left: i16,
    pub right_bbox_right: i16,

    pub left_bbox_top: i16,
    pub left_bbox_bottom: i16,
    pub left_bbox_left: i16,
    pub left_bbox_right: i16,

    pub right_child: u16,
    pub left_child: u16,
}

pub struct Subsector {
    pub num_segs: u16,
    pub first_seg: u16,
}

#[derive(Debug)]
pub struct Segment {
    pub start_vertex: u16,
    pub end_vertex: u16,
    pub angle: u16,
    pub linedef: u16,
    pub direction: u16, // 0 = same as linedef, 1 = opposite
    pub offset: u16,    // distance along linedef to start of seg
}
