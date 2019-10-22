use libc;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::rgb_blue;
pub use crate::jmorecfg_h::rgb_green;
pub use crate::jmorecfg_h::rgb_red;
pub use crate::jmorecfg_h::EXT_BGRX_BLUE;
pub use crate::jmorecfg_h::EXT_BGRX_GREEN;
pub use crate::jmorecfg_h::EXT_BGRX_RED;
pub use crate::jmorecfg_h::EXT_BGR_BLUE;
pub use crate::jmorecfg_h::EXT_BGR_GREEN;
pub use crate::jmorecfg_h::EXT_BGR_RED;
pub use crate::jmorecfg_h::EXT_RGBX_BLUE;
pub use crate::jmorecfg_h::EXT_RGBX_GREEN;
pub use crate::jmorecfg_h::EXT_RGBX_RED;
pub use crate::jmorecfg_h::EXT_RGB_BLUE;
pub use crate::jmorecfg_h::EXT_RGB_GREEN;
pub use crate::jmorecfg_h::EXT_RGB_RED;
pub use crate::jmorecfg_h::EXT_XBGR_BLUE;
pub use crate::jmorecfg_h::EXT_XBGR_GREEN;
pub use crate::jmorecfg_h::EXT_XBGR_RED;
pub use crate::jmorecfg_h::EXT_XRGB_BLUE;
pub use crate::jmorecfg_h::EXT_XRGB_GREEN;
pub use crate::jmorecfg_h::EXT_XRGB_RED;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::INT16;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::MAXJSAMPLE;
pub use crate::jmorecfg_h::RGB_BLUE;
pub use crate::jmorecfg_h::RGB_GREEN;
pub use crate::jmorecfg_h::RGB_RED;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jzero_far;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::JLONG;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_color_deconverter;
pub use crate::jpeglib_h::jpeg_color_quantizer;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_d_coef_controller;
pub use crate::jpeglib_h::jpeg_d_main_controller;
pub use crate::jpeglib_h::jpeg_d_post_controller;
pub use crate::jpeglib_h::jpeg_decomp_master;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_entropy_decoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_input_controller;
pub use crate::jpeglib_h::jpeg_inverse_dct;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_reader;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jpeg_upsampler;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JCOEFPTR;
pub use crate::jpeglib_h::JCS_CMYK;
pub use crate::jpeglib_h::JCS_EXT_ABGR;
pub use crate::jpeglib_h::JCS_EXT_ARGB;
pub use crate::jpeglib_h::JCS_EXT_BGR;
pub use crate::jpeglib_h::JCS_EXT_BGRA;
pub use crate::jpeglib_h::JCS_EXT_BGRX;
pub use crate::jpeglib_h::JCS_EXT_RGB;
pub use crate::jpeglib_h::JCS_EXT_RGBA;
pub use crate::jpeglib_h::JCS_EXT_RGBX;
pub use crate::jpeglib_h::JCS_EXT_XBGR;
pub use crate::jpeglib_h::JCS_EXT_XRGB;
pub use crate::jpeglib_h::JCS_GRAYSCALE;
pub use crate::jpeglib_h::JCS_RGB;
pub use crate::jpeglib_h::JCS_RGB565;
pub use crate::jpeglib_h::JCS_UNKNOWN;
pub use crate::jpeglib_h::JCS_YCCK;
pub use crate::jpeglib_h::JDCT_FLOAT;
pub use crate::jpeglib_h::JDCT_IFAST;
pub use crate::jpeglib_h::JDCT_ISLOW;
pub use crate::jpeglib_h::JDITHER_FS;
pub use crate::jpeglib_h::JDITHER_NONE;
pub use crate::jpeglib_h::JDITHER_ORDERED;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::src::jerror::C2RustUnnamed_3;
pub use crate::src::jerror::JERR_ARITH_NOTIMPL;
pub use crate::src::jerror::JERR_BAD_ALIGN_TYPE;
pub use crate::src::jerror::JERR_BAD_ALLOC_CHUNK;
pub use crate::src::jerror::JERR_BAD_BUFFER_MODE;
pub use crate::src::jerror::JERR_BAD_COMPONENT_ID;
pub use crate::src::jerror::JERR_BAD_CROP_SPEC;
pub use crate::src::jerror::JERR_BAD_DCTSIZE;
pub use crate::src::jerror::JERR_BAD_DCT_COEF;
pub use crate::src::jerror::JERR_BAD_HUFF_TABLE;
pub use crate::src::jerror::JERR_BAD_IN_COLORSPACE;
pub use crate::src::jerror::JERR_BAD_J_COLORSPACE;
pub use crate::src::jerror::JERR_BAD_LENGTH;
pub use crate::src::jerror::JERR_BAD_LIB_VERSION;
pub use crate::src::jerror::JERR_BAD_MCU_SIZE;
pub use crate::src::jerror::JERR_BAD_PARAM;
pub use crate::src::jerror::JERR_BAD_PARAM_VALUE;
pub use crate::src::jerror::JERR_BAD_POOL_ID;
pub use crate::src::jerror::JERR_BAD_PRECISION;
pub use crate::src::jerror::JERR_BAD_PROGRESSION;
pub use crate::src::jerror::JERR_BAD_PROG_SCRIPT;
pub use crate::src::jerror::JERR_BAD_SAMPLING;
pub use crate::src::jerror::JERR_BAD_SCAN_SCRIPT;
pub use crate::src::jerror::JERR_BAD_STATE;
pub use crate::src::jerror::JERR_BAD_STRUCT_SIZE;
pub use crate::src::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use crate::src::jerror::JERR_BUFFER_SIZE;
pub use crate::src::jerror::JERR_CANT_SUSPEND;
pub use crate::src::jerror::JERR_CCIR601_NOTIMPL;
pub use crate::src::jerror::JERR_COMPONENT_COUNT;
pub use crate::src::jerror::JERR_CONVERSION_NOTIMPL;
pub use crate::src::jerror::JERR_DAC_INDEX;
pub use crate::src::jerror::JERR_DAC_VALUE;
pub use crate::src::jerror::JERR_DHT_INDEX;
pub use crate::src::jerror::JERR_DQT_INDEX;
pub use crate::src::jerror::JERR_EMPTY_IMAGE;
pub use crate::src::jerror::JERR_EMS_READ;
pub use crate::src::jerror::JERR_EMS_WRITE;
pub use crate::src::jerror::JERR_EOI_EXPECTED;
pub use crate::src::jerror::JERR_FILE_READ;
pub use crate::src::jerror::JERR_FILE_WRITE;
pub use crate::src::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use crate::src::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use crate::src::jerror::JERR_HUFF_MISSING_CODE;
pub use crate::src::jerror::JERR_IMAGE_TOO_BIG;
pub use crate::src::jerror::JERR_INPUT_EMPTY;
pub use crate::src::jerror::JERR_INPUT_EOF;
pub use crate::src::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use crate::src::jerror::JERR_MISSING_DATA;
pub use crate::src::jerror::JERR_MODE_CHANGE;
pub use crate::src::jerror::JERR_NOTIMPL;
pub use crate::src::jerror::JERR_NOT_COMPILED;
pub use crate::src::jerror::JERR_NO_BACKING_STORE;
pub use crate::src::jerror::JERR_NO_HUFF_TABLE;
pub use crate::src::jerror::JERR_NO_IMAGE;
pub use crate::src::jerror::JERR_NO_QUANT_TABLE;
pub use crate::src::jerror::JERR_NO_SOI;
pub use crate::src::jerror::JERR_OUT_OF_MEMORY;
pub use crate::src::jerror::JERR_QUANT_COMPONENTS;
pub use crate::src::jerror::JERR_QUANT_FEW_COLORS;
pub use crate::src::jerror::JERR_QUANT_MANY_COLORS;
pub use crate::src::jerror::JERR_SOF_DUPLICATE;
pub use crate::src::jerror::JERR_SOF_NO_SOS;
pub use crate::src::jerror::JERR_SOF_UNSUPPORTED;
pub use crate::src::jerror::JERR_SOI_DUPLICATE;
pub use crate::src::jerror::JERR_SOS_NO_SOF;
pub use crate::src::jerror::JERR_TFILE_CREATE;
pub use crate::src::jerror::JERR_TFILE_READ;
pub use crate::src::jerror::JERR_TFILE_SEEK;
pub use crate::src::jerror::JERR_TFILE_WRITE;
pub use crate::src::jerror::JERR_TOO_LITTLE_DATA;
pub use crate::src::jerror::JERR_UNKNOWN_MARKER;
pub use crate::src::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use crate::src::jerror::JERR_VIRTUAL_BUG;
pub use crate::src::jerror::JERR_WIDTH_OVERFLOW;
pub use crate::src::jerror::JERR_XMS_READ;
pub use crate::src::jerror::JERR_XMS_WRITE;
pub use crate::src::jerror::JMSG_COPYRIGHT;
pub use crate::src::jerror::JMSG_LASTMSGCODE;
pub use crate::src::jerror::JMSG_NOMESSAGE;
pub use crate::src::jerror::JMSG_VERSION;
pub use crate::src::jerror::JTRC_16BIT_TABLES;
pub use crate::src::jerror::JTRC_ADOBE;
pub use crate::src::jerror::JTRC_APP0;
pub use crate::src::jerror::JTRC_APP14;
pub use crate::src::jerror::JTRC_DAC;
pub use crate::src::jerror::JTRC_DHT;
pub use crate::src::jerror::JTRC_DQT;
pub use crate::src::jerror::JTRC_DRI;
pub use crate::src::jerror::JTRC_EMS_CLOSE;
pub use crate::src::jerror::JTRC_EMS_OPEN;
pub use crate::src::jerror::JTRC_EOI;
pub use crate::src::jerror::JTRC_HUFFBITS;
pub use crate::src::jerror::JTRC_JFIF;
pub use crate::src::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use crate::src::jerror::JTRC_JFIF_EXTENSION;
pub use crate::src::jerror::JTRC_JFIF_THUMBNAIL;
pub use crate::src::jerror::JTRC_MISC_MARKER;
pub use crate::src::jerror::JTRC_PARMLESS_MARKER;
pub use crate::src::jerror::JTRC_QUANTVALS;
pub use crate::src::jerror::JTRC_QUANT_3_NCOLORS;
pub use crate::src::jerror::JTRC_QUANT_NCOLORS;
pub use crate::src::jerror::JTRC_QUANT_SELECTED;
pub use crate::src::jerror::JTRC_RECOVERY_ACTION;
pub use crate::src::jerror::JTRC_RST;
pub use crate::src::jerror::JTRC_SMOOTH_NOTIMPL;
pub use crate::src::jerror::JTRC_SOF;
pub use crate::src::jerror::JTRC_SOF_COMPONENT;
pub use crate::src::jerror::JTRC_SOI;
pub use crate::src::jerror::JTRC_SOS;
pub use crate::src::jerror::JTRC_SOS_COMPONENT;
pub use crate::src::jerror::JTRC_SOS_PARAMS;
pub use crate::src::jerror::JTRC_TFILE_CLOSE;
pub use crate::src::jerror::JTRC_TFILE_OPEN;
pub use crate::src::jerror::JTRC_THUMB_JPEG;
pub use crate::src::jerror::JTRC_THUMB_PALETTE;
pub use crate::src::jerror::JTRC_THUMB_RGB;
pub use crate::src::jerror::JTRC_UNKNOWN_IDS;
pub use crate::src::jerror::JTRC_XMS_CLOSE;
pub use crate::src::jerror::JTRC_XMS_OPEN;
pub use crate::src::jerror::JWRN_ADOBE_XFORM;
pub use crate::src::jerror::JWRN_BOGUS_ICC;
pub use crate::src::jerror::JWRN_BOGUS_PROGRESSION;
pub use crate::src::jerror::JWRN_EXTRANEOUS_DATA;
pub use crate::src::jerror::JWRN_HIT_MARKER;
pub use crate::src::jerror::JWRN_HUFF_BAD_CODE;
pub use crate::src::jerror::JWRN_JFIF_MAJOR;
pub use crate::src::jerror::JWRN_JPEG_EOF;
pub use crate::src::jerror::JWRN_MUST_RESYNC;
pub use crate::src::jerror::JWRN_NOT_SEQUENTIAL;
pub use crate::src::jerror::JWRN_TOO_MUCH_DATA;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
/* use 'int' for calculation temps */

