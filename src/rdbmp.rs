pub use crate::cderror_h::{
    C2RustUnnamed_91, JERR_BAD_CMAP_FILE, JERR_BMP_BADCMAP, JERR_BMP_BADDEPTH, JERR_BMP_BADHEADER,
    JERR_BMP_BADPLANES, JERR_BMP_COLORSPACE, JERR_BMP_COMPRESSED, JERR_BMP_EMPTY, JERR_BMP_NOT,
    JERR_BMP_OUTOFRANGE, JERR_GIF_BUG, JERR_GIF_CODESIZE, JERR_GIF_COLORSPACE,
    JERR_GIF_IMAGENOTFOUND, JERR_GIF_NOT, JERR_PPM_COLORSPACE, JERR_PPM_NONNUMERIC, JERR_PPM_NOT,
    JERR_PPM_OUTOFRANGE, JERR_TGA_BADCMAP, JERR_TGA_BADPARMS, JERR_TGA_COLORSPACE,
    JERR_TOO_MANY_COLORS, JERR_UNGETC_FAILED, JERR_UNKNOWN_FORMAT, JERR_UNSUPPORTED_FORMAT,
    JMSG_FIRSTADDONCODE, JMSG_LASTADDONCODE, JTRC_BMP, JTRC_BMP_MAPPED, JTRC_BMP_OS2,
    JTRC_BMP_OS2_MAPPED, JTRC_GIF, JTRC_GIF_BADVERSION, JTRC_GIF_EXTENSION, JTRC_GIF_NONSQUARE,
    JTRC_PGM, JTRC_PGM_TEXT, JTRC_PPM, JTRC_PPM_TEXT, JTRC_TGA, JTRC_TGA_GRAY, JTRC_TGA_MAPPED,
    JWRN_GIF_BADDATA, JWRN_GIF_CHAR, JWRN_GIF_ENDCODE, JWRN_GIF_NOMOREDATA,
};
pub use crate::cdjpeg::{
    cd_progress_ptr, cdjpeg_progress_mgr, cjpeg_source_ptr, cjpeg_source_struct,
};
pub use crate::cmyk_h::rgb_to_cmyk;
pub use crate::jerror::{
    C2RustUnnamed_3, JERR_ARITH_NOTIMPL, JERR_BAD_ALIGN_TYPE, JERR_BAD_ALLOC_CHUNK,
    JERR_BAD_BUFFER_MODE, JERR_BAD_COMPONENT_ID, JERR_BAD_CROP_SPEC, JERR_BAD_DCTSIZE,
    JERR_BAD_DCT_COEF, JERR_BAD_HUFF_TABLE, JERR_BAD_IN_COLORSPACE, JERR_BAD_J_COLORSPACE,
    JERR_BAD_LENGTH, JERR_BAD_LIB_VERSION, JERR_BAD_MCU_SIZE, JERR_BAD_PARAM, JERR_BAD_PARAM_VALUE,
    JERR_BAD_POOL_ID, JERR_BAD_PRECISION, JERR_BAD_PROGRESSION, JERR_BAD_PROG_SCRIPT,
    JERR_BAD_SAMPLING, JERR_BAD_SCAN_SCRIPT, JERR_BAD_STATE, JERR_BAD_STRUCT_SIZE,
    JERR_BAD_VIRTUAL_ACCESS, JERR_BUFFER_SIZE, JERR_CANT_SUSPEND, JERR_CCIR601_NOTIMPL,
    JERR_COMPONENT_COUNT, JERR_CONVERSION_NOTIMPL, JERR_DAC_INDEX, JERR_DAC_VALUE, JERR_DHT_INDEX,
    JERR_DQT_INDEX, JERR_EMPTY_IMAGE, JERR_EMS_READ, JERR_EMS_WRITE, JERR_EOI_EXPECTED,
    JERR_FILE_READ, JERR_FILE_WRITE, JERR_FRACT_SAMPLE_NOTIMPL, JERR_HUFF_CLEN_OVERFLOW,
    JERR_HUFF_MISSING_CODE, JERR_IMAGE_TOO_BIG, JERR_INPUT_EMPTY, JERR_INPUT_EOF,
    JERR_MISMATCHED_QUANT_TABLE, JERR_MISSING_DATA, JERR_MODE_CHANGE, JERR_NOTIMPL,
    JERR_NOT_COMPILED, JERR_NO_BACKING_STORE, JERR_NO_HUFF_TABLE, JERR_NO_IMAGE,
    JERR_NO_QUANT_TABLE, JERR_NO_SOI, JERR_OUT_OF_MEMORY, JERR_QUANT_COMPONENTS,
    JERR_QUANT_FEW_COLORS, JERR_QUANT_MANY_COLORS, JERR_SOF_DUPLICATE, JERR_SOF_NO_SOS,
    JERR_SOF_UNSUPPORTED, JERR_SOI_DUPLICATE, JERR_SOS_NO_SOF, JERR_TFILE_CREATE, JERR_TFILE_READ,
    JERR_TFILE_SEEK, JERR_TFILE_WRITE, JERR_TOO_LITTLE_DATA, JERR_UNKNOWN_MARKER,
    JERR_UNSUPPORTED_SUSPEND, JERR_VIRTUAL_BUG, JERR_WIDTH_OVERFLOW, JERR_XMS_READ, JERR_XMS_WRITE,
    JMSG_COPYRIGHT, JMSG_LASTMSGCODE, JMSG_NOMESSAGE, JMSG_VERSION, JTRC_16BIT_TABLES, JTRC_ADOBE,
    JTRC_APP0, JTRC_APP14, JTRC_DAC, JTRC_DHT, JTRC_DQT, JTRC_DRI, JTRC_EMS_CLOSE, JTRC_EMS_OPEN,
    JTRC_EOI, JTRC_HUFFBITS, JTRC_JFIF, JTRC_JFIF_BADTHUMBNAILSIZE, JTRC_JFIF_EXTENSION,
    JTRC_JFIF_THUMBNAIL, JTRC_MISC_MARKER, JTRC_PARMLESS_MARKER, JTRC_QUANTVALS,
    JTRC_QUANT_3_NCOLORS, JTRC_QUANT_NCOLORS, JTRC_QUANT_SELECTED, JTRC_RECOVERY_ACTION, JTRC_RST,
    JTRC_SMOOTH_NOTIMPL, JTRC_SOF, JTRC_SOF_COMPONENT, JTRC_SOI, JTRC_SOS, JTRC_SOS_COMPONENT,
    JTRC_SOS_PARAMS, JTRC_TFILE_CLOSE, JTRC_TFILE_OPEN, JTRC_THUMB_JPEG, JTRC_THUMB_PALETTE,
    JTRC_THUMB_RGB, JTRC_UNKNOWN_IDS, JTRC_XMS_CLOSE, JTRC_XMS_OPEN, JWRN_ADOBE_XFORM,
    JWRN_BOGUS_ICC, JWRN_BOGUS_PROGRESSION, JWRN_EXTRANEOUS_DATA, JWRN_HIT_MARKER,
    JWRN_HUFF_BAD_CODE, JWRN_JFIF_MAJOR, JWRN_JPEG_EOF, JWRN_MUST_RESYNC, JWRN_NOT_SEQUENTIAL,
    JWRN_TOO_MUCH_DATA,
};
pub use crate::jmorecfg_h::{
    boolean, rgb_blue, rgb_green, rgb_pixelsize, rgb_red, EXT_BGRX_BLUE, EXT_BGRX_GREEN,
    EXT_BGRX_PIXELSIZE, EXT_BGRX_RED, EXT_BGR_BLUE, EXT_BGR_GREEN, EXT_BGR_PIXELSIZE, EXT_BGR_RED,
    EXT_RGBX_BLUE, EXT_RGBX_GREEN, EXT_RGBX_PIXELSIZE, EXT_RGBX_RED, EXT_RGB_BLUE, EXT_RGB_GREEN,
    EXT_RGB_PIXELSIZE, EXT_RGB_RED, EXT_XBGR_BLUE, EXT_XBGR_GREEN, EXT_XBGR_PIXELSIZE,
    EXT_XBGR_RED, EXT_XRGB_BLUE, EXT_XRGB_GREEN, EXT_XRGB_PIXELSIZE, EXT_XRGB_RED, FALSE, JCOEF,
    JDIMENSION, JOCTET, JSAMPLE, RGB_BLUE, RGB_GREEN, RGB_PIXELSIZE, RGB_RED, TRUE, UINT16, UINT8,
};
pub use crate::jpegint_h::{
    jpeg_c_coef_controller, jpeg_c_main_controller, jpeg_c_prep_controller, jpeg_color_converter,
    jpeg_comp_master, jpeg_downsampler, jpeg_entropy_encoder, jpeg_forward_dct, jpeg_marker_writer,
    JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE,
    J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_compress_ptr, jpeg_common_struct, jpeg_component_info, jpeg_compress_struct,
    jpeg_destination_mgr, jpeg_error_mgr, jpeg_marker_struct, jpeg_memory_mgr, jpeg_progress_mgr,
    jpeg_saved_marker_ptr, jpeg_scan_info, jvirt_barray_control, jvirt_barray_ptr,
    jvirt_sarray_control, jvirt_sarray_ptr, C2RustUnnamed_2, JCS_YCbCr, JBLOCK, JBLOCKARRAY,
    JBLOCKROW, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX,
    JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB,
    JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JHUFF_TBL, JPOOL_IMAGE,
    JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD,
};
pub use crate::stddef_h::{size_t, NULL};
use crate::stdlib::memcpy;
pub use crate::stdlib::{
    _IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, __off64_t, __off_t, feof, fread, getc, EOF,
    FILE, _IO_FILE,
};
use libc::{self, c_int, c_long, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort, c_void};
/* Private version of data source object */
pub type bmp_source_ptr = *mut _bmp_source_struct;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _bmp_source_struct {
    pub pub_0: cjpeg_source_struct,
    pub cinfo: j_compress_ptr,
    pub colormap: JSAMPARRAY,
    pub whole_image: jvirt_sarray_ptr,
    pub source_row: JDIMENSION,
    pub row_width: JDIMENSION,
    pub bits_per_pixel: c_int,
    pub cmap_length: c_int,
    pub use_inversion_array: boolean,
    pub iobuffer: *mut U_CHAR,
}
/*
 * rdbmp.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1996, Thomas G. Lane.
 * Modified 2009-2017 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Modified 2011 by Siarhei Siamashka.
 * Copyright (C) 2015, 2017-2018, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains routines to read input images in Microsoft "BMP"
 * format (MS Windows 3.x, OS/2 1.x, and OS/2 2.x flavors).
 * Currently, only 8-bit and 24-bit images are supported, not 1-bit or
 * 4-bit (feeding such low-depth images into JPEG would be silly anyway).
 * Also, we don't support RLE-compressed files.
 *
 * These routines may need modification for non-Unix environments or
 * specialized applications.  As they stand, they assume input from
 * an ordinary stdio stream.  They further assume that reading begins
 * at the start of the file; start_input may need work if the
 * user interface has already read some data (e.g., to determine that
 * the file is indeed BMP format).
 *
 * This code contributed by James Arthur Boucher.
 */
