use crate::*;

impl Wad {
    /// Returns a vector of vertices
    pub fn get_vertices(&self, map_name: &str) -> Vec<Vertex> {
        let map_index = self.get_lump_index(map_name);

        let directory = self.read_directory();
        let vertices_index = directory
            .lumps
            .iter()
            .skip(map_index + 1)
            .position(|lump| lump.name == "VERTEXES")
            .unwrap();
        let vertices_lump = &directory.lumps[map_index + 1 + vertices_index];

        let mut vertices = Vec::new();

        for i in 0..vertices_lump.size / 4 {
            let offset = vertices_lump.filepos + i as i32 * 4;
            let x = i16::from_le_bytes(self.read_n_bytes(offset, 2).try_into().unwrap());
            let y = i16::from_le_bytes(self.read_n_bytes(offset + 2, 2).try_into().unwrap());
            vertices.push(Vertex {
                x_position: x,
                y_position: y,
            });
        }
        vertices
    }

    /// Returns a vector of linedefs
    pub fn get_linedefs(&self, map_name: &str) -> Vec<Linedef> {
        let map_index = self.get_lump_index(map_name);

        let directory = self.read_directory();
        let linedefs_index = directory
            .lumps
            .iter()
            .skip(map_index + 1)
            .position(|lump| lump.name == "LINEDEFS")
            .unwrap();
        let linedefs_lump = &directory.lumps[map_index + 1 + linedefs_index];

        let mut linedefs = Vec::new();

        for i in 0..linedefs_lump.size / 14 {
            let offset = linedefs_lump.filepos + i as i32 * 14;
            let start = i16::from_le_bytes(self.read_n_bytes(offset, 2).try_into().unwrap());
            let end = i16::from_le_bytes(self.read_n_bytes(offset + 2, 2).try_into().unwrap());
            linedefs.push(Linedef { start, end });
        }
        linedefs
    }

    /// Returns a vector of subsectors
    pub fn get_subsectors(&self, map_name: &str) -> Vec<Subsector> {
        let map_index = self.get_lump_index(map_name);
        let directory = self.read_directory();
        let subsectors_index = directory
            .lumps
            .iter()
            .skip(map_index + 1)
            .position(|lump| lump.name == "SSECTORS")
            .unwrap();
        let subsectors_lump = &directory.lumps[map_index + 1 + subsectors_index];
        let mut subsectors = Vec::new();

        for i in 0..subsectors_lump.size / 4 {
            let offset = subsectors_lump.filepos + i as i32 * 4;
            let seg_count = i16::from_le_bytes(self.read_n_bytes(offset, 2).try_into().unwrap());
            let first_seg =
                i16::from_le_bytes(self.read_n_bytes(offset + 2, 2).try_into().unwrap());
            subsectors.push(Subsector {
                seg_count,
                first_seg,
            });
        }
        subsectors
    }

    pub fn get_things(&self, map_name: &str) -> Vec<Thing> {
        let map_index = self.get_lump_index(map_name);
        let directory = self.read_directory();
        let things_index = directory
            .lumps
            .iter()
            .skip(map_index + 1)
            .position(|lump| lump.name == "THINGS")
            .unwrap();

        let things_lump = &directory.lumps[map_index + 1 + things_index];
        let mut things = Vec::new();

        for i in 0..things_lump.size / 10 {
            let offset = things_lump.filepos + i as i32 * 10;
            let x = i16::from_le_bytes(self.read_n_bytes(offset, 2).try_into().unwrap());
            let y = i16::from_le_bytes(self.read_n_bytes(offset + 2, 2).try_into().unwrap());
            let angle = u16::from_le_bytes(self.read_n_bytes(offset + 4, 2).try_into().unwrap());
            let thing_type =
                u16::from_le_bytes(self.read_n_bytes(offset + 6, 2).try_into().unwrap());
            let flags = u16::from_le_bytes(self.read_n_bytes(offset + 8, 2).try_into().unwrap());

            things.push(Thing {
                x,
                y,
                angle,
                thing_type,
                flags,
            });
        }
        things
    }