pub type FSERRPTR = *mut FSERROR;
/* Declarations for Floyd-Steinberg dithering.
 *
 * Errors are accumulated into the array fserrors[], at a resolution of
 * 1/16th of a pixel count.  The error at a given pixel is propagated
 * to its not-yet-processed neighbors using the standard F-S fractions,
 *              ...     (here)  7/16
 *              3/16    5/16    1/16
 * We work left-to-right on even rows, right-to-left on odd rows.
 *
 * We can get away with a single array (holding one row's worth of errors)
 * by using it to store the current row's errors at pixel columns not yet
 * processed, but the next row's errors at columns already processed.  We
 * need only a few extra variables to hold the errors immediately around the
 * current column.  (If we are lucky, those variables are in registers, but
 * even if not, they're probably cheaper to access than array elements are.)
 *
 * The fserrors[] array is indexed [component#][position].
 * We provide (#columns + 2) entries per component; the extra entry at each
 * end saves us from special-casing the first and last pixels.
 */

pub type FSERROR = crate::jmorecfg_h::INT16;

pub type my_cquantize_ptr = *mut my_cquantizer;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_cquantizer {
    pub pub_0: crate::jpeglib_h::jpeg_color_quantizer,
    pub sv_colormap: crate::jpeglib_h::JSAMPARRAY,
    pub sv_actual: libc::c_int,
    pub colorindex: crate::jpeglib_h::JSAMPARRAY,
    pub is_padded: crate::jmorecfg_h::boolean,
    pub Ncolors: [libc::c_int; 4],
    pub row_index: libc::c_int,
    pub odither: [ODITHER_MATRIX_PTR; 4],
    pub fserrors: [FSERRPTR; 4],
    pub on_odd_row: crate::jmorecfg_h::boolean,
}

pub type ODITHER_MATRIX_PTR = *mut [libc::c_int; 16];
/* 16 bits should be enough */

pub type LOCFSERROR = libc::c_int;
/* mask for wrapping around
counters */

pub type ODITHER_MATRIX = [[libc::c_int; 16]; 16];
/*
 * jquant1.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2009, 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains 1-pass color quantization (color mapping) routines.
 * These routines provide mapping to a fixed color map using equally spaced
 * color values.  Optional Floyd-Steinberg or ordered dithering is available.
 */
/*
 * The main purpose of 1-pass quantization is to provide a fast, if not very
 * high quality, colormapped output capability.  A 2-pass quantizer usually
 * gives better visual quality; however, for quantized grayscale output this
 * quantizer is perfectly adequate.  Dithering is highly recommended with this
 * quantizer, though you can turn it off if you really want to.
 *
 * In 1-pass quantization the colormap must be chosen in advance of seeing the
 * image.  We use a map consisting of all combinations of Ncolors[i] color
 * values for the i'th component.  The Ncolors[] values are chosen so that
 * their product, the total number of colors, is no more than that requested.
 * (In most cases, the product will be somewhat less.)
 *
 * Since the colormap is orthogonal, the representative value for each color
 * component can be determined without considering the other components;
 * then these indexes can be combined into a colormap index by a standard
 * N-dimensional-array-subscript calculation.  Most of the arithmetic involved
 * can be precalculated and stored in the lookup table colorindex[].
 * colorindex[i][j] maps pixel value j in component i to the nearest
 * representative value (grid plane) for that component; this index is
 * multiplied by the array stride for component i, so that the
 * index of the colormap entry closest to a given pixel value is just
 *    sum( colorindex[component-number][pixel-component-value] )
 * Aside from being fast, this scheme allows for variable spacing between
 * representative values with no additional lookup cost.
 *
 * If gamma correction has been applied in color conversion, it might be wise
 * to adjust the color grid spacing so that the representative colors are
 * equidistant in linear space.  At this writing, gamma correction is not
 * implemented by jdcolor, so nothing is done here.
 */
