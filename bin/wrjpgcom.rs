#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(
    const_raw_ptr_to_usize_cast,
    const_transmute,
    extern_types,
    label_break_value,
    main,
    register_tool
)]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;

    pub const NULL: libc::c_int = 0 as libc::c_int;
}
pub mod stdlib {
    extern "C" {
        #[no_mangle]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;

        #[no_mangle]
        pub fn __ctype_tolower_loc() -> *mut *const crate::stdlib::__int32_t;
        #[no_mangle]
        pub static mut stdout: *mut crate::stdlib::FILE;

        #[no_mangle]
        pub static mut stderr: *mut crate::stdlib::FILE;

        #[no_mangle]
        pub fn fclose(__stream: *mut crate::stdlib::FILE) -> libc::c_int;

        #[no_mangle]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut crate::stdlib::FILE;

        #[no_mangle]
        pub fn fprintf(_: *mut crate::stdlib::FILE, _: *const libc::c_char, _: ...) -> libc::c_int;

        #[no_mangle]
        pub fn getc(__stream: *mut crate::stdlib::FILE) -> libc::c_int;

        #[no_mangle]
        pub fn putc(__c: libc::c_int, __stream: *mut crate::stdlib::FILE) -> libc::c_int;

        #[no_mangle]
        pub static mut stdin: *mut crate::stdlib::FILE;
        #[no_mangle]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;

        #[no_mangle]
        pub fn exit(_: libc::c_int) -> !;
        #[no_mangle]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

        #[no_mangle]
        pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        pub type _IO_wide_data;

        pub type _IO_codecvt;

        pub type _IO_marker;
    }
    pub type FILE = crate::stdlib::_IO_FILE;
    pub type C2RustUnnamed_0 = libc::c_uint;

    pub const _ISalnum: crate::stdlib::C2RustUnnamed_0 = 8;

    pub const _ISpunct: crate::stdlib::C2RustUnnamed_0 = 4;

    pub const _IScntrl: crate::stdlib::C2RustUnnamed_0 = 2;

    pub const _ISblank: crate::stdlib::C2RustUnnamed_0 = 1;

    pub const _ISgraph: crate::stdlib::C2RustUnnamed_0 = 32768;

    pub const _ISprint: crate::stdlib::C2RustUnnamed_0 = 16384;

    pub const _ISspace: crate::stdlib::C2RustUnnamed_0 = 8192;

    pub const _ISxdigit: crate::stdlib::C2RustUnnamed_0 = 4096;

    pub const _ISdigit: crate::stdlib::C2RustUnnamed_0 = 2048;

    pub const _ISalpha: crate::stdlib::C2RustUnnamed_0 = 1024;

    pub const _ISlower: crate::stdlib::C2RustUnnamed_0 = 512;

    pub const _ISupper: crate::stdlib::C2RustUnnamed_0 = 256;
    pub const EOF: libc::c_int = -(1 as libc::c_int);
    pub const EXIT_SUCCESS: libc::c_int = 0 as libc::c_int;

    pub const EXIT_FAILURE: libc::c_int = 1 as libc::c_int;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut crate::stdlib::_IO_marker,
        pub _chain: *mut crate::stdlib::_IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: crate::stdlib::__off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: crate::stdlib::__off64_t,
        pub _codecvt: *mut crate::stdlib::_IO_codecvt,
        pub _wide_data: *mut crate::stdlib::_IO_wide_data,
        pub _freeres_list: *mut crate::stdlib::_IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: crate::stddef_h::size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }

    pub type _IO_lock_t = ();
    pub type __int32_t = libc::c_int;

    pub type __off_t = libc::c_long;

    pub type __off64_t = libc::c_long;
}
use ::mozjpeg::*;

#[c2rust::header_src = "/usr/include/ctype.h:23"]
pub mod ctype_h {

    #[inline]

    pub unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
        return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
            *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
}

pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::ctype_h::tolower;
pub use crate::stdlib::C2RustUnnamed_0;
pub use crate::stdlib::_ISalnum;
pub use crate::stdlib::_ISalpha;
pub use crate::stdlib::_ISblank;
pub use crate::stdlib::_IScntrl;
pub use crate::stdlib::_ISdigit;
pub use crate::stdlib::_ISgraph;
pub use crate::stdlib::_ISlower;
pub use crate::stdlib::_ISprint;
pub use crate::stdlib::_ISpunct;
pub use crate::stdlib::_ISspace;
pub use crate::stdlib::_ISupper;
pub use crate::stdlib::_ISxdigit;
pub use crate::stdlib::__ctype_b_loc;
pub use crate::stdlib::__ctype_tolower_loc;
pub use crate::stdlib::exit;
pub use crate::stdlib::fclose;
pub use crate::stdlib::fopen;
pub use crate::stdlib::fprintf;
pub use crate::stdlib::getc;
pub use crate::stdlib::malloc;
pub use crate::stdlib::putc;
pub use crate::stdlib::stderr;
pub use crate::stdlib::stdin;
pub use crate::stdlib::stdout;
use crate::stdlib::strcat;
use crate::stdlib::strcpy;
use crate::stdlib::strlen;
pub use crate::stdlib::EOF;
pub use crate::stdlib::EXIT_FAILURE;
pub use crate::stdlib::EXIT_SUCCESS;
/*
 * wrjpgcom.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1997, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2014, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains a very simple stand-alone application that inserts
 * user-supplied text as a COM (comment) marker in a JFIF file.
 * This may be useful as an example of the minimum logic needed to parse
 * JPEG markers.
 */
/* to get the command-line config symbols */
/* <stdlib.h> should declare malloc() */
/* to declare isupper(), tolower() */
/* command-line reader for Macintosh */
/* define mode parameters for fopen() */

pub const READ_BINARY: [libc::c_char; 3] =
    unsafe { *::std::mem::transmute::<&[u8; 3], &[libc::c_char; 3]>(b"rb\x00") };
/* define exit() codes if not provided */
/* Reduce this value if your malloc() can't allocate blocks up to 64K.
 * On DOS, compiling in large model is usually a better solution.
 */

pub const MAX_COM_LENGTH: libc::c_long = 65000 as libc::c_long;
/* must be <= 65533 in any case */
/*
 * These macros are used to read the input file and write the output file.
 * To reuse this code in another application, you might need to change these.
 */

static mut infile: *mut crate::stdlib::FILE =
    0 as *const crate::stdlib::FILE as *mut crate::stdlib::FILE;
/* input JPEG file */
/* Return next input byte, or EOF if no more */

static mut outfile: *mut crate::stdlib::FILE =
    0 as *const crate::stdlib::FILE as *mut crate::stdlib::FILE;
/* output JPEG file */
/* Emit an output byte */
/* Error exit handler */
/* Read one byte, testing for EOF */

unsafe extern "C" fn read_1_byte() -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = crate::stdlib::getc(infile);
    if c == crate::stdlib::EOF {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            b"Premature EOF in JPEG file\x00" as *const u8 as *const libc::c_char,
        );
        crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
    }
    return c;
}
/* Read 2 bytes, convert to unsigned int */
/* All 2-byte quantities in JPEG markers are MSB first */

unsafe extern "C" fn read_2_bytes() -> libc::c_uint {
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    c1 = crate::stdlib::getc(infile);
    if c1 == crate::stdlib::EOF {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            b"Premature EOF in JPEG file\x00" as *const u8 as *const libc::c_char,
        );
        crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
    }
    c2 = crate::stdlib::getc(infile);
    if c2 == crate::stdlib::EOF {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            b"Premature EOF in JPEG file\x00" as *const u8 as *const libc::c_char,
        );
        crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
    }
    return ((c1 as libc::c_uint) << 8 as libc::c_int).wrapping_add(c2 as libc::c_uint);
}
/* Routines to write data to output file */

unsafe extern "C" fn write_1_byte(mut c: libc::c_int) {
    crate::stdlib::putc(c, outfile);
}

