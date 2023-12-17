static LZ4FILE: &[u8; 31] = include_bytes!("../hello.txt.lz4");

use tiny_lz4dec::fileheader;
use lz4_flex::block::decompress;
fn main() {
    let header = fileheader::parse_header(LZ4FILE).unwrap();
    // dbg!(header);
    let start = header.header_length as usize + 1; // 12
    let datasize = header.block_size as usize; // 12
    let end = start + datasize;
    println!("{start} {datasize} {end}");
    dbg!(header);
    // try to decompress file
    let decoded = decompress(&LZ4FILE[start..end], 64).unwrap();
    let decoded_string = std::str::from_utf8(&decoded).unwrap();
    println!("{}", decoded_string);
}