/* Declarations for ordered dithering.
 *
 * We use a standard 16x16 ordered dither array.  The basic concept of ordered
 * dithering is described in many references, for instance Dale Schumacher's
 * chapter II.2 of Graphics Gems II (James Arvo, ed. Academic Press, 1991).
 * In place of Schumacher's comparisons against a "threshold" value, we add a
 * "dither" value to the input pixel and then round the result to the nearest
 * output value.  The dither value is equivalent to (0.5 - threshold) times
 * the distance between output values.  For ordered dithering, we assume that
 * the output colors are equally spaced; if not, results will probably be
 * worse, since the dither may be too much or too little at a given point.
 *
 * The normal calculation would be to form pixel value + dither, range-limit
 * this to 0..MAXJSAMPLE, and then index into the colorindex table as usual.
 * We can skip the separate range-limiting step by extending the colorindex
 * table in both directions.
 */

pub const ODITHER_SIZE: libc::c_int = 16i32;
/* dimension of dither matrix */
/* NB: if ODITHER_SIZE is not a power of 2, ODITHER_MASK uses will break */

pub const ODITHER_CELLS: libc::c_int = ODITHER_SIZE * ODITHER_SIZE;
/* # cells in matrix */

pub const ODITHER_MASK: libc::c_int = ODITHER_SIZE - 1i32;