unsafe extern "C" fn write_2_bytes(mut val: libc::c_uint) {
    crate::stdlib::putc(
        (val >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
        outfile,
    );
    crate::stdlib::putc(
        (val & 0xff as libc::c_int as libc::c_uint) as libc::c_int,
        outfile,
    );
}

unsafe extern "C" fn write_marker(mut marker: libc::c_int) {
    crate::stdlib::putc(0xff as libc::c_int, outfile);
    crate::stdlib::putc(marker, outfile);
}

unsafe extern "C" fn copy_rest_of_file() {
    let mut c: libc::c_int = 0;
    loop {
        c = crate::stdlib::getc(infile);
        if !(c != crate::stdlib::EOF) {
            break;
        }
        crate::stdlib::putc(c, outfile);
    }
}

pub const M_SOI: libc::c_int = 0xd8 as libc::c_int;
/* COMment */
/*
 * Find the next JPEG marker and return its marker code.
 * We expect at least one FF byte, possibly more if the compressor used FFs
 * to pad the file.  (Padding FFs will NOT be replicated in the output file.)
 * There could also be non-FF garbage between markers.  The treatment of such
 * garbage is unspecified; we choose to skip over it but emit a warning msg.
 * NB: this routine must not be used after seeing SOS marker, since it will
 * not deal correctly with FF/00 sequences in the compressed image data...
 */

unsafe extern "C" fn next_marker() -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut discarded_bytes: libc::c_int = 0 as libc::c_int;
    /* Find 0xFF byte; count and skip any non-FFs. */
    c = read_1_byte();
    while c != 0xff as libc::c_int {
        discarded_bytes += 1;
        c = read_1_byte()
    }
    loop
    /* Get marker code byte, swallowing any duplicate FF bytes.  Extra FFs
     * are legal as pad bytes, so don't count them in discarded_bytes.
     */
    {
        c = read_1_byte();
        if !(c == 0xff as libc::c_int) {
            break;
        }
    }
    if discarded_bytes != 0 as libc::c_int {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"Warning: garbage data found in JPEG file\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    return c;
}
/*
 * Read the initial marker, which should be SOI.
 * For a JFIF file, the first two bytes of the file should be literally
 * 0xFF M_SOI.  To be more general, we could use next_marker, but if the
 * input file weren't actually JPEG at all, next_marker might read the whole
 * file and then return a misleading error message...
 */

unsafe extern "C" fn first_marker() -> libc::c_int {
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    c1 = crate::stdlib::getc(infile);
    c2 = crate::stdlib::getc(infile);
    if c1 != 0xff as libc::c_int || c2 != M_SOI {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            b"Not a JPEG file\x00" as *const u8 as *const libc::c_char,
        );
        crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
    }
    return c2;
}
/*
 * Most types of marker are followed by a variable-length parameter segment.
 * This routine skips over the parameters for any marker we don't otherwise
 * want to process.
 * Note that we MUST skip the parameter segment explicitly in order not to
 * be fooled by 0xFF bytes that might appear within the parameter segment;
 * such bytes do NOT introduce new markers.
 */

unsafe extern "C" fn copy_variable()
/* Copy an unknown or uninteresting variable-length marker */
{
    let mut length: libc::c_uint = 0;
    /* Get the marker parameter length count */
    length = read_2_bytes();
    write_2_bytes(length);
    /* Length includes itself, so must be at least 2 */
    if length < 2 as libc::c_int as libc::c_uint {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            b"Erroneous JPEG marker length\x00" as *const u8 as *const libc::c_char,
        );
        crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
    }
    length = length.wrapping_sub(2 as libc::c_int as libc::c_uint);
    /* Copy the remaining bytes */
    while length > 0 as libc::c_int as libc::c_uint {
        write_1_byte(read_1_byte());
        length = length.wrapping_sub(1)
    }
}

unsafe extern "C" fn skip_variable()
/* Skip over an unknown or uninteresting variable-length marker */
{
    let mut length: libc::c_uint = 0;
    /* Get the marker parameter length count */
    length = read_2_bytes();
    /* Length includes itself, so must be at least 2 */
    if length < 2 as libc::c_int as libc::c_uint {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            b"Erroneous JPEG marker length\x00" as *const u8 as *const libc::c_char,
        );
        crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
    }
    length = length.wrapping_sub(2 as libc::c_int as libc::c_uint);
    /* Skip over the remaining bytes */
    while length > 0 as libc::c_int as libc::c_uint {
        read_1_byte();
        length = length.wrapping_sub(1)
    }
}
/*
 * Parse the marker stream until SOFn or EOI is seen;
 * copy data to output, but discard COM markers unless keep_COM is true.
 */

