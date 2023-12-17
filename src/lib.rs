#![no_std]
//! cortex-m0 asm only lz4 decompression routines
//!
//! written by Jens Bauer, published as public domain to
//! https://community.arm.com/arm-community-blogs/b/architectures-and-processors-blog/posts/lz4-decompression-routine-for-cortex-m0-and-later
//! internet archive link
//! https://web.archive.org/web/20231109105802/https://community.arm.com/arm-community-blogs/b/architectures-and-processors-blog/posts/lz4-decompression-routine-for-cortex-m0-and-later

pub mod fileheader;

use core::ptr;
/*
/* Original C fn signatures */
void unlz4(const void *aSource, void *aDestination);
void unlz4_len(const void *aSource, void *aDestination, uint32_t aLength);
*/

extern "C" {
    /// Decode LZ4 compressed data. Assumes the length of the compressed block is the first value of the src buffer
    /// This data format is created by lz4cut
    pub fn unlz4(src_addr: *const u8, dst_addr: *mut u8);
    /// Decode and LZ4 stream. Length of the compressed block given as an argument to the function
    pub fn unlz4_len(src_addr: *const u8, dst_addr: *mut u8, len: u32);
}

pub fn lz4decode(src: &[u8], dst: &mut [u8]) {
    unsafe {
        unlz4_len(
            ptr::addr_of!(src[0]),
            ptr::addr_of_mut!(dst[0]),
            src.len() as u32,
        );
    }
}