static mut base_dither_matrix: [[crate::jmorecfg_h::UINT8; 16]; 16] = [
    [
        0i32 as crate::jmorecfg_h::UINT8,
        192i32 as crate::jmorecfg_h::UINT8,
        48i32 as crate::jmorecfg_h::UINT8,
        240i32 as crate::jmorecfg_h::UINT8,
        12i32 as crate::jmorecfg_h::UINT8,
        204i32 as crate::jmorecfg_h::UINT8,
        60i32 as crate::jmorecfg_h::UINT8,
        252i32 as crate::jmorecfg_h::UINT8,
        3i32 as crate::jmorecfg_h::UINT8,
        195i32 as crate::jmorecfg_h::UINT8,
        51i32 as crate::jmorecfg_h::UINT8,
        243i32 as crate::jmorecfg_h::UINT8,
        15i32 as crate::jmorecfg_h::UINT8,
        207i32 as crate::jmorecfg_h::UINT8,
        63i32 as crate::jmorecfg_h::UINT8,
        255i32 as crate::jmorecfg_h::UINT8,
    ],
    [
        128i32 as crate::jmorecfg_h::UINT8,
        64i32 as crate::jmorecfg_h::UINT8,
        176i32 as crate::jmorecfg_h::UINT8,
        112i32 as crate::jmorecfg_h::UINT8,
        140i32 as crate::jmorecfg_h::UINT8,
        76i32 as crate::jmorecfg_h::UINT8,
        188i32 as crate::jmorecfg_h::UINT8,
        124i32 as crate::jmorecfg_h::UINT8,
        131i32 as crate::jmorecfg_h::UINT8,
        67i32 as crate::jmorecfg_h::UINT8,
        179i32 as crate::jmorecfg_h::UINT8,
        115i32 as crate::jmorecfg_h::UINT8,
        143i32 as crate::jmorecfg_h::UINT8,
        79i32 as crate::jmorecfg_h::UINT8,
        191i32 as crate::jmorecfg_h::UINT8,
        127i32 as crate::jmorecfg_h::UINT8,
    ],
    [
        32i32 as crate::jmorecfg_h::UINT8,
        224i32 as crate::jmorecfg_h::UINT8,
        16i32 as crate::jmorecfg_h::UINT8,
        208i32 as crate::jmorecfg_h::UINT8,
        44i32 as crate::jmorecfg_h::UINT8,
        236i32 as crate::jmorecfg_h::UINT8,
        28i32 as crate::jmorecfg_h::UINT8,
        220i32 as crate::jmorecfg_h::UINT8,
        35i32 as crate::jmorecfg_h::UINT8,
        227i32 as crate::jmorecfg_h::UINT8,
        19i32 as crate::jmorecfg_h::UINT8,
        211i32 as crate::jmorecfg_h::UINT8,
        47i32 as crate::jmorecfg_h::UINT8,
        239i32 as crate::jmorecfg_h::UINT8,
        31i32 as crate::jmorecfg_h::UINT8,
        223i32 as crate::jmorecfg_h::UINT8,
    ],
    [
        160i32 as crate::jmorecfg_h::UINT8,
        96i32 as crate::jmorecfg_h::UINT8,
        144i32 as crate::jmorecfg_h::UINT8,
        80i32 as crate::jmorecfg_h::UINT8,
        172i32 as crate::jmorecfg_h::UINT8,
        108i32 as crate::jmorecfg_h::UINT8,
        156i32 as crate::jmorecfg_h::UINT8,
        92i32 as crate::jmorecfg_h::UINT8,
        163i32 as crate::jmorecfg_h::UINT8,
        99i32 as crate::jmorecfg_h::UINT8,
        147i32 as crate::jmorecfg_h::UINT8,
        83i32 as crate::jmorecfg_h::UINT8,
        175i32 as crate::jmorecfg_h::UINT8,
        111i32 as crate::jmorecfg_h::UINT8,
        159i32 as crate::jmorecfg_h::UINT8,
        95i32 as crate::jmorecfg_h::UINT8,
    ],
    [
        8i32 as crate::jmorecfg_h::UINT8,
        200i32 as crate::jmorecfg_h::UINT8,
        56i32 as crate::jmorecfg_h::UINT8,
        248i32 as crate::jmorecfg_h::UINT8,
        4i32 as crate::jmorecfg_h::UINT8,
        196i32 as crate::jmorecfg_h::UINT8,
        52i32 as crate::jmorecfg_h::UINT8,
        244i32 as crate::jmorecfg_h::UINT8,
        11i32 as crate::jmorecfg_h::UINT8,
        203i32 as crate::jmorecfg_h::UINT8,
        59i32 as crate::jmorecfg_h::UINT8,
        251i32 as crate::jmorecfg_h::UINT8,
        7i32 as crate::jmorecfg_h::UINT8,
        199i32 as crate::jmorecfg_h::UINT8,
        55i32 as crate::jmorecfg_h::UINT8,
        247i32 as crate::jmorecfg_h::UINT8,
    ],
    [
        136i32 as crate::jmorecfg_h::UINT8,
        72i32 as crate::jmorecfg_h::UINT8,
        184i32 as crate::jmorecfg_h::UINT8,
        120i32 as crate::jmorecfg_h::UINT8,
        132i32 as crate::jmorecfg_h::UINT8,
        68i32 as crate::jmorecfg_h::UINT8,
        180i32 as crate::jmorecfg_h::UINT8,
        116i32 as crate::jmorecfg_h::UINT8,
        139i32 as crate::jmorecfg_h::UINT8,
        75i32 as crate::jmorecfg_h::UINT8,
        187i32 as crate::jmorecfg_h::UINT8,
        123i32 as crate::jmorecfg_h::UINT8,
        135i32 as crate::jmorecfg_h::UINT8,
        71i32 as crate::jmorecfg_h::UINT8,
        183i32 as crate::jmorecfg_h::UINT8,
        119i32 as crate::jmorecfg_h::UINT8,
    ],
    [
        40i32 as crate::jmorecfg_h::UINT8,
        232i32 as crate::jmorecfg_h::UINT8,
        24i32 as crate::jmorecfg_h::UINT8,
        216i32 as crate::jmorecfg_h::UINT8,
        36i32 as crate::jmorecfg_h::UINT8,
        228i32 as crate::jmorecfg_h::UINT8,
        20i32 as crate::jmorecfg_h::UINT8,
        212i32 as crate::jmorecfg_h::UINT8,
        43i32 as crate::jmorecfg_h::UINT8,
        235i32 as crate::jmorecfg_h::UINT8,
        27i32 as crate::jmorecfg_h::UINT8,
        219i32 as crate::jmorecfg_h::UINT8,
        39i32 as crate::jmorecfg_h::UINT8,
        231i32 as crate::jmorecfg_h::UINT8,
        23i32 as crate::jmorecfg_h::UINT8,
        215i32 as crate::jmorecfg_h::UINT8,
    ],
    [
        168i32 as crate::jmorecfg_h::UINT8,
        104i32 as crate::jmorecfg_h::UINT8,
        152i32 as crate::jmorecfg_h::UINT8,
        88i32 as crate::jmorecfg_h::UINT8,
        164i32 as crate::jmorecfg_h::UINT8,
        100i32 as crate::jmorecfg_h::UINT8,
        148i32 as crate::jmorecfg_h::UINT8,
        84i32 as crate::jmorecfg_h::UINT8,
        171i32 as crate::jmorecfg_h::UINT8,
        107i32 as crate::jmorecfg_h::UINT8,
        155i32 as crate::jmorecfg_h::UINT8,
        91i32 as crate::jmorecfg_h::UINT8,
        167i32 as crate::jmorecfg_h::UINT8,
        103i32 as crate::jmorecfg_h::UINT8,
        151i32 as crate::jmorecfg_h::UINT8,
        87i32 as crate::jmorecfg_h::UINT8,
    ],
    [
        2i32 as crate::jmorecfg_h::UINT8,
        194i32 as crate::jmorecfg_h::UINT8,
        50i32 as crate::jmorecfg_h::UINT8,
        242i32 as crate::jmorecfg_h::UINT8,
        14i32 as crate::jmorecfg_h::UINT8,
        206i32 as crate::jmorecfg_h::UINT8,
        62i32 as crate::jmorecfg_h::UINT8,
        254i32 as crate::jmorecfg_h::UINT8,
        1i32 as crate::jmorecfg_h::UINT8,
        193i32 as crate::jmorecfg_h::UINT8,
        49i32 as crate::jmorecfg_h::UINT8,
        241i32 as crate::jmorecfg_h::UINT8,
        13i32 as crate::jmorecfg_h::UINT8,
        205i32 as crate::jmorecfg_h::UINT8,
        61i32 as crate::jmorecfg_h::UINT8,
        253i32 as crate::jmorecfg_h::UINT8,
    ],
    [
        130i32 as crate::jmorecfg_h::UINT8,
        66i32 as crate::jmorecfg_h::UINT8,
        178i32 as crate::jmorecfg_h::UINT8,
        114i32 as crate::jmorecfg_h::UINT8,
        142i32 as crate::jmorecfg_h::UINT8,
        78i32 as crate::jmorecfg_h::UINT8,
        190i32 as crate::jmorecfg_h::UINT8,
        126i32 as crate::jmorecfg_h::UINT8,
        129i32 as crate::jmorecfg_h::UINT8,
        65i32 as crate::jmorecfg_h::UINT8,
        177i32 as crate::jmorecfg_h::UINT8,
        113i32 as crate::jmorecfg_h::UINT8,
        141i32 as crate::jmorecfg_h::UINT8,
        77i32 as crate::jmorecfg_h::UINT8,
        189i32 as crate::jmorecfg_h::UINT8,
        125i32 as crate::jmorecfg_h::UINT8,
    ],
    [
        34i32 as crate::jmorecfg_h::UINT8,
        226i32 as crate::jmorecfg_h::UINT8,
        18i32 as crate::jmorecfg_h::UINT8,
        210i32 as crate::jmorecfg_h::UINT8,
        46i32 as crate::jmorecfg_h::UINT8,
        238i32 as crate::jmorecfg_h::UINT8,
        30i32 as crate::jmorecfg_h::UINT8,
        222i32 as crate::jmorecfg_h::UINT8,
        33i32 as crate::jmorecfg_h::UINT8,
        225i32 as crate::jmorecfg_h::UINT8,
        17i32 as crate::jmorecfg_h::UINT8,
        209i32 as crate::jmorecfg_h::UINT8,
        45i32 as crate::jmorecfg_h::UINT8,
        237i32 as crate::jmorecfg_h::UINT8,
        29i32 as crate::jmorecfg_h::UINT8,
        221i32 as crate::jmorecfg_h::UINT8,
    ],
    [
        162i32 as crate::jmorecfg_h::UINT8,
        98i32 as crate::jmorecfg_h::UINT8,
        146i32 as crate::jmorecfg_h::UINT8,
        82i32 as crate::jmorecfg_h::UINT8,
        174i32 as crate::jmorecfg_h::UINT8,
        110i32 as crate::jmorecfg_h::UINT8,
        158i32 as crate::jmorecfg_h::UINT8,
        94i32 as crate::jmorecfg_h::UINT8,
        161i32 as crate::jmorecfg_h::UINT8,
        97i32 as crate::jmorecfg_h::UINT8,
        145i32 as crate::jmorecfg_h::UINT8,
        81i32 as crate::jmorecfg_h::UINT8,
        173i32 as crate::jmorecfg_h::UINT8,
        109i32 as crate::jmorecfg_h::UINT8,
        157i32 as crate::jmorecfg_h::UINT8,
        93i32 as crate::jmorecfg_h::UINT8,
    ],
    [
        10i32 as crate::jmorecfg_h::UINT8,
        202i32 as crate::jmorecfg_h::UINT8,
        58i32 as crate::jmorecfg_h::UINT8,
        250i32 as crate::jmorecfg_h::UINT8,
        6i32 as crate::jmorecfg_h::UINT8,
        198i32 as crate::jmorecfg_h::UINT8,
        54i32 as crate::jmorecfg_h::UINT8,
        246i32 as crate::jmorecfg_h::UINT8,
        9i32 as crate::jmorecfg_h::UINT8,
        201i32 as crate::jmorecfg_h::UINT8,
        57i32 as crate::jmorecfg_h::UINT8,
        249i32 as crate::jmorecfg_h::UINT8,
        5i32 as crate::jmorecfg_h::UINT8,
        197i32 as crate::jmorecfg_h::UINT8,
        53i32 as crate::jmorecfg_h::UINT8,
        245i32 as crate::jmorecfg_h::UINT8,
    ],
    [
        138i32 as crate::jmorecfg_h::UINT8,
        74i32 as crate::jmorecfg_h::UINT8,
        186i32 as crate::jmorecfg_h::UINT8,
        122i32 as crate::jmorecfg_h::UINT8,
        134i32 as crate::jmorecfg_h::UINT8,
        70i32 as crate::jmorecfg_h::UINT8,
        182i32 as crate::jmorecfg_h::UINT8,
        118i32 as crate::jmorecfg_h::UINT8,
        137i32 as crate::jmorecfg_h::UINT8,
        73i32 as crate::jmorecfg_h::UINT8,
        185i32 as crate::jmorecfg_h::UINT8,
        121i32 as crate::jmorecfg_h::UINT8,
        133i32 as crate::jmorecfg_h::UINT8,
        69i32 as crate::jmorecfg_h::UINT8,
        181i32 as crate::jmorecfg_h::UINT8,
        117i32 as crate::jmorecfg_h::UINT8,
    ],
    [
        42i32 as crate::jmorecfg_h::UINT8,
        234i32 as crate::jmorecfg_h::UINT8,
        26i32 as crate::jmorecfg_h::UINT8,
        218i32 as crate::jmorecfg_h::UINT8,
        38i32 as crate::jmorecfg_h::UINT8,
        230i32 as crate::jmorecfg_h::UINT8,
        22i32 as crate::jmorecfg_h::UINT8,
        214i32 as crate::jmorecfg_h::UINT8,
        41i32 as crate::jmorecfg_h::UINT8,
        233i32 as crate::jmorecfg_h::UINT8,
        25i32 as crate::jmorecfg_h::UINT8,
        217i32 as crate::jmorecfg_h::UINT8,
        37i32 as crate::jmorecfg_h::UINT8,
        229i32 as crate::jmorecfg_h::UINT8,
        21i32 as crate::jmorecfg_h::UINT8,
        213i32 as crate::jmorecfg_h::UINT8,
    ],
    [
        170i32 as crate::jmorecfg_h::UINT8,
        106i32 as crate::jmorecfg_h::UINT8,
        154i32 as crate::jmorecfg_h::UINT8,
        90i32 as crate::jmorecfg_h::UINT8,
        166i32 as crate::jmorecfg_h::UINT8,
        102i32 as crate::jmorecfg_h::UINT8,
        150i32 as crate::jmorecfg_h::UINT8,
        86i32 as crate::jmorecfg_h::UINT8,
        169i32 as crate::jmorecfg_h::UINT8,
        105i32 as crate::jmorecfg_h::UINT8,
        153i32 as crate::jmorecfg_h::UINT8,
        89i32 as crate::jmorecfg_h::UINT8,
        165i32 as crate::jmorecfg_h::UINT8,
        101i32 as crate::jmorecfg_h::UINT8,
        149i32 as crate::jmorecfg_h::UINT8,
        85i32 as crate::jmorecfg_h::UINT8,
    ],
];
/* pointer to error array */
/* Private subobject */

