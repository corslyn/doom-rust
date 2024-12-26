use wad::Wad;

mod wad;
fn main() {
    let wad = Wad::new("wad/DOOM1.WAD".into());
    println!("Wad size : {} bytes", wad.data.len());
}