unsafe extern "C" fn scan_JPEG_header(mut keep_COM: libc::c_int) -> libc::c_int {
    let mut marker: libc::c_int = 0;
    /* Expect SOI at start of file */
    if first_marker() != M_SOI {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            b"Expected SOI marker first\x00" as *const u8 as *const libc::c_char,
        );
        crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
    }
    write_marker(M_SOI);
    loop
    /* Scan miscellaneous markers until we reach SOFn. */
    {
        marker = next_marker();
        's_105: {
            let mut current_block_14: u64;
            match marker {
                192 => {
                    /* Baseline */
                    current_block_14 = 7462249631196382341;
                }
                193 => {
                    current_block_14 = 7462249631196382341;
                }
                194 => {
                    current_block_14 = 4732643689795092812;
                }
                195 => {
                    current_block_14 = 4870176945462440287;
                }
                197 => {
                    current_block_14 = 11467873499089977493;
                }
                198 => {
                    current_block_14 = 7725194011799108818;
                }
                199 => {
                    current_block_14 = 10745513706560517726;
                }
                201 => {
                    current_block_14 = 3427529614598637366;
                }
                202 => {
                    current_block_14 = 4895532786262333738;
                }
                203 => {
                    current_block_14 = 9016838511914872075;
                }
                205 => {
                    current_block_14 = 5018439318894558507;
                }
                206 => {
                    current_block_14 = 6371008424711220450;
                }
                207 => {
                    current_block_14 = 3462426091603060970;
                }
                218 => {
                    /* should not see compressed data before SOF */
                    crate::stdlib::fprintf(
                        crate::stdlib::stderr,
                        b"%s\n\x00" as *const u8 as *const libc::c_char,
                        b"SOS without prior SOFn\x00" as *const u8 as *const libc::c_char,
                    );
                    crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
                    current_block_14 = 11584701595673473500;
                }
                217 => {
                    /* in case it's a tables-only JPEG stream */
                    return marker;
                }
                254 => {
                    /* Existing COM: conditionally discard */
                    if keep_COM != 0 {
                        write_marker(marker);
                        copy_variable();
                    } else {
                        skip_variable();
                    }
                    current_block_14 = 11584701595673473500;
                }
                _ => {
                    /* Anything else just gets copied */
                    write_marker(marker); /* we assume it has a parameter count... */
                    copy_variable();
                    current_block_14 = 11584701595673473500;
                }
            }
            match current_block_14 {
                7462249631196382341 =>
                /* Extended sequential, Huffman */
                {
                    current_block_14 = 4732643689795092812;
                }
                11584701595673473500 => {
                    break 's_105;
                }
                _ => {}
            }
            match current_block_14 {
                4732643689795092812 =>
                /* Progressive, Huffman */
                {
                    current_block_14 = 4870176945462440287;
                }
                _ => {}
            }
            match current_block_14 {
                4870176945462440287 =>
                /* Lossless, Huffman */
                {
                    current_block_14 = 11467873499089977493;
                }
                _ => {}
            }
            match current_block_14 {
                11467873499089977493 =>
                /* Differential sequential, Huffman */
                {
                    current_block_14 = 7725194011799108818;
                }
                _ => {}
            }
            match current_block_14 {
                7725194011799108818 =>
                /* Differential progressive, Huffman */
                {
                    current_block_14 = 10745513706560517726;
                }
                _ => {}
            }
            match current_block_14 {
                10745513706560517726 =>
                /* Differential lossless, Huffman */
                {
                    current_block_14 = 3427529614598637366;
                }
                _ => {}
            }
            match current_block_14 {
                3427529614598637366 =>
                /* Extended sequential, arithmetic */
                {
                    current_block_14 = 4895532786262333738;
                }
                _ => {}
            }
            match current_block_14 {
                4895532786262333738 =>
                /* Progressive, arithmetic */
                {
                    current_block_14 = 9016838511914872075;
                }
                _ => {}
            }
            match current_block_14 {
                9016838511914872075 =>
                /* Lossless, arithmetic */
                {
                    current_block_14 = 5018439318894558507;
                }
                _ => {}
            }
            match current_block_14 {
                5018439318894558507 =>
                /* Differential sequential, arithmetic */
                {
                    current_block_14 = 6371008424711220450;
                }
                _ => {}
            }
            match current_block_14 {
                6371008424711220450 =>
                    /* Differential progressive, arithmetic */
                    {}
                _ => {}
            }
            /* Note that marker codes 0xC4, 0xC8, 0xCC are not, and must not be,
             * treated as SOFn.  C4 in particular is actually DHT.
             */
            /* Differential lossless, arithmetic */
            return marker;
        }
    }
    /* end loop */
}
/* Command line parsing code */