pub const MAX_Q_COMPS: libc::c_int = 4i32;
/*
 * Policy-making subroutines for create_colormap and create_colorindex.
 * These routines determine the colormap to be used.  The rest of the module
 * only assumes that the colormap is orthogonal.
 *
 *  * select_ncolors decides how to divvy up the available colors
 *    among the components.
 *  * output_value defines the set of representative values for a component.
 *  * largest_input_value defines the mapping from input values to
 *    representative values for a component.
 * Note that the latter two routines may impose different policies for
 * different components, though this is not currently done.
 */

unsafe extern "C" fn select_ncolors(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut Ncolors: *mut libc::c_int,
) -> libc::c_int
/* Determine allocation of desired colors to components, */
/* and fill in Ncolors[] array to indicate choice. */
/* Return value is total number of colors (product of Ncolors[] values). */ {
    let mut nc: libc::c_int = (*cinfo).out_color_components; /* number of color components */
    let mut max_colors: libc::c_int = (*cinfo).desired_number_of_colors;
    let mut total_colors: libc::c_int = 0;
    let mut iroot: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut changed: crate::jmorecfg_h::boolean = 0;
    let mut temp: libc::c_long = 0;
    let mut RGB_order: [libc::c_int; 3] = [
        crate::jmorecfg_h::RGB_GREEN,
        crate::jmorecfg_h::RGB_RED,
        crate::jmorecfg_h::RGB_BLUE,
    ];
    RGB_order[0] = crate::jmorecfg_h::rgb_green[(*cinfo).out_color_space as usize];
    RGB_order[1] = crate::jmorecfg_h::rgb_red[(*cinfo).out_color_space as usize];
    RGB_order[2] = crate::jmorecfg_h::rgb_blue[(*cinfo).out_color_space as usize];
    /* We can allocate at least the nc'th root of max_colors per component. */
    /* Compute floor(nc'th root of max_colors). */
    iroot = 1i32; /* repeat till iroot exceeds root */
    loop {
        iroot += 1; /* set temp = iroot ** nc */
        temp = iroot as libc::c_long; /* now iroot = floor(root) */
        i = 1i32;
        while i < nc {
            temp *= iroot as libc::c_long;
            i += 1
        }
        if !(temp <= max_colors as libc::c_long) {
            break;
        }
    }
    iroot -= 1;
    /* Must have at least 2 color values per component */
    if iroot < 2i32 {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_QUANT_FEW_COLORS as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = temp as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Initialize to iroot color values for each component */
    total_colors = 1i32;
    i = 0i32;
    while i < nc {
        *Ncolors.offset(i as isize) = iroot;
        total_colors *= iroot;
        i += 1
    }
    loop
    /* We may be able to increment the count for one or more components without
     * exceeding max_colors, though we know not all can be incremented.
     * Sometimes, the first component can be incremented more than once!
     * (Example: for 16 colors, we start at 2*2*2, go to 3*2*2, then 4*2*2.)
     * In RGB colorspace, try to increment G first, then R, then B.
     */
    {
        changed = crate::jmorecfg_h::FALSE;
        i = 0i32;
        while i < nc {
            j = if (*cinfo).out_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_RGB as libc::c_int as libc::c_uint
            {
                RGB_order[i as usize]
            } else {
                i
            };
            /* calculate new total_colors if Ncolors[j] is incremented */
            temp = (total_colors / *Ncolors.offset(j as isize)) as libc::c_long; /* done in long arith to avoid oflo */
            temp *= (*Ncolors.offset(j as isize) + 1i32) as libc::c_long; /* won't fit, done with this pass */
            if temp > max_colors as libc::c_long {
                break; /* OK, apply the increment */
            }
            let ref mut fresh0 = *Ncolors.offset(j as isize);
            *fresh0 += 1;
            total_colors = temp as libc::c_int;
            changed = crate::jmorecfg_h::TRUE;
            i += 1
        }
        if !(changed != 0) {
            break;
        }
    }
    return total_colors;
}

unsafe extern "C" fn output_value(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut ci: libc::c_int,
    mut j: libc::c_int,
    mut maxj: libc::c_int,
) -> libc::c_int
/* Return j'th output value, where j will range from 0 to maxj */
/* The output values must fall in 0..MAXJSAMPLE in increasing order */ {
    /* We always provide values 0 and MAXJSAMPLE for each component;
     * any additional values are equally spaced between these limits.
     * (Forcing the upper and lower values to the limits ensures that
     * dithering can't produce a color outside the selected gamut.)
     */
    return ((j as crate::jpegint_h::JLONG * crate::jmorecfg_h::MAXJSAMPLE as libc::c_long
        + (maxj / 2i32) as libc::c_long)
        / maxj as libc::c_long) as libc::c_int;
}

unsafe extern "C" fn largest_input_value(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut ci: libc::c_int,
    mut j: libc::c_int,
    mut maxj: libc::c_int,
) -> libc::c_int
/* Return largest input value that should map to j'th output value */
/* Must have largest(j=0) >= 0, and largest(j=maxj) >= MAXJSAMPLE */ {
    /* Breakpoints are halfway between values returned by output_value */
    return (((2i32 * j + 1i32) as crate::jpegint_h::JLONG
        * crate::jmorecfg_h::MAXJSAMPLE as libc::c_long
        + maxj as libc::c_long)
        / (2i32 * maxj) as libc::c_long) as libc::c_int;
}
/*
 * Create the colormap.
 */

unsafe extern "C" fn create_colormap(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* Created colormap */
    let mut colormap: crate::jpeglib_h::JSAMPARRAY = 0 as *mut crate::jpeglib_h::JSAMPROW; /* Number of distinct output colors */
    let mut total_colors: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nci: libc::c_int = 0;
    let mut blksize: libc::c_int = 0;
    let mut blkdist: libc::c_int = 0;
    let mut ptr: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    /* Select number of colors for each component */
    total_colors = select_ncolors(cinfo, (*cquantize).Ncolors.as_mut_ptr());
    /* Report selected color counts */
    if (*cinfo).out_color_components == 3i32 {
        let mut _mp: *mut libc::c_int = (*(*cinfo).err).msg_parm.i.as_mut_ptr();
        *_mp.offset(0) = total_colors;
        *_mp.offset(1) = (*cquantize).Ncolors[0];
        *_mp.offset(2) = (*cquantize).Ncolors[1];
        *_mp.offset(3) = (*cquantize).Ncolors[2];
        (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_QUANT_3_NCOLORS as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 1i32);
    } else {
        (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_QUANT_NCOLORS as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = total_colors;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 1i32);
    }
    /* Allocate and fill in the colormap. */
    /* The colors are ordered in the map in standard row-major order, */
    /* i.e. rightmost (highest-indexed) color changes most rapidly. */
    colormap = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        total_colors as crate::jmorecfg_h::JDIMENSION,
        (*cinfo).out_color_components as crate::jmorecfg_h::JDIMENSION,
    );
    /* blksize is number of adjacent repeated entries for a component */
    /* blkdist is distance between groups of identical entries for a component */
    blkdist = total_colors;
    i = 0i32;
    while i < (*cinfo).out_color_components {
        /* fill in colormap entries for i'th color component */
        nci = (*cquantize).Ncolors[i as usize];
        blksize = blkdist / nci;
        j = 0i32;
        while j < nci {
            /* # of distinct values for this color */
            /* blksize of this color is blkdist of next */
            /* Compute j'th output value (out of nci) for component */
            val = output_value(cinfo, i, j, nci - 1i32);
            /* Fill in all colormap entries that have this value of this component */
            ptr = j * blksize;
            while ptr < total_colors {
                /* fill in blksize entries beginning at ptr */
                k = 0i32;
                while k < blksize {
                    *(*colormap.offset(i as isize)).offset((ptr + k) as isize) =
                        val as crate::jmorecfg_h::JSAMPLE;
                    k += 1
                }
                ptr += blkdist
            }
            j += 1
        }
        blkdist = blksize;
        i += 1
    }
    /* Save the colormap in private storage,
     * where it will survive color quantization mode changes.
     */
    (*cquantize).sv_colormap = colormap;
    (*cquantize).sv_actual = total_colors;
}
/*
 * Create the color index table.
 */

