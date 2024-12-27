use wad::Wad;

mod data;
mod wad;
fn main() {
    let wad = Wad::new("wad/DOOM1.WAD".into());
    let header = wad.read_header();
    println!("Wad size : {} bytes", wad.data.len());
    println!("Wad type : {}", header.wad_type);
    println!("Numlumps : {}", header.numlumps);
    println!("Info table offset : {}", header.infotableofs);

    println!(
        "E1M1 index: {}",
        wad.get_lump_index("E1M1".to_string()).unwrap()
    );
    // println!("Lumps : {:?}", wad.read_directory().lumps)
}
