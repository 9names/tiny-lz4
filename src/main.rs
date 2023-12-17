static LZ4FILE: &[u8; 31] = include_bytes!("../hello.txt.lz4");

use tiny_lz4dec::fileheader;
fn main() {
    let header = fileheader::parse_header(LZ4FILE).unwrap();
    dbg!(header);
}