unsafe extern "C" fn create_colorindex(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut indexptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nci: libc::c_int = 0;
    let mut blksize: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut pad: libc::c_int = 0;
    /* For ordered dither, we pad the color index tables by MAXJSAMPLE in
     * each direction (input index values can be -MAXJSAMPLE .. 2*MAXJSAMPLE).
     * This is not necessary in the other dithering modes.  However, we
     * flag whether it was done in case user changes dithering mode.
     */
    if (*cinfo).dither_mode as libc::c_uint
        == crate::jpeglib_h::JDITHER_ORDERED as libc::c_int as libc::c_uint
    {
        pad = crate::jmorecfg_h::MAXJSAMPLE * 2i32;
        (*cquantize).is_padded = crate::jmorecfg_h::TRUE
    } else {
        pad = 0i32;
        (*cquantize).is_padded = crate::jmorecfg_h::FALSE
    }
    (*cquantize).colorindex = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        (crate::jmorecfg_h::MAXJSAMPLE + 1i32 + pad) as crate::jmorecfg_h::JDIMENSION,
        (*cinfo).out_color_components as crate::jmorecfg_h::JDIMENSION,
    );
    /* blksize is number of adjacent repeated entries for a component */
    blksize = (*cquantize).sv_actual;
    i = 0i32;
    while i < (*cinfo).out_color_components {
        /* fill in colorindex entries for i'th color component */
        nci = (*cquantize).Ncolors[i as usize]; /* # of distinct values for this color */
        blksize = blksize / nci;
        /* adjust colorindex pointers to provide padding at negative indexes. */
        if pad != 0 {
            let ref mut fresh1 = *(*cquantize).colorindex.offset(i as isize);
            *fresh1 = (*fresh1).offset(crate::jmorecfg_h::MAXJSAMPLE as isize)
        }
        /* in loop, val = index of current output value, */
        /* and k = largest j that maps to current val */
        indexptr = *(*cquantize).colorindex.offset(i as isize);
        val = 0i32;
        k = largest_input_value(cinfo, i, 0i32, nci - 1i32);
        j = 0i32;
        while j <= crate::jmorecfg_h::MAXJSAMPLE {
            while j > k {
                /* advance val if past boundary */
                val += 1;
                k = largest_input_value(cinfo, i, val, nci - 1i32)
            }
            /* premultiply so that no multiplication needed in main processing */
            *indexptr.offset(j as isize) = (val * blksize) as crate::jmorecfg_h::JSAMPLE;
            j += 1
        }
        /* Pad at both ends if necessary */
        if pad != 0 {
            j = 1i32;
            while j <= crate::jmorecfg_h::MAXJSAMPLE {
                *indexptr.offset(-j as isize) = *indexptr.offset(0);
                *indexptr.offset((crate::jmorecfg_h::MAXJSAMPLE + j) as isize) =
                    *indexptr.offset(crate::jmorecfg_h::MAXJSAMPLE as isize);
                j += 1
            }
        }
        i += 1
    }
}
/*
 * Create an ordered-dither array for a component having ncolors
 * distinct output values.
 */

unsafe extern "C" fn make_odither_array(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut ncolors: libc::c_int,
) -> ODITHER_MATRIX_PTR {
    let mut odither: ODITHER_MATRIX_PTR = 0 as *mut [libc::c_int; 16];
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut num: crate::jpegint_h::JLONG = 0;
    let mut den: crate::jpegint_h::JLONG = 0;
    odither = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<ODITHER_MATRIX>() as libc::c_ulong,
    ) as ODITHER_MATRIX_PTR;
    /* The inter-value distance for this color is MAXJSAMPLE/(ncolors-1).
     * Hence the dither value for the matrix cell with fill order f
     * (f=0..N-1) should be (N-1-2*f)/(2*N) * MAXJSAMPLE/(ncolors-1).
     * On 16-bit-int machine, be careful to avoid overflow.
     */
    den = (2i32 * ODITHER_CELLS) as libc::c_long * (ncolors - 1i32) as crate::jpegint_h::JLONG;
    j = 0i32;
    while j < ODITHER_SIZE {
        k = 0i32;
        while k < ODITHER_SIZE {
            num = (ODITHER_CELLS
                - 1i32
                - 2i32 * base_dither_matrix[j as usize][k as usize] as libc::c_int)
                as crate::jpegint_h::JLONG
                * crate::jmorecfg_h::MAXJSAMPLE as libc::c_long;
            /* Ensure round towards zero despite C's lack of consistency
             * about rounding negative values in integer division...
             */
            (*odither.offset(j as isize))[k as usize] = if num < 0i32 as libc::c_long {
                -(-num / den)
            } else {
                (num) / den
            } as libc::c_int;
            k += 1
        }
        j += 1
    }
    return odither;
}
/*
 * Create the ordered-dither tables.
 * Components having the same number of representative colors may
 * share a dither table.
 */

unsafe extern "C" fn create_odither_tables(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* # of distinct values for this color */
    let mut odither: ODITHER_MATRIX_PTR = 0 as *mut [libc::c_int; 16]; /* search for matching prior component */
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut nci: libc::c_int = 0;
    i = 0i32;
    while i < (*cinfo).out_color_components {
        nci = (*cquantize).Ncolors[i as usize];
        odither = crate::stddef_h::NULL as ODITHER_MATRIX_PTR;
        j = 0i32;
        while j < i {
            if nci == (*cquantize).Ncolors[j as usize] {
                odither = (*cquantize).odither[j as usize];
                break;
            } else {
                j += 1
            }
        }
        if odither.is_null() {
            /* need a new table? */
            odither = make_odither_array(cinfo, nci)
        }
        (*cquantize).odither[i as usize] = odither;
        i += 1
    }
}
/*
 * Map some rows of pixels to the output colormapped representation.
 */

unsafe extern "C" fn color_quantize(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
)
/* General case, no dithering */
{
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut colorindex: crate::jpeglib_h::JSAMPARRAY = (*cquantize).colorindex;
    let mut pixcode: libc::c_int = 0;
    let mut ci: libc::c_int = 0;
    let mut ptrin: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ptrout: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut row: libc::c_int = 0;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut width: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    let mut nc: libc::c_int = (*cinfo).out_color_components;
    row = 0i32;
    while row < num_rows {
        ptrin = *input_buf.offset(row as isize);
        ptrout = *output_buf.offset(row as isize);
        col = width;
        while col > 0i32 as libc::c_uint {
            pixcode = 0i32;
            ci = 0i32;
            while ci < nc {
                let fresh2 = ptrin;
                ptrin = ptrin.offset(1);
                pixcode += *(*colorindex.offset(ci as isize))
                    .offset(*fresh2 as libc::c_int as isize)
                    as libc::c_int;
                ci += 1
            }
            let fresh3 = ptrout;
            ptrout = ptrout.offset(1);
            *fresh3 = pixcode as crate::jmorecfg_h::JSAMPLE;
            col = col.wrapping_sub(1)
        }
        row += 1
    }
}

unsafe extern "C" fn color_quantize3(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
)
/* Fast path for out_color_components==3, no dithering */
{
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut pixcode: libc::c_int = 0;
    let mut ptrin: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ptrout: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut colorindex0: crate::jpeglib_h::JSAMPROW = *(*cquantize).colorindex.offset(0);
    let mut colorindex1: crate::jpeglib_h::JSAMPROW = *(*cquantize).colorindex.offset(1);
    let mut colorindex2: crate::jpeglib_h::JSAMPROW = *(*cquantize).colorindex.offset(2);
    let mut row: libc::c_int = 0;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut width: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    row = 0i32;
    while row < num_rows {
        ptrin = *input_buf.offset(row as isize);
        ptrout = *output_buf.offset(row as isize);
        col = width;
        while col > 0i32 as libc::c_uint {
            let fresh4 = ptrin;
            ptrin = ptrin.offset(1);
            pixcode = *colorindex0.offset(*fresh4 as libc::c_int as isize) as libc::c_int;
            let fresh5 = ptrin;
            ptrin = ptrin.offset(1);
            pixcode += *colorindex1.offset(*fresh5 as libc::c_int as isize) as libc::c_int;
            let fresh6 = ptrin;
            ptrin = ptrin.offset(1);
            pixcode += *colorindex2.offset(*fresh6 as libc::c_int as isize) as libc::c_int;
            let fresh7 = ptrout;
            ptrout = ptrout.offset(1);
            *fresh7 = pixcode as crate::jmorecfg_h::JSAMPLE;
            col = col.wrapping_sub(1)
        }
        row += 1
    }
}

