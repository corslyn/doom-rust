use crate::data_types::{Directory, Lump, Wad, WadHeader};

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
                self.read_8_bytes(header.directory_offset as usize + i as usize * 16 + 8)
                    .to_vec(),
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
}