    pub fn get_segments(&self, map_name: &str) -> Vec<Segment> {
        let map_index = self.get_lump_index(map_name);
        let directory = self.read_directory();
        let segs_index = directory
            .lumps
            .iter()
            .skip(map_index + 1)
            .position(|lump| lump.name == "SEGS")
            .unwrap();
        let segs_lump = &directory.lumps[map_index + 1 + segs_index];
        let mut segs = Vec::new();

        for i in 0..segs_lump.size / 12 {
            let offset = segs_lump.filepos + i as i32 * 12;
            let start = i16::from_le_bytes(self.read_n_bytes(offset, 2).try_into().unwrap());
            let end = i16::from_le_bytes(self.read_n_bytes(offset + 2, 2).try_into().unwrap());
            let angle = i16::from_le_bytes(self.read_n_bytes(offset + 4, 2).try_into().unwrap());
            let linedef_num =
                i16::from_le_bytes(self.read_n_bytes(offset + 6, 2).try_into().unwrap());
            let direction =
                i16::from_le_bytes(self.read_n_bytes(offset + 8, 2).try_into().unwrap());
            let offset = i16::from_le_bytes(self.read_n_bytes(offset + 10, 2).try_into().unwrap());

            segs.push(Segment {
                start,
                end,
                angle,
                linedef_num,
                direction,
                offset,
            });
        }
        segs
    }

    pub fn get_nodes(&self, map_name: &str) -> Vec<Node> {
        let map_index = self.get_lump_index(map_name);
        let directory = self.read_directory();
        let nodes_index = directory
            .lumps
            .iter()
            .skip(map_index + 1)
            .position(|lump| lump.name == "NODES")
            .unwrap();
        let nodes_lump = &directory.lumps[map_index + 1 + nodes_index];
        let mut nodes = Vec::new();

        for i in 0..nodes_lump.size / 28 {
            let offset = nodes_lump.filepos + i as i32 * 28;
            let x_start = i16::from_le_bytes(self.read_n_bytes(offset, 2).try_into().unwrap());
            let y_start = i16::from_le_bytes(self.read_n_bytes(offset + 2, 2).try_into().unwrap());
            let dx_start = i16::from_le_bytes(self.read_n_bytes(offset + 4, 2).try_into().unwrap());
            let dy_start = i16::from_le_bytes(self.read_n_bytes(offset + 6, 2).try_into().unwrap());

            let r_box = BBox {
                top: i16::from_le_bytes(self.read_n_bytes(offset + 8, 2).try_into().unwrap()),
                bottom: i16::from_le_bytes(self.read_n_bytes(offset + 10, 2).try_into().unwrap()),
                left: i16::from_le_bytes(self.read_n_bytes(offset + 12, 2).try_into().unwrap()),
                right: i16::from_le_bytes(self.read_n_bytes(offset + 14, 2).try_into().unwrap()),
            };

            let l_box = BBox {
                top: i16::from_le_bytes(self.read_n_bytes(offset + 16, 2).try_into().unwrap()),
                bottom: i16::from_le_bytes(self.read_n_bytes(offset + 18, 2).try_into().unwrap()),
                left: i16::from_le_bytes(self.read_n_bytes(offset + 20, 2).try_into().unwrap()),
                right: i16::from_le_bytes(self.read_n_bytes(offset + 22, 2).try_into().unwrap()),
            };

            let r_child = u16::from_le_bytes(self.read_n_bytes(offset + 24, 2).try_into().unwrap());
            let l_child = u16::from_le_bytes(self.read_n_bytes(offset + 26, 2).try_into().unwrap());

            nodes.push(Node {
                x_start,
                y_start,
                dx_start,
                dy_start,
                r_box,
                l_box,
                r_child,
                l_child,
            });
        }
        nodes
    }
}