unsafe extern "C" fn quantize_ord_dither(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
)
/* General case, with ordered dithering */
{
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* points to active row of dither matrix */
    let mut input_ptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE; /* current indexes into dither matrix */
    let mut output_ptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut colorindex_ci: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut dither: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut row_index: libc::c_int = 0;
    let mut col_index: libc::c_int = 0;
    let mut nc: libc::c_int = (*cinfo).out_color_components;
    let mut ci: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut width: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    row = 0i32;
    while row < num_rows {
        /* Initialize output values to 0 so can process components separately */
        crate::jpegint_h::jzero_far(
            *output_buf.offset(row as isize) as *mut libc::c_void,
            (width as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>() as libc::c_ulong),
        );
        row_index = (*cquantize).row_index;
        ci = 0i32;
        while ci < nc {
            input_ptr = (*input_buf.offset(row as isize)).offset(ci as isize);
            output_ptr = *output_buf.offset(row as isize);
            colorindex_ci = *(*cquantize).colorindex.offset(ci as isize);
            dither = (*(*cquantize).odither[ci as usize].offset(row_index as isize)).as_mut_ptr();
            col_index = 0i32;
            col = width;
            while col > 0i32 as libc::c_uint {
                /* Form pixel value + dither, range-limit to 0..MAXJSAMPLE,
                 * select output value, accumulate into output code for this pixel.
                 * Range-limiting need not be done explicitly, as we have extended
                 * the colorindex table to produce the right answers for out-of-range
                 * inputs.  The maximum dither is +- MAXJSAMPLE; this sets the
                 * required amount of padding.
                 */
                *output_ptr = (*output_ptr as libc::c_int
                    + *colorindex_ci.offset(
                        (*input_ptr as libc::c_int + *dither.offset(col_index as isize)) as isize,
                    ) as libc::c_int) as crate::jmorecfg_h::JSAMPLE;
                input_ptr = input_ptr.offset(nc as isize);
                output_ptr = output_ptr.offset(1);
                col_index = col_index + 1i32 & ODITHER_MASK;
                col = col.wrapping_sub(1)
            }
            ci += 1
        }
        /* Advance row index for next row */
        row_index = row_index + 1i32 & ODITHER_MASK;
        (*cquantize).row_index = row_index;
        row += 1
    }
}

unsafe extern "C" fn quantize3_ord_dither(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
)
/* Fast path for out_color_components==3, with ordered dithering */
{
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* points to active row of dither matrix */
    let mut pixcode: libc::c_int = 0; /* current indexes into dither matrix */
    let mut input_ptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut output_ptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut colorindex0: crate::jpeglib_h::JSAMPROW = *(*cquantize).colorindex.offset(0);
    let mut colorindex1: crate::jpeglib_h::JSAMPROW = *(*cquantize).colorindex.offset(1);
    let mut colorindex2: crate::jpeglib_h::JSAMPROW = *(*cquantize).colorindex.offset(2);
    let mut dither0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dither1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dither2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut row_index: libc::c_int = 0;
    let mut col_index: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut width: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    row = 0i32;
    while row < num_rows {
        row_index = (*cquantize).row_index;
        input_ptr = *input_buf.offset(row as isize);
        output_ptr = *output_buf.offset(row as isize);
        dither0 = (*(*cquantize).odither[0].offset(row_index as isize)).as_mut_ptr();
        dither1 = (*(*cquantize).odither[1].offset(row_index as isize)).as_mut_ptr();
        dither2 = (*(*cquantize).odither[2].offset(row_index as isize)).as_mut_ptr();
        col_index = 0i32;
        col = width;
        while col > 0i32 as libc::c_uint {
            let fresh8 = input_ptr;
            input_ptr = input_ptr.offset(1);
            pixcode = *colorindex0
                .offset((*fresh8 as libc::c_int + *dither0.offset(col_index as isize)) as isize)
                as libc::c_int;
            let fresh9 = input_ptr;
            input_ptr = input_ptr.offset(1);
            pixcode += *colorindex1
                .offset((*fresh9 as libc::c_int + *dither1.offset(col_index as isize)) as isize)
                as libc::c_int;
            let fresh10 = input_ptr;
            input_ptr = input_ptr.offset(1);
            pixcode += *colorindex2
                .offset((*fresh10 as libc::c_int + *dither2.offset(col_index as isize)) as isize)
                as libc::c_int;
            let fresh11 = output_ptr;
            output_ptr = output_ptr.offset(1);
            *fresh11 = pixcode as crate::jmorecfg_h::JSAMPLE;
            col_index = col_index + 1i32 & ODITHER_MASK;
            col = col.wrapping_sub(1)
        }
        row_index = row_index + 1i32 & ODITHER_MASK;
        (*cquantize).row_index = row_index;
        row += 1
    }
}

unsafe extern "C" fn quantize_fs_dither(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
)
/* General case, with Floyd-Steinberg dithering */
{
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr; /* current error or pixel value */
    let mut cur: LOCFSERROR = 0; /* error for pixel below cur */
    let mut belowerr: LOCFSERROR = 0; /* error for below/prev col */
    let mut bpreverr: LOCFSERROR = 0; /* error for below/next col */
    let mut bnexterr: LOCFSERROR = 0; /* => fserrors[] at column before current */
    let mut delta: LOCFSERROR = 0; /* 1 for left-to-right, -1 for right-to-left */
    let mut errorptr: FSERRPTR = 0 as *mut FSERROR; /* dir * nc */
    let mut input_ptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut output_ptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut colorindex_ci: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut colormap_ci: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut pixcode: libc::c_int = 0;
    let mut nc: libc::c_int = (*cinfo).out_color_components;
    let mut dir: libc::c_int = 0;
    let mut dirnc: libc::c_int = 0;
    let mut ci: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut width: crate::jmorecfg_h::JDIMENSION = (*cinfo).output_width;
    let mut range_limit: *mut crate::jmorecfg_h::JSAMPLE = (*cinfo).sample_range_limit;
    row = 0i32;
    while row < num_rows {
        /* Initialize output values to 0 so can process components separately */
        crate::jpegint_h::jzero_far(
            *output_buf.offset(row as isize) as *mut libc::c_void,
            (width as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>() as libc::c_ulong),
        );
        ci = 0i32;
        while ci < nc {
            input_ptr = (*input_buf.offset(row as isize)).offset(ci as isize);
            output_ptr = *output_buf.offset(row as isize);
            if (*cquantize).on_odd_row != 0 {
                /* unload prev err into array */
                /* work right to left in this row */
                input_ptr = input_ptr.offset(
                    width
                        .wrapping_sub(1i32 as libc::c_uint)
                        .wrapping_mul(nc as libc::c_uint) as isize,
                );
                output_ptr = output_ptr.offset(width.wrapping_sub(1i32 as libc::c_uint) as isize);
                dir = -1i32;
                dirnc = -nc;
                errorptr = (*cquantize).fserrors[ci as usize]
                    .offset(width.wrapping_add(1i32 as libc::c_uint) as isize) /* so point to rightmost pixel */
            /* => entry after last column */
            } else {
                /* work left to right in this row */
                dir = 1i32;
                dirnc = nc;
                errorptr = (*cquantize).fserrors[ci as usize]
                /* => entry before first column */
            }
            colorindex_ci = *(*cquantize).colorindex.offset(ci as isize);
            colormap_ci = *(*cquantize).sv_colormap.offset(ci as isize);
            cur = 0i32;
            bpreverr = 0i32;
            belowerr = bpreverr;
            col = width;
            while col > 0i32 as libc::c_uint {
                /* Preset error values: no error propagated to first pixel from left */
                /* and no error propagated to row below yet */
                /* cur holds the error propagated from the previous pixel on the
                 * current line.  Add the error propagated from the previous line
                 * to form the complete error correction term for this pixel, and
                 * round the error term (which is expressed * 16) to an integer.
                 * RIGHT_SHIFT rounds towards minus infinity, so adding 8 is correct
                 * for either sign of the error value.
                 * Note: errorptr points to *previous* column's array entry.
                 */
                cur = cur + *errorptr.offset(dir as isize) as libc::c_int + 8i32 >> 4i32;
                /* advance errorptr to current column */
                cur += *input_ptr as libc::c_int;
                cur = *range_limit.offset(cur as isize) as libc::c_int;
                pixcode = *colorindex_ci.offset(cur as isize) as libc::c_int;
                *output_ptr = (*output_ptr as libc::c_int
                    + pixcode as crate::jmorecfg_h::JSAMPLE as libc::c_int)
                    as crate::jmorecfg_h::JSAMPLE;
                cur -= *colormap_ci.offset(pixcode as isize) as libc::c_int;
                bnexterr = cur;
                delta = cur * 2i32;
                cur += delta;
                *errorptr.offset(0) = (bpreverr + cur) as FSERROR;
                cur += delta;
                bpreverr = belowerr + cur;
                belowerr = bnexterr;
                cur += delta;
                input_ptr = input_ptr.offset(dirnc as isize);
                output_ptr = output_ptr.offset(dir as isize);
                errorptr = errorptr.offset(dir as isize);
                col = col.wrapping_sub(1)
            }
            *errorptr.offset(0) = bpreverr as FSERROR;
            ci += 1
        }
        (*cquantize).on_odd_row = if (*cquantize).on_odd_row != 0 {
            crate::jmorecfg_h::FALSE
        } else {
            crate::jmorecfg_h::TRUE
        };
        row += 1
    }
}
/* Form pixel value + error, and range-limit to 0..MAXJSAMPLE.
 * The maximum error is +- MAXJSAMPLE; this sets the required size
 * of the range_limit array.
 */