/* Macros to deal with unsigned chars as efficiently as compiler allows */
pub type U_CHAR = c_uchar;
pub type bmp_source_struct = _bmp_source_struct;
/* !HAVE_UNSIGNED_CHAR */
/* HAVE_UNSIGNED_CHAR */
static mut alpha_index: [c_int; 17] = [
    -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, -1i32, 3i32, 3i32,
    0i32, 0i32, -1i32,
];
unsafe extern "C" fn read_byte(mut sinfo: bmp_source_ptr) -> c_int {
    let mut infile: *mut FILE = (*sinfo).pub_0.input_file;
    let mut c: c_int = 0;
    c = getc(infile);
    if c == EOF {
        (*(*(*sinfo).cinfo).err).msg_code = JERR_INPUT_EOF as c_int;
        (*(*(*sinfo).cinfo).err)
            .error_exit
            .expect("non-null function pointer")((*sinfo).cinfo as j_common_ptr);
    }
    return c;
}
unsafe extern "C" fn read_colormap(
    mut sinfo: bmp_source_ptr,
    mut cmaplen: c_int,
    mut mapentrysize: c_int,
) {
    let mut i: c_int = 0;
    let mut gray: c_int = 1i32;
    match mapentrysize {
        3 => {
            i = 0i32;
            while i < cmaplen {
                *(*(*sinfo).colormap.offset(2isize)).offset(i as isize) =
                    read_byte(sinfo) as JSAMPLE;
                *(*(*sinfo).colormap.offset(1isize)).offset(i as isize) =
                    read_byte(sinfo) as JSAMPLE;
                *(*(*sinfo).colormap.offset(0isize)).offset(i as isize) =
                    read_byte(sinfo) as JSAMPLE;
                if *(*(*sinfo).colormap.offset(2isize)).offset(i as isize) as c_int
                    != *(*(*sinfo).colormap.offset(1isize)).offset(i as isize) as c_int
                    || *(*(*sinfo).colormap.offset(1isize)).offset(i as isize) as c_int
                        != *(*(*sinfo).colormap.offset(0isize)).offset(i as isize) as c_int
                {
                    gray = 0i32
                }
                i += 1
            }
        }
        4 => {
            i = 0i32;
            while i < cmaplen {
                *(*(*sinfo).colormap.offset(2isize)).offset(i as isize) =
                    read_byte(sinfo) as JSAMPLE;
                *(*(*sinfo).colormap.offset(1isize)).offset(i as isize) =
                    read_byte(sinfo) as JSAMPLE;
                *(*(*sinfo).colormap.offset(0isize)).offset(i as isize) =
                    read_byte(sinfo) as JSAMPLE;
                read_byte(sinfo);
                if *(*(*sinfo).colormap.offset(2isize)).offset(i as isize) as c_int
                    != *(*(*sinfo).colormap.offset(1isize)).offset(i as isize) as c_int
                    || *(*(*sinfo).colormap.offset(1isize)).offset(i as isize) as c_int
                        != *(*(*sinfo).colormap.offset(0isize)).offset(i as isize) as c_int
                {
                    gray = 0i32
                }
                i += 1
            }
        }
        _ => {
            (*(*(*sinfo).cinfo).err).msg_code = JERR_BMP_BADCMAP as c_int;
            (*(*(*sinfo).cinfo).err)
                .error_exit
                .expect("non-null function pointer")((*sinfo).cinfo as j_common_ptr);
        }
    }
    if (*(*sinfo).cinfo).in_color_space as c_uint == JCS_UNKNOWN as c_int as c_uint && 0 != gray {
        (*(*sinfo).cinfo).in_color_space = JCS_GRAYSCALE
    }
    if (*(*sinfo).cinfo).in_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint && 0 == gray {
        (*(*(*sinfo).cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as c_int;
        (*(*(*sinfo).cinfo).err)
            .error_exit
            .expect("non-null function pointer")((*sinfo).cinfo as j_common_ptr);
    };
}
/*
 * Read one row of pixels.
 * The image has been read into the whole_image array, but is otherwise
 * unprocessed.  We must read it out in top-to-bottom row order, and if
 * it is an 8-bit image, we must expand colormapped pixels to 24bit format.
 */
unsafe extern "C" fn get_8bit_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: bmp_source_ptr = sinfo as bmp_source_ptr;
    let mut colormap: JSAMPARRAY = (*source).colormap;
    let mut cmaplen: c_int = (*source).cmap_length;
    let mut image_ptr: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut t: c_int = 0;
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    if 0 != (*source).use_inversion_array {
        (*source).source_row = (*source).source_row.wrapping_sub(1);
        image_ptr = (*(*cinfo).mem)
            .access_virt_sarray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*source).whole_image,
            (*source).source_row,
            1i32 as JDIMENSION,
            FALSE,
        );
        inptr = *image_ptr.offset(0isize)
    } else {
        if !(fread(
            (*source).iobuffer as *mut c_void,
            1i32 as size_t,
            (*source).row_width as size_t,
            (*source).pub_0.input_file,
        ) == (*source).row_width as size_t)
        {
            (*(*cinfo).err).msg_code = JERR_INPUT_EOF as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        inptr = (*source).iobuffer
    }
    outptr = *(*source).pub_0.buffer.offset(0isize);
    if (*cinfo).in_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let fresh0 = inptr;
            inptr = inptr.offset(1);
            t = *fresh0 as c_int;
            if t >= cmaplen {
                (*(*cinfo).err).msg_code = JERR_BMP_OUTOFRANGE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            let fresh1 = outptr;
            outptr = outptr.offset(1);
            *fresh1 = *(*colormap.offset(0isize)).offset(t as isize);
            col = col.wrapping_sub(1)
        }
    } else if (*cinfo).in_color_space as c_uint == JCS_CMYK as c_int as c_uint {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let fresh2 = inptr;
            inptr = inptr.offset(1);
            t = *fresh2 as c_int;
            if t >= cmaplen {
                (*(*cinfo).err).msg_code = JERR_BMP_OUTOFRANGE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            rgb_to_cmyk(
                *(*colormap.offset(0isize)).offset(t as isize),
                *(*colormap.offset(1isize)).offset(t as isize),
                *(*colormap.offset(2isize)).offset(t as isize),
                outptr,
                outptr.offset(1isize),
                outptr.offset(2isize),
                outptr.offset(3isize),
            );
            outptr = outptr.offset(4isize);
            col = col.wrapping_sub(1)
        }
    } else {
        let mut rindex: c_int = rgb_red[(*cinfo).in_color_space as usize];
        let mut gindex: c_int = rgb_green[(*cinfo).in_color_space as usize];
        let mut bindex: c_int = rgb_blue[(*cinfo).in_color_space as usize];
        let mut aindex: c_int = alpha_index[(*cinfo).in_color_space as usize];
        let mut ps: c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
        if aindex >= 0i32 {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                let fresh3 = inptr;
                inptr = inptr.offset(1);
                t = *fresh3 as c_int;
                if t >= cmaplen {
                    (*(*cinfo).err).msg_code = JERR_BMP_OUTOFRANGE as c_int;
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer")(
                        cinfo as j_common_ptr
                    );
                }
                *outptr.offset(rindex as isize) = *(*colormap.offset(0isize)).offset(t as isize);
                *outptr.offset(gindex as isize) = *(*colormap.offset(1isize)).offset(t as isize);
                *outptr.offset(bindex as isize) = *(*colormap.offset(2isize)).offset(t as isize);
                *outptr.offset(aindex as isize) = 0xffi32 as JSAMPLE;
                outptr = outptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                let fresh4 = inptr;
                inptr = inptr.offset(1);
                t = *fresh4 as c_int;
                if t >= cmaplen {
                    (*(*cinfo).err).msg_code = JERR_BMP_OUTOFRANGE as c_int;
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer")(
                        cinfo as j_common_ptr
                    );
                }
                *outptr.offset(rindex as isize) = *(*colormap.offset(0isize)).offset(t as isize);
                *outptr.offset(gindex as isize) = *(*colormap.offset(1isize)).offset(t as isize);
                *outptr.offset(bindex as isize) = *(*colormap.offset(2isize)).offset(t as isize);
                outptr = outptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        }
    }
    return 1i32 as JDIMENSION;
}
unsafe extern "C" fn get_24bit_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: bmp_source_ptr = sinfo as bmp_source_ptr;
    let mut image_ptr: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    if 0 != (*source).use_inversion_array {
        (*source).source_row = (*source).source_row.wrapping_sub(1);
        image_ptr = (*(*cinfo).mem)
            .access_virt_sarray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*source).whole_image,
            (*source).source_row,
            1i32 as JDIMENSION,
            FALSE,
        );
        inptr = *image_ptr.offset(0isize)
    } else {
        if !(fread(
            (*source).iobuffer as *mut c_void,
            1i32 as size_t,
            (*source).row_width as size_t,
            (*source).pub_0.input_file,
        ) == (*source).row_width as size_t)
        {
            (*(*cinfo).err).msg_code = JERR_INPUT_EOF as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        inptr = (*source).iobuffer
    }
    outptr = *(*source).pub_0.buffer.offset(0isize);
    if (*cinfo).in_color_space as c_uint == JCS_EXT_BGR as c_int as c_uint {
        memcpy(
            outptr as *mut c_void,
            inptr as *const c_void,
            (*source).row_width as size_t,
        );
    } else if (*cinfo).in_color_space as c_uint == JCS_CMYK as c_int as c_uint {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let fresh5 = inptr;
            inptr = inptr.offset(1);
            let mut b: JSAMPLE = *fresh5;
            let fresh6 = inptr;
            inptr = inptr.offset(1);
            let mut g: JSAMPLE = *fresh6;
            let fresh7 = inptr;
            inptr = inptr.offset(1);
            let mut r: JSAMPLE = *fresh7;
            rgb_to_cmyk(
                r,
                g,
                b,
                outptr,
                outptr.offset(1isize),
                outptr.offset(2isize),
                outptr.offset(3isize),
            );
            outptr = outptr.offset(4isize);
            col = col.wrapping_sub(1)
        }
    } else {
        let mut rindex: c_int = rgb_red[(*cinfo).in_color_space as usize];
        let mut gindex: c_int = rgb_green[(*cinfo).in_color_space as usize];
        let mut bindex: c_int = rgb_blue[(*cinfo).in_color_space as usize];
        let mut aindex: c_int = alpha_index[(*cinfo).in_color_space as usize];
        let mut ps: c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
        if aindex >= 0i32 {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                let fresh8 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(bindex as isize) = *fresh8;
                let fresh9 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(gindex as isize) = *fresh9;
                let fresh10 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(rindex as isize) = *fresh10;
                *outptr.offset(aindex as isize) = 0xffi32 as JSAMPLE;
                outptr = outptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                let fresh11 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(bindex as isize) = *fresh11;
                let fresh12 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(gindex as isize) = *fresh12;
                let fresh13 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(rindex as isize) = *fresh13;
                outptr = outptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        }
    }
    return 1i32 as JDIMENSION;
}
unsafe extern "C" fn get_32bit_row(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: bmp_source_ptr = sinfo as bmp_source_ptr;
    let mut image_ptr: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut inptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut col: JDIMENSION = 0;
    if 0 != (*source).use_inversion_array {
        (*source).source_row = (*source).source_row.wrapping_sub(1);
        image_ptr = (*(*cinfo).mem)
            .access_virt_sarray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*source).whole_image,
            (*source).source_row,
            1i32 as JDIMENSION,
            FALSE,
        );
        inptr = *image_ptr.offset(0isize)
    } else {
        if !(fread(
            (*source).iobuffer as *mut c_void,
            1i32 as size_t,
            (*source).row_width as size_t,
            (*source).pub_0.input_file,
        ) == (*source).row_width as size_t)
        {
            (*(*cinfo).err).msg_code = JERR_INPUT_EOF as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        inptr = (*source).iobuffer
    }
    outptr = *(*source).pub_0.buffer.offset(0isize);
    if (*cinfo).in_color_space as c_uint == JCS_EXT_BGRX as c_int as c_uint
        || (*cinfo).in_color_space as c_uint == JCS_EXT_BGRA as c_int as c_uint
    {
        memcpy(
            outptr as *mut c_void,
            inptr as *const c_void,
            (*source).row_width as size_t,
        );
    } else if (*cinfo).in_color_space as c_uint == JCS_CMYK as c_int as c_uint {
        col = (*cinfo).image_width;
        while col > 0i32 as c_uint {
            let fresh14 = inptr;
            inptr = inptr.offset(1);
            let mut b: JSAMPLE = *fresh14;
            let fresh15 = inptr;
            inptr = inptr.offset(1);
            let mut g: JSAMPLE = *fresh15;
            let fresh16 = inptr;
            inptr = inptr.offset(1);
            let mut r: JSAMPLE = *fresh16;
            rgb_to_cmyk(
                r,
                g,
                b,
                outptr,
                outptr.offset(1isize),
                outptr.offset(2isize),
                outptr.offset(3isize),
            );
            inptr = inptr.offset(1isize);
            outptr = outptr.offset(4isize);
            col = col.wrapping_sub(1)
        }
    } else {
        let mut rindex: c_int = rgb_red[(*cinfo).in_color_space as usize];
        let mut gindex: c_int = rgb_green[(*cinfo).in_color_space as usize];
        let mut bindex: c_int = rgb_blue[(*cinfo).in_color_space as usize];
        let mut aindex: c_int = alpha_index[(*cinfo).in_color_space as usize];
        let mut ps: c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
        if aindex >= 0i32 {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                let fresh17 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(bindex as isize) = *fresh17;
                let fresh18 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(gindex as isize) = *fresh18;
                let fresh19 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(rindex as isize) = *fresh19;
                let fresh20 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(aindex as isize) = *fresh20;
                outptr = outptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0i32 as c_uint {
                let fresh21 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(bindex as isize) = *fresh21;
                let fresh22 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(gindex as isize) = *fresh22;
                let fresh23 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(rindex as isize) = *fresh23;
                inptr = inptr.offset(1isize);
                outptr = outptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        }
    }
    return 1i32 as JDIMENSION;
}
/*
 * This method loads the image into whole_image during the first call on
 * get_pixel_rows.  The get_pixel_rows pointer is then adjusted to call
 * get_8bit_row, get_24bit_row, or get_32bit_row on subsequent calls.
 */
unsafe extern "C" fn preload_image(
    mut cinfo: j_compress_ptr,
    mut sinfo: cjpeg_source_ptr,
) -> JDIMENSION {
    let mut source: bmp_source_ptr = sinfo as bmp_source_ptr;
    let mut infile: *mut FILE = (*source).pub_0.input_file;
    let mut out_ptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut image_ptr: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut row: JDIMENSION = 0;
    let mut progress: cd_progress_ptr = (*cinfo).progress as cd_progress_ptr;
    row = 0i32 as JDIMENSION;
    while row < (*cinfo).image_height {
        if !progress.is_null() {
            (*progress).pub_0.pass_counter = row as c_long;
            (*progress).pub_0.pass_limit = (*cinfo).image_height as c_long;
            (*progress)
                .pub_0
                .progress_monitor
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        image_ptr = (*(*cinfo).mem)
            .access_virt_sarray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            (*source).whole_image,
            row,
            1i32 as JDIMENSION,
            TRUE,
        );
        out_ptr = *image_ptr.offset(0isize);
        if fread(
            out_ptr as *mut c_void,
            1i32 as c_ulong,
            (*source).row_width as c_ulong,
            infile,
        ) != (*source).row_width as c_ulong
        {
            if 0 != feof(infile) {
                (*(*cinfo).err).msg_code = JERR_INPUT_EOF as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            } else {
                (*(*cinfo).err).msg_code = JERR_FILE_READ as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
        row = row.wrapping_add(1)
    }
    if !progress.is_null() {
        (*progress).completed_extra_passes += 1
    }
    match (*source).bits_per_pixel {
        8 => {
            (*source).pub_0.get_pixel_rows = Some(
                get_8bit_row
                    as unsafe extern "C" fn(_: j_compress_ptr, _: cjpeg_source_ptr) -> JDIMENSION,
            )
        }
        24 => {
            (*source).pub_0.get_pixel_rows = Some(
                get_24bit_row
                    as unsafe extern "C" fn(_: j_compress_ptr, _: cjpeg_source_ptr) -> JDIMENSION,
            )
        }
        32 => {
            (*source).pub_0.get_pixel_rows = Some(
                get_32bit_row
                    as unsafe extern "C" fn(_: j_compress_ptr, _: cjpeg_source_ptr) -> JDIMENSION,
            )
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_BMP_BADDEPTH as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    }
    (*source).source_row = (*cinfo).image_height;
    return (*source)
        .pub_0
        .get_pixel_rows
        .expect("non-null function pointer")(cinfo, sinfo);
}
/*
 * Read the file header; return image size and component count.
 */
unsafe extern "C" fn start_input_bmp(mut cinfo: j_compress_ptr, mut sinfo: cjpeg_source_ptr) {
    let mut source: bmp_source_ptr = sinfo as bmp_source_ptr;
    let mut bmpfileheader: [U_CHAR; 14] = [0; 14];
    let mut bmpinfoheader: [U_CHAR; 64] = [0; 64];
    let mut bfOffBits: c_uint = 0;
    let mut headerSize: c_uint = 0;
    let mut biWidth: c_int = 0;
    let mut biHeight: c_int = 0;
    let mut biPlanes: c_ushort = 0;
    let mut biCompression: c_uint = 0;
    let mut biXPelsPerMeter: c_int = 0;
    let mut biYPelsPerMeter: c_int = 0;
    let mut biClrUsed: c_uint = 0i32 as c_uint;
    /* 0 indicates no colormap */
    let mut mapentrysize: c_int = 0i32;
    let mut bPad: c_int = 0;
    let mut row_width: JDIMENSION = 0i32 as JDIMENSION;
    if !(fread(
        bmpfileheader.as_mut_ptr() as *mut c_void,
        1i32 as size_t,
        14i32 as size_t,
        (*source).pub_0.input_file,
    ) == 14i32 as size_t)
    {
        (*(*cinfo).err).msg_code = JERR_INPUT_EOF as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if bmpfileheader[0usize] as c_int as c_ushort as c_int
        + ((bmpfileheader[(0i32 + 1i32) as usize] as c_int as c_ushort as c_int) << 8i32)
        != 0x4d42i32
    {
        (*(*cinfo).err).msg_code = JERR_BMP_NOT as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    bfOffBits = (bmpfileheader[10usize] as c_int as c_uint)
        .wrapping_add((bmpfileheader[(10i32 + 1i32) as usize] as c_int as c_uint) << 8i32)
        .wrapping_add((bmpfileheader[(10i32 + 2i32) as usize] as c_int as c_uint) << 16i32)
        .wrapping_add((bmpfileheader[(10i32 + 3i32) as usize] as c_int as c_uint) << 24i32);
    if !(fread(
        bmpinfoheader.as_mut_ptr() as *mut c_void,
        1i32 as size_t,
        4i32 as size_t,
        (*source).pub_0.input_file,
    ) == 4i32 as size_t)
    {
        (*(*cinfo).err).msg_code = JERR_INPUT_EOF as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    headerSize = (bmpinfoheader[0usize] as c_int as c_uint)
        .wrapping_add((bmpinfoheader[(0i32 + 1i32) as usize] as c_int as c_uint) << 8i32)
        .wrapping_add((bmpinfoheader[(0i32 + 2i32) as usize] as c_int as c_uint) << 16i32)
        .wrapping_add((bmpinfoheader[(0i32 + 3i32) as usize] as c_int as c_uint) << 24i32);
    if headerSize < 12i32 as c_uint || headerSize > 64i32 as c_uint {
        (*(*cinfo).err).msg_code = JERR_BMP_BADHEADER as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if !(fread(
        bmpinfoheader.as_mut_ptr().offset(4isize) as *mut c_void,
        1i32 as size_t,
        headerSize.wrapping_sub(4i32 as c_uint) as size_t,
        (*source).pub_0.input_file,
    ) == headerSize.wrapping_sub(4i32 as c_uint) as size_t)
    {
        (*(*cinfo).err).msg_code = JERR_INPUT_EOF as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    match headerSize {
        12 => {
            biWidth = bmpinfoheader[4usize] as c_int as c_ushort as c_int
                + ((bmpinfoheader[(4i32 + 1i32) as usize] as c_int as c_ushort as c_int) << 8i32);
            biHeight = bmpinfoheader[6usize] as c_int as c_ushort as c_int
                + ((bmpinfoheader[(6i32 + 1i32) as usize] as c_int as c_ushort as c_int) << 8i32);
            biPlanes = (bmpinfoheader[8usize] as c_int as c_ushort as c_int
                + ((bmpinfoheader[(8i32 + 1i32) as usize] as c_int as c_ushort as c_int) << 8i32))
                as c_ushort;
            (*source).bits_per_pixel = bmpinfoheader[10usize] as c_int as c_ushort as c_int
                + ((bmpinfoheader[(10i32 + 1i32) as usize] as c_int as c_ushort as c_int) << 8i32);
            match (*source).bits_per_pixel {
                8 => {
                    mapentrysize = 3i32;
                    (*(*cinfo).err).msg_code = JTRC_BMP_OS2_MAPPED as c_int;
                    (*(*cinfo).err).msg_parm.i[0usize] = biWidth;
                    (*(*cinfo).err).msg_parm.i[1usize] = biHeight;
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer")(
                        cinfo as j_common_ptr, 1i32
                    );
                }
                24 => {
                    (*(*cinfo).err).msg_code = JTRC_BMP_OS2 as c_int;
                    (*(*cinfo).err).msg_parm.i[0usize] = biWidth;
                    (*(*cinfo).err).msg_parm.i[1usize] = biHeight;
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer")(
                        cinfo as j_common_ptr, 1i32
                    );
                }
                _ => {
                    (*(*cinfo).err).msg_code = JERR_BMP_BADDEPTH as c_int;
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer")(
                        cinfo as j_common_ptr
                    );
                }
            }
        }
        40 | 64 => {
            biWidth = (bmpinfoheader[4usize] as c_int as c_uint)
                .wrapping_add((bmpinfoheader[(4i32 + 1i32) as usize] as c_int as c_uint) << 8i32)
                .wrapping_add((bmpinfoheader[(4i32 + 2i32) as usize] as c_int as c_uint) << 16i32)
                .wrapping_add((bmpinfoheader[(4i32 + 3i32) as usize] as c_int as c_uint) << 24i32)
                as c_int;
            biHeight = (bmpinfoheader[8usize] as c_int as c_uint)
                .wrapping_add((bmpinfoheader[(8i32 + 1i32) as usize] as c_int as c_uint) << 8i32)
                .wrapping_add((bmpinfoheader[(8i32 + 2i32) as usize] as c_int as c_uint) << 16i32)
                .wrapping_add((bmpinfoheader[(8i32 + 3i32) as usize] as c_int as c_uint) << 24i32)
                as c_int;
            biPlanes = (bmpinfoheader[12usize] as c_int as c_ushort as c_int
                + ((bmpinfoheader[(12i32 + 1i32) as usize] as c_int as c_ushort as c_int) << 8i32))
                as c_ushort;
            (*source).bits_per_pixel = bmpinfoheader[14usize] as c_int as c_ushort as c_int
                + ((bmpinfoheader[(14i32 + 1i32) as usize] as c_int as c_ushort as c_int) << 8i32);
            biCompression = (bmpinfoheader[16usize] as c_int as c_uint)
                .wrapping_add((bmpinfoheader[(16i32 + 1i32) as usize] as c_int as c_uint) << 8i32)
                .wrapping_add((bmpinfoheader[(16i32 + 2i32) as usize] as c_int as c_uint) << 16i32)
                .wrapping_add((bmpinfoheader[(16i32 + 3i32) as usize] as c_int as c_uint) << 24i32);
            biXPelsPerMeter = (bmpinfoheader[24usize] as c_int as c_uint)
                .wrapping_add((bmpinfoheader[(24i32 + 1i32) as usize] as c_int as c_uint) << 8i32)
                .wrapping_add((bmpinfoheader[(24i32 + 2i32) as usize] as c_int as c_uint) << 16i32)
                .wrapping_add((bmpinfoheader[(24i32 + 3i32) as usize] as c_int as c_uint) << 24i32)
                as c_int;
            biYPelsPerMeter = (bmpinfoheader[28usize] as c_int as c_uint)
                .wrapping_add((bmpinfoheader[(28i32 + 1i32) as usize] as c_int as c_uint) << 8i32)
                .wrapping_add((bmpinfoheader[(28i32 + 2i32) as usize] as c_int as c_uint) << 16i32)
                .wrapping_add((bmpinfoheader[(28i32 + 3i32) as usize] as c_int as c_uint) << 24i32)
                as c_int;
            biClrUsed = (bmpinfoheader[32usize] as c_int as c_uint)
                .wrapping_add((bmpinfoheader[(32i32 + 1i32) as usize] as c_int as c_uint) << 8i32)
                .wrapping_add((bmpinfoheader[(32i32 + 2i32) as usize] as c_int as c_uint) << 16i32)
                .wrapping_add((bmpinfoheader[(32i32 + 3i32) as usize] as c_int as c_uint) << 24i32);
            match (*source).bits_per_pixel {
                8 => {
                    mapentrysize = 4i32;
                    (*(*cinfo).err).msg_code = JTRC_BMP_MAPPED as c_int;
                    (*(*cinfo).err).msg_parm.i[0usize] = biWidth;
                    (*(*cinfo).err).msg_parm.i[1usize] = biHeight;
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer")(
                        cinfo as j_common_ptr, 1i32
                    );
                }
                24 => {
                    (*(*cinfo).err).msg_code = JTRC_BMP as c_int;
                    (*(*cinfo).err).msg_parm.i[0usize] = biWidth;
                    (*(*cinfo).err).msg_parm.i[1usize] = biHeight;
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer")(
                        cinfo as j_common_ptr, 1i32
                    );
                }
                32 => {
                    (*(*cinfo).err).msg_code = JTRC_BMP as c_int;
                    (*(*cinfo).err).msg_parm.i[0usize] = biWidth;
                    (*(*cinfo).err).msg_parm.i[1usize] = biHeight;
                    (*(*cinfo).err)
                        .emit_message
                        .expect("non-null function pointer")(
                        cinfo as j_common_ptr, 1i32
                    );
                }
                _ => {
                    (*(*cinfo).err).msg_code = JERR_BMP_BADDEPTH as c_int;
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer")(
                        cinfo as j_common_ptr
                    );
                }
            }
            if biCompression != 0i32 as c_uint {
                (*(*cinfo).err).msg_code = JERR_BMP_COMPRESSED as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if biXPelsPerMeter > 0i32 && biYPelsPerMeter > 0i32 {
                (*cinfo).X_density = (biXPelsPerMeter / 100i32) as UINT16;
                (*cinfo).Y_density = (biYPelsPerMeter / 100i32) as UINT16;
                (*cinfo).density_unit = 2i32 as UINT8
            }
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_BMP_BADHEADER as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            return;
        }
    }
    if biWidth <= 0i32
        || biHeight <= 0i32
        || biWidth as c_long > 0x7fffffffi64
        || biHeight as c_long > 0x7fffffffi64
    {
        (*(*cinfo).err).msg_code = JERR_BMP_EMPTY as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if biPlanes as c_int != 1i32 {
        (*(*cinfo).err).msg_code = JERR_BMP_BADPLANES as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    bPad = bfOffBits.wrapping_sub(headerSize.wrapping_add(14i32 as c_uint)) as c_int;
    if mapentrysize > 0i32 {
        if biClrUsed <= 0i32 as c_uint {
            biClrUsed = 256i32 as c_uint
        } else if biClrUsed > 256i32 as c_uint {
            (*(*cinfo).err).msg_code = JERR_BMP_BADCMAP as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        (*source).colormap = (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            biClrUsed,
            3i32 as JDIMENSION,
        );
        (*source).cmap_length = biClrUsed as c_int;
        read_colormap(source, biClrUsed as c_int, mapentrysize);
        bPad = (bPad as c_uint).wrapping_sub(biClrUsed.wrapping_mul(mapentrysize as c_uint))
            as c_int as c_int
    }
    if bPad < 0i32 {
        (*(*cinfo).err).msg_code = JERR_BMP_BADHEADER as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    loop {
        bPad -= 1;
        if !(bPad >= 0i32) {
            break;
        }
        read_byte(source);
    }
    match (*source).bits_per_pixel {
        8 => {
            if (*cinfo).in_color_space as c_uint == JCS_UNKNOWN as c_int as c_uint {
                (*cinfo).in_color_space = JCS_EXT_RGB
            }
            if (*cinfo).in_color_space as c_uint == JCS_RGB as c_int as c_uint
                || (*cinfo).in_color_space as c_uint >= JCS_EXT_RGB as c_int as c_uint
                    && (*cinfo).in_color_space as c_uint <= JCS_EXT_ARGB as c_int as c_uint
            {
                (*cinfo).input_components = rgb_pixelsize[(*cinfo).in_color_space as usize]
            } else if (*cinfo).in_color_space as c_uint == JCS_GRAYSCALE as c_int as c_uint {
                (*cinfo).input_components = 1i32
            } else if (*cinfo).in_color_space as c_uint == JCS_CMYK as c_int as c_uint {
                (*cinfo).input_components = 4i32
            } else {
                (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            row_width = biWidth as JDIMENSION
        }
        24 => {
            if (*cinfo).in_color_space as c_uint == JCS_UNKNOWN as c_int as c_uint {
                (*cinfo).in_color_space = JCS_EXT_BGR
            }
            if (*cinfo).in_color_space as c_uint == JCS_RGB as c_int as c_uint
                || (*cinfo).in_color_space as c_uint >= JCS_EXT_RGB as c_int as c_uint
                    && (*cinfo).in_color_space as c_uint <= JCS_EXT_ARGB as c_int as c_uint
            {
                (*cinfo).input_components = rgb_pixelsize[(*cinfo).in_color_space as usize]
            } else if (*cinfo).in_color_space as c_uint == JCS_CMYK as c_int as c_uint {
                (*cinfo).input_components = 4i32
            } else {
                (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            row_width = (biWidth * 3i32) as JDIMENSION
        }
        32 => {
            if (*cinfo).in_color_space as c_uint == JCS_UNKNOWN as c_int as c_uint {
                (*cinfo).in_color_space = JCS_EXT_BGRA
            }
            if (*cinfo).in_color_space as c_uint == JCS_RGB as c_int as c_uint
                || (*cinfo).in_color_space as c_uint >= JCS_EXT_RGB as c_int as c_uint
                    && (*cinfo).in_color_space as c_uint <= JCS_EXT_ARGB as c_int as c_uint
            {
                (*cinfo).input_components = rgb_pixelsize[(*cinfo).in_color_space as usize]
            } else if (*cinfo).in_color_space as c_uint == JCS_CMYK as c_int as c_uint {
                (*cinfo).input_components = 4i32
            } else {
                (*(*cinfo).err).msg_code = JERR_BAD_IN_COLORSPACE as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            row_width = (biWidth * 4i32) as JDIMENSION
        }
        _ => {
            (*(*cinfo).err).msg_code = JERR_BMP_BADDEPTH as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    }
    while row_width & 3i32 as c_uint != 0i32 as c_uint {
        row_width = row_width.wrapping_add(1)
    }
    (*source).row_width = row_width;
    if 0 != (*source).use_inversion_array {
        (*source).whole_image = (*(*cinfo).mem)
            .request_virt_sarray
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            FALSE,
            row_width,
            biHeight as JDIMENSION,
            1i32 as JDIMENSION,
        );
        (*source).pub_0.get_pixel_rows = Some(
            preload_image
                as unsafe extern "C" fn(_: j_compress_ptr, _: cjpeg_source_ptr) -> JDIMENSION,
        );
        if !(*cinfo).progress.is_null() {
            let mut progress: cd_progress_ptr = (*cinfo).progress as cd_progress_ptr;
            (*progress).total_extra_passes += 1
        }
    } else {
        (*source).iobuffer = (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            row_width as size_t,
        ) as *mut U_CHAR;
        match (*source).bits_per_pixel {
            8 => {
                (*source).pub_0.get_pixel_rows = Some(
                    get_8bit_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            }
            24 => {
                (*source).pub_0.get_pixel_rows = Some(
                    get_24bit_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            }
            32 => {
                (*source).pub_0.get_pixel_rows = Some(
                    get_32bit_row
                        as unsafe extern "C" fn(
                            _: j_compress_ptr,
                            _: cjpeg_source_ptr,
                        ) -> JDIMENSION,
                )
            }
            _ => {
                (*(*cinfo).err).msg_code = JERR_BMP_BADDEPTH as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
        }
    }
    if (biWidth as c_ulonglong).wrapping_mul((*cinfo).input_components as c_ulonglong)
        > 0xffffffffu64
    {
        (*(*cinfo).err).msg_code = JERR_WIDTH_OVERFLOW as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*source).pub_0.buffer = (*(*cinfo).mem)
        .alloc_sarray
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        (biWidth * (*cinfo).input_components) as JDIMENSION,
        1i32 as JDIMENSION,
    );
    (*source).pub_0.buffer_height = 1i32 as JDIMENSION;
    (*cinfo).data_precision = 8i32;
    (*cinfo).image_width = biWidth as JDIMENSION;
    (*cinfo).image_height = biHeight as JDIMENSION;
}
/*
 * Finish up at the end of the file.
 */
unsafe extern "C" fn finish_input_bmp(mut _cinfo: j_compress_ptr, mut _sinfo: cjpeg_source_ptr) {}
/* no work */
/*
 * The module selection routine for BMP format input.
 */
#[no_mangle]
pub unsafe extern "C" fn jinit_read_bmp(
    mut cinfo: j_compress_ptr,
    mut use_inversion_array: boolean,
) -> cjpeg_source_ptr {
    let mut source: bmp_source_ptr = 0 as *mut _bmp_source_struct;
    source = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<bmp_source_struct>() as c_ulong,
    ) as bmp_source_ptr;
    (*source).cinfo = cinfo;
    (*source).pub_0.start_input =
        Some(start_input_bmp as unsafe extern "C" fn(_: j_compress_ptr, _: cjpeg_source_ptr) -> ());
    (*source).pub_0.finish_input = Some(
        finish_input_bmp as unsafe extern "C" fn(_: j_compress_ptr, _: cjpeg_source_ptr) -> (),
    );
    (*source).use_inversion_array = use_inversion_array;
    return source as cjpeg_source_ptr;
}
