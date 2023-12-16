#![no_std]
//! cortex-m0 asm only lz4 decompression routines
//!
//! written by Jens Bauer, published as public domain to
//! https://community.arm.com/arm-community-blogs/b/architectures-and-processors-blog/posts/lz4-decompression-routine-for-cortex-m0-and-later
//! internet archive link
//! https://web.archive.org/web/20231109105802/https://community.arm.com/arm-community-blogs/b/architectures-and-processors-blog/posts/lz4-decompression-routine-for-cortex-m0-and-later

use core::ffi::c_void;

/*
/* Original C fn signatures */
void unlz4(const void *aSource, void *aDestination);
void unlz4_len(const void *aSource, void *aDestination, uint32_t aLength);
*/

extern "C" {
    pub fn unlz4(src_addr: *const u8, dst_addr: *mut u8);
    pub fn unlz4_len(src_addr: *const u8, dst_addr: *mut u8, len: u32);
}
