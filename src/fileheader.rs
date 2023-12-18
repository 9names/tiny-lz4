use core::hash::Hasher;
use twox_hash::XxHash32;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Flg {
    version: u8,
    b_indep: bool,
    b_checksum: bool,
    c_size: bool,
    c_checksum: bool,
    dict_id: bool,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Bd {
    block_maxsize: u8,
}

impl From<u8> for Flg {
    fn from(byte: u8) -> Flg {
        Flg {
            version: byte >> 6,
            b_indep: byte >> 5 & 1 == 1,
            b_checksum: byte >> 4 & 1 == 1,
            c_size: byte >> 3 & 1 == 1,
            c_checksum: byte >> 2 & 1 == 1,
            dict_id: byte & 1 == 1,
        }
    }
}

impl From<u8> for Bd {
    fn from(byte: u8) -> Bd {
        Bd {
            block_maxsize: byte >> 4 & 0b111,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Header {
    /// Flag bits
    pub flg: Flg,
    /// Block definition
    pub bd: Bd,
    /// Header checksum
    pub hc: u8,
    /// Size (in bytes) of compressed data block
    pub block_size: u32,
    /// is the lz4 block compressed
    pub compressed: bool,
    /// header length (in bytes)
    pub header_length: u8,
    /// Size (in bytes) data will occupy when uncompressed
    pub datasize: Option<u64>,
}

#[derive(Debug)]
pub enum Lz4Err {
    InvalidMagic,
    NotEnoughData,
    InvalidChecksum,
}

pub fn parse_header(buf: &[u8]) -> Result<Header, Lz4Err> {
    let magic = u32::from_le_bytes(buf[0..4].try_into().or(Err(Lz4Err::NotEnoughData))?);
    if magic != 0x184d2204 {
        return Err(Lz4Err::InvalidMagic);
    }
    assert_eq!(magic, 0x184d2204);
    let flg: Flg = buf[4].into();
    let bd: Bd = buf[5].into();
    // todo: cleanup
    // todo: move block decoding out of header decoding
    let (hc, hash, mut datablocks, datasize, header_length) = if !flg.c_size {
        let hc = buf[6];
        let mut t = XxHash32::with_seed(0);
        t.write(&buf[4..6]);
        let t = t.finish();
        let datablocks: [u8; 4] = buf[7..11].try_into().or(Err(Lz4Err::NotEnoughData))?;
        (hc, t, datablocks, None, 11)
    } else {
        let hc = buf[14];
        let mut t = XxHash32::with_seed(0);
        t.write(&buf[4..14]);
        let t = t.finish();
        let datasize: u64 =
            u64::from_le_bytes(buf[5..13].try_into().or(Err(Lz4Err::NotEnoughData))?);
        let datablocks: [u8; 4] = buf[15..19].try_into().or(Err(Lz4Err::NotEnoughData))?;
        (hc, t, datablocks, Some(datasize), 20)
    };
    // lz4 only uses the 2nd byte
    let hash_checksum = hash.to_le_bytes()[1];
    if hash_checksum != hc {
        return Err(Lz4Err::InvalidMagic);
    }
    let compressed = datablocks[3] >> 7 != 0;
    datablocks[3] &= 0b0111_1111;
    let block_size = u32::from_le_bytes(datablocks);

    Ok(Header {
        bd,
        flg,
        hc,
        compressed,
        block_size,
        header_length,
        datasize,
    })
}