/* Select output value, accumulate into output code for this pixel */
/* Compute actual representation error at this pixel */
/* Note: we can do this even though we don't have the final */
/* pixel code, because the colormap is orthogonal. */
/* Compute error fractions to be propagated to adjacent pixels.
 * Add these into the running sums, and simultaneously shift the
 * next-line error sums left by 1 column.
 */
/* form error * 3 */
/* form error * 5 */
/* form error * 7 */
/* At this point cur contains the 7/16 error value to be propagated
 * to the next pixel on the current line, and all the errors for the
 * next line have been shifted over. We are therefore ready to move on.
 */
/* advance input ptr to next column */
/* advance output ptr to next column */
/* Post-loop cleanup: we must unload the final error value into the
 * final fserrors[] entry.  Note we need not unload belowerr because
 * it is for the dummy column before or after the actual array.
 */
/*
 * Allocate workspace for Floyd-Steinberg errors.
 */

unsafe extern "C" fn alloc_fs_workspace(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut arraysize: crate::stddef_h::size_t = 0;
    let mut i: libc::c_int = 0;
    arraysize = ((*cinfo).output_width.wrapping_add(2i32 as libc::c_uint) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<FSERROR>() as libc::c_ulong);
    i = 0i32;
    while i < (*cinfo).out_color_components {
        (*cquantize).fserrors[i as usize] = Some(
            (*(*cinfo).mem)
                .alloc_large
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            arraysize,
        ) as FSERRPTR;
        i += 1
    }
}
/*
 * Initialize for one-pass color quantization.
 */

unsafe extern "C" fn start_pass_1_quant(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut is_pre_scan: crate::jmorecfg_h::boolean,
) {
    let mut cquantize: my_cquantize_ptr = (*cinfo).cquantize as my_cquantize_ptr;
    let mut arraysize: crate::stddef_h::size_t = 0;
    let mut i: libc::c_int = 0;
    /* Install my colormap. */
    (*cinfo).colormap = (*cquantize).sv_colormap;
    (*cinfo).actual_number_of_colors = (*cquantize).sv_actual;
    /* Initialize for desired dithering mode. */
    match (*cinfo).dither_mode as libc::c_uint {
        0 => {
            if (*cinfo).out_color_components == 3i32 {
                (*cquantize).pub_0.color_quantize = Some(
                    color_quantize3
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                )
            } else {
                (*cquantize).pub_0.color_quantize = Some(
                    color_quantize
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                )
            }
        }
        1 => {
            if (*cinfo).out_color_components == 3i32 {
                (*cquantize).pub_0.color_quantize = Some(
                    quantize3_ord_dither
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                )
            } else {
                (*cquantize).pub_0.color_quantize = Some(
                    quantize_ord_dither
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: libc::c_int,
                        ) -> (),
                )
            } /* initialize state for ordered dither */
            (*cquantize).row_index = 0i32;
            /* If user changed to ordered dither from another mode,
             * we must recreate the color index table with padding.
             * This will cost extra space, but probably isn't very likely.
             */
            if (*cquantize).is_padded == 0 {
                create_colorindex(cinfo);
            }
            /* Create ordered-dither tables if we didn't already. */
            if (*cquantize).odither[0].is_null() {
                create_odither_tables(cinfo); /* initialize state for F-S dither */
            }
        }
        2 => {
            (*cquantize).pub_0.color_quantize = Some(
                quantize_fs_dither
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_decompress_ptr,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: libc::c_int,
                    ) -> (),
            );
            (*cquantize).on_odd_row = crate::jmorecfg_h::FALSE;
            /* Allocate Floyd-Steinberg workspace if didn't already. */
            if (*cquantize).fserrors[0].is_null() {
                alloc_fs_workspace(cinfo);
            }
            /* Initialize the propagated errors to zero. */
            arraysize = ((*cinfo).output_width.wrapping_add(2i32 as libc::c_uint) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<FSERROR>() as libc::c_ulong);
            i = 0i32;
            while i < (*cinfo).out_color_components {
                crate::jpegint_h::jzero_far(
                    (*cquantize).fserrors[i as usize] as *mut libc::c_void,
                    arraysize,
                );
                i += 1
            }
        }
        _ => {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_NOT_COMPILED as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
    };
}
/*
 * Finish up at the end of the pass.
 */

unsafe extern "C" fn finish_pass_1_quant(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    /* no work in 1-pass case */
}
/*
 * Switch to a new external colormap between output passes.
 * Shouldn't get to this module!
 */

unsafe extern "C" fn new_color_map_1_quant(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    (*(*cinfo).err).msg_code = crate::src::jerror::JERR_MODE_CHANGE as libc::c_int;
    Some(
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
}
/*
 * Module initialization routine for 1-pass color quantization.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_1pass_quantizer(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut cquantize: my_cquantize_ptr = 0 as *mut my_cquantizer; /* Flag FS workspace not allocated */
    cquantize = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<my_cquantizer>() as libc::c_ulong,
    ) as my_cquantize_ptr; /* Also flag odither arrays not allocated */
    (*cinfo).cquantize = cquantize as *mut crate::jpeglib_h::jpeg_color_quantizer;
    (*cquantize).pub_0.start_pass = Some(
        start_pass_1_quant
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_decompress_ptr,
                _: crate::jmorecfg_h::boolean,
            ) -> (),
    );
    (*cquantize).pub_0.finish_pass = Some(
        finish_pass_1_quant as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> (),
    );
    (*cquantize).pub_0.new_color_map = Some(
        new_color_map_1_quant as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> (),
    );
    (*cquantize).fserrors[0] = crate::stddef_h::NULL as FSERRPTR;
    (*cquantize).odither[0] = crate::stddef_h::NULL as ODITHER_MATRIX_PTR;
    /* Make sure my internal arrays won't overflow */
    if (*cinfo).out_color_components > MAX_Q_COMPS {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_QUANT_COMPONENTS as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = 4i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Make sure colormap indexes can be represented by JSAMPLEs */
    if (*cinfo).desired_number_of_colors > crate::jmorecfg_h::MAXJSAMPLE + 1i32 {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_QUANT_MANY_COLORS as libc::c_int;
        (*(*cinfo).err).msg_parm.i[0] = 255i32 + 1i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Create the colormap and color index table. */
    create_colormap(cinfo);
    create_colorindex(cinfo);
    /* Allocate Floyd-Steinberg workspace now if requested.
     * We do this now since it may affect the memory manager's space
     * calculations.  If the user changes to FS dither mode in a later pass, we
     * will allocate the space then, and will possibly overrun the
     * max_memory_to_use setting.
     */
    if (*cinfo).dither_mode as libc::c_uint
        == crate::jpeglib_h::JDITHER_FS as libc::c_int as libc::c_uint
    {
        alloc_fs_workspace(cinfo);
    };
}
/* QUANT_1PASS_SUPPORTED */
