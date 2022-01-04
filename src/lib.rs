#![allow(dead_code)]
// #![no_std]
// #![no_main]

#[repr(C)]
pub struct phr_header {
    pub name: *const i8,
    pub name_len: usize,
    pub value: *const i8,
    pub value_len: usize,
}

#[repr(C)]
pub struct phr_chunked_decoder {
    pub bytes_left_in_chunk: usize,
    pub consume_trailer: i8,
    pub hex_count: i8,
    pub state: i8,
}

#[link(name = "picohttpparser", kind = "static")]
extern "C" {
    pub fn phr_parse_request(
        buf_start: *const i8,
        buf_len: usize,
        method: *mut *const i8,
        method_len: *mut usize,
        path: *mut *const i8,
        path_len: *mut usize,
        minor_version: *mut i32,
        headers: *mut phr_header,
        num_headers: *mut usize,
        prev_buf_len: usize,
    ) -> i32;

    pub fn phr_parse_response(
        buf_start: *const i8,
        buf_len: usize,
        minor_version: *mut i32,
        status: *mut i32,
        message: *mut *const i8,
        message_len: *mut usize,
        headers: *mut phr_header,
        num_headers: *mut usize,
        prev_buf_len: usize,
    ) -> i32;

    pub fn phr_parse_headers(
        buf_start: *const i8,
        buf_len: usize,
        headers: *mut phr_header,
        num_headers: *mut usize,
        prev_buf_len: usize,
    ) -> i32;

    pub fn phr_decode_chunked(
        decoder: *mut phr_chunked_decoder,
        buf: *mut i8,
        buf_len: *mut usize,
    ) -> isize;

    pub fn phr_parse_request_path_pipeline(
        buf_start: *const i8,
        len: usize,
        method: *mut *const i8,
        method_len: *mut usize,
        path: *mut *const i8,
        path_len: *mut usize,
    ) -> i32;

    pub fn phr_parse_request_path_fast_pipeline(
        buf_start: *const i8,
        len: usize,
        method: *mut *const i8,
        method_len: *mut usize,
        path: *mut *const i8,
        path_len: *mut usize,
    ) -> i32;

    pub fn test_simd() -> i32;
}

// #[lang = "eh_personality"]
// fn eh_personality() {}

// #[panic_handler]
// fn panic(_info: &core::panic::PanicInfo) -> ! {
//     loop {}
// }