static mut progname: *const libc::c_char = 0 as *const libc::c_char;
/* program name for error messages */

unsafe extern "C" fn usage()
/* complain about bad command line */
{
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"wrjpgcom inserts a textual comment in a JPEG file.\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"You can add to or replace any existing comment(s).\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"Usage: %s [switches] \x00" as *const u8 as *const libc::c_char,
        progname,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"[inputfile]\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"Switches (names may be abbreviated):\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -replace         Delete any existing comments\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -comment \"text\"  Insert comment with given text\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"  -cfile name      Read comment from named file\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"Notice that you must put quotes around the comment text\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"when you use -comment.\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"If you do not give either -comment or -cfile on the command line,\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"then the comment text is read from standard input.\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"It can be multiple lines, up to %u characters total.\n\x00" as *const u8
            as *const libc::c_char,
        MAX_COM_LENGTH as libc::c_uint,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"You must specify an input JPEG file name when supplying\n\x00" as *const u8
            as *const libc::c_char,
    );
    crate::stdlib::fprintf(
        crate::stdlib::stderr,
        b"comment text from standard input.\n\x00" as *const u8 as *const libc::c_char,
    );
    crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
}

unsafe extern "C" fn keymatch(
    mut arg: *mut libc::c_char,
    mut keyword: *const libc::c_char,
    mut minchars: libc::c_int,
) -> libc::c_int
/* Case-insensitive matching of (possibly abbreviated) keyword switches. */
/* keyword is the constant keyword (must be lower case already), */
/* minchars is length of minimum legal abbreviation. */ {
    let mut ca: libc::c_int = 0; /* arg longer than keyword, no good */
    let mut ck: libc::c_int = 0;
    let mut nmatched: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh0 = arg;
        arg = arg.offset(1);
        ca = *fresh0 as libc::c_int;
        if !(ca != '\u{0}' as i32) {
            break;
        }
        let fresh1 = keyword;
        keyword = keyword.offset(1);
        ck = *fresh1 as libc::c_int;
        if ck == '\u{0}' as i32 {
            return 0 as libc::c_int;
        }
        if *(*crate::stdlib::__ctype_b_loc()).offset(ca as isize) as libc::c_int
            & crate::stdlib::_ISupper as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            /* count matched characters */
            /* force arg to lcase (assume ck is already) */
            ca = ({
                let mut __res: libc::c_int = 0; /* no good */
                if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = ca;
                        __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                            __c
                        } else {
                            *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = tolower(ca)
                    }
                } else {
                    __res = *(*crate::stdlib::__ctype_tolower_loc()).offset(ca as isize)
                }
                __res
            })
        }
        if ca != ck {
            return 0 as libc::c_int;
        }
        nmatched += 1
    }
    /* reached end of argument; fail if it's too short for unique abbrev */
    if nmatched < minchars {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
    /* A-OK */
}
/*
 * The main program.
 */

unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut argn: libc::c_int = 0;
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut keep_COM: libc::c_int = 1 as libc::c_int;
    let mut comment_arg: *mut libc::c_char = crate::stddef_h::NULL as *mut libc::c_char;
    let mut comment_file: *mut crate::stdlib::FILE =
        crate::stddef_h::NULL as *mut crate::stdlib::FILE;
    let mut comment_length: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut marker: libc::c_int = 0;
    /* On Mac, fetch a command line. */
    progname = *argv.offset(0 as libc::c_int as isize); /* in case C library doesn't provide it */
    if progname.is_null()
        || *progname.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
    {
        progname = b"wrjpgcom\x00" as *const u8 as *const libc::c_char
    }
    /* Parse switches, if any */
    argn = 1 as libc::c_int; /* not switch, must be file name */
    while argn < argc {
        arg = *argv.offset(argn as isize); /* advance over '-' */
        if *arg.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32 {
            break;
        }
        arg = arg.offset(1);
        if keymatch(arg, b"replace\x00" as *const u8 as *const libc::c_char,
                    1 as libc::c_int) != 0 {
            keep_COM = 0 as libc::c_int
        } else if keymatch(arg,
                           b"cfile\x00" as *const u8 as *const libc::c_char,
                           2 as libc::c_int) != 0 {
            argn += 1;
            if argn >= argc { usage(); }
            comment_file =
                crate::stdlib::fopen(*argv.offset(argn as isize),
                      b"r\x00" as *const u8 as *const libc::c_char);
            if comment_file.is_null() {
                crate::stdlib::fprintf(crate::stdlib::stderr,
                        b"%s: can\'t open %s\n\x00" as *const u8 as
                            *const libc::c_char, progname,
                        *argv.offset(argn as isize));
                crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
            }
        } else if keymatch(arg,
                           b"comment\x00" as *const u8 as *const libc::c_char,
                           1 as libc::c_int) != 0 {
            argn += 1;
            if argn >= argc { usage(); }
            comment_arg = *argv.offset(argn as isize);
            /* If the comment text starts with '"', then we are probably running
       * under MS-DOG and must parse out the quoted string ourselves.  Sigh.
       */
            if *comment_arg.offset(0 as libc::c_int as isize) as libc::c_int
                   == '\"' as i32 {
                comment_arg =
                    crate::stdlib::malloc(MAX_COM_LENGTH as crate::stddef_h::size_t) as
                        *mut libc::c_char; /* zap terminating quote */
                if comment_arg.is_null() {
                    crate::stdlib::fprintf(crate::stdlib::stderr,
                            b"%s\n\x00" as *const u8 as *const libc::c_char,
                            b"Insufficient memory\x00" as *const u8 as
                                *const libc::c_char);
                    crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
                }
                if crate::stdlib::strlen(*argv.offset(argn as
                                           isize)).wrapping_add(2 as
                                                                    libc::c_int
                                                                    as
                                                                    libc::c_ulong)
                       >= MAX_COM_LENGTH as crate::stddef_h::size_t {
                    crate::stdlib::fprintf(crate::stdlib::stderr,
                            b"Comment text may not exceed %u bytes\n\x00" as
                                *const u8 as *const libc::c_char,
                            MAX_COM_LENGTH as libc::c_uint);
                    crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
                }
                crate::stdlib::strcpy(comment_arg,
                       (*argv.offset(argn as
                                         isize)).offset(1 as libc::c_int as
                                                            isize));
                loop  {
                    comment_length = crate::stdlib::strlen(comment_arg) as libc::c_uint;
                    if comment_length > 0 as libc::c_int as libc::c_uint &&
                           *comment_arg.offset(comment_length.wrapping_sub(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_uint)
                                                   as isize) as libc::c_int ==
                               '\"' as i32 {
                        *comment_arg.offset(comment_length.wrapping_sub(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_uint)
                                                as isize) =
                            '\u{0}' as i32 as libc::c_char;
                        break ;
                    } else {
                        argn += 1;
                        if argn >= argc {
                            crate::stdlib::fprintf(crate::stdlib::stderr,
                                    b"%s\n\x00" as *const u8 as
                                        *const libc::c_char,
                                    b"Missing ending quote mark\x00" as
                                        *const u8 as *const libc::c_char);
                            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
                        }
                        if crate::stdlib::strlen(comment_arg).wrapping_add(crate::stdlib::strlen(*argv.offset(argn
                                                                                    as
                                                                                    isize))).wrapping_add(2
                                                                                                              as
                                                                                                              libc::c_int
                                                                                                              as
                                                                                                              libc::c_ulong)
                               >= MAX_COM_LENGTH as crate::stddef_h::size_t {
                            crate::stdlib::fprintf(crate::stdlib::stderr,
                                    b"Comment text may not exceed %u bytes\n\x00"
                                        as *const u8 as *const libc::c_char,
                                    MAX_COM_LENGTH as libc::c_uint);
                            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
                        }
                        crate::stdlib::strcat(comment_arg,
                               b" \x00" as *const u8 as *const libc::c_char);
                        crate::stdlib::strcat(comment_arg, *argv.offset(argn as isize));
                    }
                }
            } else if crate::stdlib::strlen(*argv.offset(argn as isize)) >=
                          MAX_COM_LENGTH as crate::stddef_h::size_t {
                crate::stdlib::fprintf(crate::stdlib::stderr,
                        b"Comment text may not exceed %u bytes\n\x00" as
                            *const u8 as *const libc::c_char,
                        MAX_COM_LENGTH as libc::c_uint);
                crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
            }
            comment_length = crate::stdlib::strlen(comment_arg) as libc::c_uint
        } else { usage(); }
        argn += 1
    }
    /* Cannot use both -comment and -cfile. */
    if !comment_arg.is_null() && !comment_file.is_null() {
        usage();
    }
    /* If there is neither -comment nor -cfile, we will read the comment text
     * from stdin; in this case there MUST be an input JPEG file name.
     */
    if comment_arg.is_null() && comment_file.is_null() && argn >= argc {
        usage();
    }
    /* Open the input file. */
    if argn < argc {
        infile = crate::stdlib::fopen(*argv.offset(argn as isize), READ_BINARY.as_ptr());
        if infile.is_null() {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s: can\'t open %s\n\x00" as *const u8 as *const libc::c_char,
                progname,
                *argv.offset(argn as isize),
            );
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
    } else {
        /* default input file is stdin */
        /* need to hack file mode? */
        /* need to re-open in binary mode? */
        infile = crate::stdlib::stdin
    }
    /* Open the output file. */
    /* Unix style: expect zero or one file name */
    if argn < argc - 1 as libc::c_int {
        crate::stdlib::fprintf(
            crate::stdlib::stderr,
            b"%s: only one input file\n\x00" as *const u8 as *const libc::c_char,
            progname,
        );
        usage();
    }
    /* default output file is stdout */
    /* need to hack file mode? */
    /* need to re-open in binary mode? */
    outfile = crate::stdlib::stdout;
    /* TWO_FILE_COMMANDLINE */
    /* Collect comment text from comment_file or stdin, if necessary */
    if comment_arg.is_null() {
        let mut src_file: *mut crate::stdlib::FILE = 0 as *mut crate::stdlib::FILE;
        let mut c: libc::c_int = 0;
        comment_arg =
            crate::stdlib::malloc(MAX_COM_LENGTH as crate::stddef_h::size_t) as *mut libc::c_char;
        if comment_arg.is_null() {
            crate::stdlib::fprintf(
                crate::stdlib::stderr,
                b"%s\n\x00" as *const u8 as *const libc::c_char,
                b"Insufficient memory\x00" as *const u8 as *const libc::c_char,
            );
            crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
        }
        comment_length = 0 as libc::c_int as libc::c_uint;
        src_file = if !comment_file.is_null() {
            comment_file
        } else {
            crate::stdlib::stdin
        };
        loop {
            c = crate::stdlib::getc(src_file);
            if !(c != crate::stdlib::EOF) {
                break;
            }
            if comment_length >= MAX_COM_LENGTH as libc::c_uint {
                crate::stdlib::fprintf(
                    crate::stdlib::stderr,
                    b"Comment text may not exceed %u bytes\n\x00" as *const u8
                        as *const libc::c_char,
                    MAX_COM_LENGTH as libc::c_uint,
                );
                crate::stdlib::exit(crate::stdlib::EXIT_FAILURE);
            }
            let fresh2 = comment_length;
            comment_length = comment_length.wrapping_add(1);
            *comment_arg.offset(fresh2 as isize) = c as libc::c_char
        }
        if !comment_file.is_null() {
            crate::stdlib::fclose(comment_file);
        }
    }
    /* Copy JPEG headers until SOFn marker;
     * we will insert the new comment marker just before SOFn.
     * This (a) causes the new comment to appear after, rather than before,
     * existing comments; and (b) ensures that comments come after any JFIF
     * or JFXX markers, as required by the JFIF specification.
     */
    marker = scan_JPEG_header(keep_COM);
    /* Insert the new COM marker, but only if nonempty text has been supplied */
    if comment_length > 0 as libc::c_int as libc::c_uint {
        write_marker(0xfe as libc::c_int);
        write_2_bytes(comment_length.wrapping_add(2 as libc::c_int as libc::c_uint));
        while comment_length > 0 as libc::c_int as libc::c_uint {
            let fresh3 = comment_arg;
            comment_arg = comment_arg.offset(1);
            write_1_byte(*fresh3 as libc::c_int);
            comment_length = comment_length.wrapping_sub(1)
        }
    }
    /* Duplicate the remainder of the source file.
     * Note that any COM markers occuring after SOF will not be touched.
     */
    write_marker(marker);
    copy_rest_of_file();
    /* All done. */
    crate::stdlib::exit(crate::stdlib::EXIT_SUCCESS);
    /* suppress no-return-value warnings */
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
