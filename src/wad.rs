use crate::{
    data_types::{Directory, Linedef, Lump, Map, Thing, Vertex, Wad, WadHeader},
    map::LumpIndex,
};

impl Wad {
    /// Creates a new Wad instance from a file
    pub fn new(filepath: &str) -> Wad {
        let wad_data = std::fs::read(filepath).expect("File not found");
        Wad { data: wad_data }
    }

    /// Reads 2 bytes from the data at the given offset
    pub fn read_2_bytes(&self, offset: usize) -> Vec<u8> {
        self.data[offset..offset + 2].to_vec()
    }

    /// Reads 4 bytes from the data at the given offset
    pub fn read_4_bytes(&self, offset: usize) -> Vec<u8> {
        self.data[offset..offset + 4].to_vec()
    }

    /// Reads 8 bytes from the data at the given offset
    pub fn read_8_bytes(&self, offset: usize) -> Vec<u8> {
        self.data[offset..offset + 8].to_vec()
    }

    /// Reads the wad header
    pub fn read_header(&self) -> WadHeader {
        let wad_type = String::from_utf8(self.read_4_bytes(0).to_vec()).unwrap();
        let directory_count = u32::from_le_bytes(self.read_4_bytes(4).try_into().unwrap());
        let directory_offset = u32::from_le_bytes(self.read_4_bytes(8).try_into().unwrap());

        WadHeader {
            wad_type,
            directory_count,
            directory_offset,
        }
    }

    /// Returns a directory with all the lumps
    pub fn read_directory(&self) -> Directory {
        let header = self.read_header();
        let mut lumps = Vec::new();
        for i in 0..header.directory_count {
            let lump_offset = u32::from_le_bytes(
                self.read_4_bytes(header.directory_offset as usize + i as usize * 16)
                    .try_into()
                    .unwrap(),
            );
            let lump_size = u32::from_le_bytes(
                self.read_4_bytes(header.directory_offset as usize + i as usize * 16 + 4)
                    .try_into()
                    .unwrap(),
            );
            let lump_name = String::from_utf8(
                self.read_8_bytes(header.directory_offset as usize + i as usize * 16 + 8),
            )
            .unwrap();
            let lump = Lump {
                lump_offset,
                lump_size,
                lump_name,
            };
            lumps.push(lump);
        }
        Directory { lumps }
    }

    /// Reads the vertex data at the given offset
    pub fn read_vertex_data(&self, offset: usize) -> Vertex {
        Vertex {
            x_position: i16::from_le_bytes(self.read_2_bytes(offset).try_into().unwrap()),
            y_position: i16::from_le_bytes(self.read_2_bytes(offset + 2).try_into().unwrap()),
        }
    }

    /// Reads the linedef data at the given offset
    pub fn read_linedef_data(&self, offset: usize) -> Linedef {
        Linedef {
            start_vertex: u16::from_le_bytes(self.read_2_bytes(offset).try_into().unwrap()),
            end_vertex: u16::from_le_bytes(self.read_2_bytes(offset + 2).try_into().unwrap()),
            flags: u16::from_le_bytes(self.read_2_bytes(offset + 4).try_into().unwrap()),
            linetype: u16::from_le_bytes(self.read_2_bytes(offset + 6).try_into().unwrap()),
            sector_tag: u16::from_le_bytes(self.read_2_bytes(offset + 8).try_into().unwrap()),
            right_sidedef: u16::from_le_bytes(self.read_2_bytes(offset + 10).try_into().unwrap()),
            left_sidedef: u16::from_le_bytes(self.read_2_bytes(offset + 12).try_into().unwrap()),
        }
    }

    /// Returns the lump index of the given lump name
    pub fn get_lump_index(&self, lump_name: &str) -> usize {
        self.read_directory()
            .lumps
            .iter()
            .position(|lump| lump.lump_name.trim_end_matches('\0') == lump_name)
            .unwrap()
    }

    /// Returns the vertices of the given map
    pub fn get_vertices(&self, map: &mut Map) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        let directory = self.read_directory();

        let map_index = self.get_lump_index(&map.map_name);
        let vertices_index = map_index + LumpIndex::VERTEXES as usize;

        let vertices_lump = &directory.lumps[vertices_index];
        for i in 0..vertices_lump.lump_size / 4 {
            let offset = vertices_lump.lump_offset + i * 4;
            let x = i16::from_le_bytes(self.read_2_bytes(offset as usize).try_into().unwrap());
            let y = i16::from_le_bytes(self.read_2_bytes(offset as usize + 2).try_into().unwrap());

            if map.x_min > x {
                map.x_min = x;
            } else if map.x_max < x {
                map.x_max = x;
            }

            if map.y_min > y {
                map.y_min = y;
            } else if map.y_max < y {
                map.y_max = y;
            }
            vertices.push(Vertex {
                x_position: x,
                y_position: y,
            });
        }
        vertices
    }

    /// Returns the linedefs of the given map
    pub fn get_linedefs(&self, map_name: &str) -> Vec<Linedef> {
        let mut linedefs = Vec::new();
        let directory = self.read_directory();

        let map_index = self.get_lump_index(map_name);
        let linedefs_index = map_index + LumpIndex::LINEDEFS as usize;

        let linedefs_lump = &directory.lumps[linedefs_index];

        for i in 0..linedefs_lump.lump_size / 14 {
            let offset = linedefs_lump.lump_offset + i * 14;
            let start_vertex =
                u16::from_le_bytes(self.read_2_bytes(offset as usize).try_into().unwrap());
            let end_vertex =
                u16::from_le_bytes(self.read_2_bytes(offset as usize + 2).try_into().unwrap());
            let flags =
                u16::from_le_bytes(self.read_2_bytes(offset as usize + 4).try_into().unwrap());
            let linetype =
                u16::from_le_bytes(self.read_2_bytes(offset as usize + 6).try_into().unwrap());
            let sector_tag =
                u16::from_le_bytes(self.read_2_bytes(offset as usize + 8).try_into().unwrap());
            let right_sidedef =
                u16::from_le_bytes(self.read_2_bytes(offset as usize + 10).try_into().unwrap());
            let left_sidedef =
                u16::from_le_bytes(self.read_2_bytes(offset as usize + 12).try_into().unwrap());
            linedefs.push(Linedef {
                start_vertex,
                end_vertex,
                flags,
                linetype,
                sector_tag,
                right_sidedef,
                left_sidedef,
            });
        }
        linedefs
    }

    pub fn get_map_things(&self, map_name: &str) -> Vec<Thing> {
        let mut things = Vec::new();
        let directory = self.read_directory();

        let map_index = self.get_lump_index(map_name);
        let things_index = map_index + LumpIndex::THINGS as usize;

        let things_lump = &directory.lumps[things_index];

        for i in 0..things_lump.lump_size / 10 {
            let offset = things_lump.lump_offset + i * 10;
            let x_position =
                i16::from_le_bytes(self.read_2_bytes(offset as usize).try_into().unwrap());
            let y_position =
                i16::from_le_bytes(self.read_2_bytes(offset as usize + 2).try_into().unwrap());
            let angle =
                u16::from_le_bytes(self.read_2_bytes(offset as usize + 4).try_into().unwrap());
            let thing_type =
                u16::from_le_bytes(self.read_2_bytes(offset as usize + 6).try_into().unwrap());
            let flags =
                u16::from_le_bytes(self.read_2_bytes(offset as usize + 8).try_into().unwrap());
            things.push(Thing {
                x_position,
                y_position,
                angle,
                thing_type,
                flags,
            });
        }
        things
    }
}
