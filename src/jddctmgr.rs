use ::libc;

pub use crate::jdct_h::FLOAT_MULT_TYPE;
pub use crate::jdct_h::IFAST_MULT_TYPE;
pub use crate::jdct_h::ISLOW_MULT_TYPE;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::INT16;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jpeg_color_deconverter;
pub use crate::jpegint_h::jpeg_color_quantizer;
pub use crate::jpegint_h::jpeg_d_coef_controller;
pub use crate::jpegint_h::jpeg_d_main_controller;
pub use crate::jpegint_h::jpeg_d_post_controller;
pub use crate::jpegint_h::jpeg_decomp_master;
pub use crate::jpegint_h::jpeg_entropy_decoder;
pub use crate::jpegint_h::jpeg_input_controller;
pub use crate::jpegint_h::jpeg_inverse_dct;
pub use crate::jpegint_h::jpeg_marker_reader;
pub use crate::jpegint_h::jpeg_upsampler;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::JLONG;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_idct_method;
pub use crate::jpeglib_h::jpeg_idct_method_selector;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::DCTSIZE2;
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
pub use crate::src::jdmaster::my_decomp_master;
pub use crate::src::jdmaster::my_master_ptr;
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
pub use crate::src::jidctflt::jpeg_idct_float;
pub use crate::src::jidctfst::jpeg_idct_ifast;
pub use crate::src::jidctint::jpeg_idct_10x10;
pub use crate::src::jidctint::jpeg_idct_11x11;
pub use crate::src::jidctint::jpeg_idct_12x12;
pub use crate::src::jidctint::jpeg_idct_13x13;
pub use crate::src::jidctint::jpeg_idct_14x14;
pub use crate::src::jidctint::jpeg_idct_15x15;
pub use crate::src::jidctint::jpeg_idct_16x16;
pub use crate::src::jidctint::jpeg_idct_3x3;
pub use crate::src::jidctint::jpeg_idct_5x5;
pub use crate::src::jidctint::jpeg_idct_6x6;
pub use crate::src::jidctint::jpeg_idct_7x7;
pub use crate::src::jidctint::jpeg_idct_9x9;
pub use crate::src::jidctint::jpeg_idct_islow;
pub use crate::src::jidctred::jpeg_idct_1x1;
pub use crate::src::jidctred::jpeg_idct_2x2;
pub use crate::src::jidctred::jpeg_idct_4x4;
use crate::src::simd::x86_64::jsimd::jsimd_can_idct_2x2;
use crate::src::simd::x86_64::jsimd::jsimd_can_idct_4x4;
use crate::src::simd::x86_64::jsimd::jsimd_can_idct_float;
use crate::src::simd::x86_64::jsimd::jsimd_can_idct_ifast;
use crate::src::simd::x86_64::jsimd::jsimd_can_idct_islow;
use crate::src::simd::x86_64::jsimd::jsimd_idct_2x2;
use crate::src::simd::x86_64::jsimd::jsimd_idct_4x4;
use crate::src::simd::x86_64::jsimd::jsimd_idct_float;
use crate::src::simd::x86_64::jsimd::jsimd_idct_ifast;
use crate::src::simd::x86_64::jsimd::jsimd_idct_islow;
pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
use crate::stdlib::memset;
pub use crate::stdlib::C2RustUnnamed_0;

pub type my_idct_ptr = *mut my_idct_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_idct_controller {
    pub pub_0: crate::jpegint_h::jpeg_inverse_dct,
    pub cur_method: [libc::c_int; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union multiplier_table {
    pub islow_array: [crate::jdct_h::ISLOW_MULT_TYPE; 64],
    pub ifast_array: [crate::jdct_h::IFAST_MULT_TYPE; 64],
    pub float_array: [crate::jdct_h::FLOAT_MULT_TYPE; 64],
}
/* Method pointers */
/* Limit on memory allocation for this JPEG object.  (Note that this is
 * merely advisory, not a guaranteed maximum; it only affects the space
 * used for virtual-array buffers.)  May be changed by outer application
 * after creating the JPEG object.
 */
/* Maximum allocation request accepted by alloc_large. */
/* Routine signature for application-supplied marker processing methods.
 * Need not pass marker code since it is stored in cinfo->unread_marker.
 */
/* Originally, this macro was used as a way of defining function prototypes
 * for both modern compilers as well as older compilers that did not support
 * prototype parameters.  libjpeg-turbo has never supported these older,
 * non-ANSI compilers, but the macro is still included because there is some
 * software out there that uses it.
 */
/* Default error-management setup */
/* Initialization of JPEG compression objects.
 * jpeg_create_compress() and jpeg_create_decompress() are the exported
 * names that applications should call.  These expand to calls on
 * jpeg_CreateCompress and jpeg_CreateDecompress with additional information
 * passed for version mismatch checking.
 * NB: you must set up the error-manager BEFORE calling jpeg_create_xxx.
 */
/* Destruction of JPEG compression objects */
/* Standard data source and destination managers: stdio streams. */
/* Caller is responsible for opening the file before and closing after. */
/* Data source and destination managers: memory buffers. */
/* Default parameter setup for compression */
/* Compression parameter setup aids */
/* Main entry points for compression */
/* Replaces jpeg_write_scanlines when writing raw downsampled data. */
/* Write a special marker.  See libjpeg.txt concerning safe usage. */
/* Same, but piecemeal. */
/* Alternate compression function: just write an abbreviated table file */
/* Write ICC profile.  See libjpeg.txt for usage information. */
/* Decompression startup: read start of JPEG datastream to see what's there */
/* Return value is one of: */
/* Suspended due to lack of input data */
/* Found valid image datastream */
/* Found valid table-specs-only datastream */
/* If you pass require_image = TRUE (normal case), you need not check for
 * a TABLES_ONLY return code; an abbreviated file will cause an error exit.
 * JPEG_SUSPENDED is only possible if you use a data source module that can
 * give a suspension return (the stdio source module doesn't).
 */
/* Main entry points for decompression */
/* Replaces jpeg_read_scanlines when reading raw downsampled data. */
/* Additional entry points for buffered-image mode. */
/* Return value is one of: */
/* #define JPEG_SUSPENDED       0    Suspended due to lack of input data */
/* Reached start of new scan */
/* Reached end of image */
/* Completed one iMCU row */
/* Completed last iMCU row of a scan */
/* Precalculate output dimensions for current decompression parameters. */
/* Control saving of COM and APPn markers into marker_list. */
/* Install a special processing method for COM or APPn markers. */
/* Read or write raw DCT coefficients --- useful for lossless transcoding. */
/* If you choose to abort compression or decompression before completing
 * jpeg_finish_(de)compress, then you need to clean up to release memory,
 * temporary files, etc.  You can just call jpeg_destroy_(de)compress
 * if you're done with the JPEG object, but if you want to clean it up and
 * reuse it, call this:
 */
/* Generic versions of jpeg_abort and jpeg_destroy that work on either
 * flavor of JPEG object.  These may be more convenient in some places.
 */
/* Default restart-marker-resync procedure for use by data source modules */
/* Accessor functions for extension parameters */
/* Read ICC profile.  See libjpeg.txt for usage information. */
/*
 * Permit users to replace the IDCT method dynamically.
 * The selector callback is called after the default idct implementation was choosen,
 * and is able to override it.
 */
/* The current scaled-IDCT routines require ISLOW-style multiplier tables,
 * so be sure to compile that code if either ISLOW or SCALING is requested.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_set_idct_method_selector(
    mut cinfo: crate::jpeglib_h::j_decompress_ptr,
    mut selector: crate::jpeglib_h::jpeg_idct_method_selector,
) {
    let mut master: crate::src::jdmaster::my_master_ptr =
        (*cinfo).master as crate::src::jdmaster::my_master_ptr;
    (*master).custom_idct_selector = selector;
}
/*
 * Prepare for an output pass.
 * Here we select the proper IDCT routine for each component and build
 * a matching multiplier table.
 */

unsafe extern "C" fn start_pass(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut idct: my_idct_ptr = (*cinfo).idct as my_idct_ptr;
    let mut ci: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    let mut method: libc::c_int = 0 as libc::c_int;
    let mut method_ptr: crate::jpegint_h::inverse_DCT_method_ptr =
        ::std::mem::transmute::<libc::intptr_t, crate::jpegint_h::inverse_DCT_method_ptr>(
            crate::stddef_h::NULL as libc::intptr_t,
        );
    let mut qtbl: *mut crate::jpeglib_h::JQUANT_TBL = 0 as *mut crate::jpeglib_h::JQUANT_TBL;
    let mut master: crate::src::jdmaster::my_master_ptr =
        0 as *mut crate::src::jdmaster::my_decomp_master;
    ci = 0 as libc::c_int;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Select the proper IDCT routine for this component's scaling */
        match (*compptr).DCT_scaled_size {
            1 => {
                method_ptr = Some(
                    crate::src::jidctred::jpeg_idct_1x1
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JCOEFPTR,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jidctred uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as libc::c_int
            }
            2 => {
                if crate::src::simd::x86_64::jsimd::jsimd_can_idct_2x2() != 0 {
                    method_ptr = Some(
                        crate::src::simd::x86_64::jsimd::jsimd_idct_2x2
                            as unsafe extern "C" fn(
                                _: crate::jpeglib_h::j_decompress_ptr,
                                _: *mut crate::jpeglib_h::jpeg_component_info,
                                _: crate::jpeglib_h::JCOEFPTR,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: crate::jmorecfg_h::JDIMENSION,
                            ) -> (),
                    )
                } else {
                    method_ptr = Some(
                        crate::src::jidctred::jpeg_idct_2x2
                            as unsafe extern "C" fn(
                                _: crate::jpeglib_h::j_decompress_ptr,
                                _: *mut crate::jpeglib_h::jpeg_component_info,
                                _: crate::jpeglib_h::JCOEFPTR,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: crate::jmorecfg_h::JDIMENSION,
                            ) -> (),
                    )
                } /* jidctred uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as libc::c_int
            }
            3 => {
                method_ptr = Some(
                    crate::src::jidctint::jpeg_idct_3x3
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JCOEFPTR,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as libc::c_int
            }
            4 => {
                if crate::src::simd::x86_64::jsimd::jsimd_can_idct_4x4() != 0 {
                    method_ptr = Some(
                        crate::src::simd::x86_64::jsimd::jsimd_idct_4x4
                            as unsafe extern "C" fn(
                                _: crate::jpeglib_h::j_decompress_ptr,
                                _: *mut crate::jpeglib_h::jpeg_component_info,
                                _: crate::jpeglib_h::JCOEFPTR,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: crate::jmorecfg_h::JDIMENSION,
                            ) -> (),
                    )
                } else {
                    method_ptr = Some(
                        crate::src::jidctred::jpeg_idct_4x4
                            as unsafe extern "C" fn(
                                _: crate::jpeglib_h::j_decompress_ptr,
                                _: *mut crate::jpeglib_h::jpeg_component_info,
                                _: crate::jpeglib_h::JCOEFPTR,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: crate::jmorecfg_h::JDIMENSION,
                            ) -> (),
                    )
                } /* jidctred uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as libc::c_int
            }
            5 => {
                method_ptr = Some(
                    crate::src::jidctint::jpeg_idct_5x5
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JCOEFPTR,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as libc::c_int
            }
            6 => {
                method_ptr = Some(
                    crate::src::jidctint::jpeg_idct_6x6
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JCOEFPTR,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as libc::c_int
            }
            7 => {
                method_ptr = Some(
                    crate::src::jidctint::jpeg_idct_7x7
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JCOEFPTR,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as libc::c_int
            }
            8 => {
                match (*cinfo).dct_method as libc::c_uint {
                    0 => {
                        if crate::src::simd::x86_64::jsimd::jsimd_can_idct_islow() != 0 {
                            method_ptr = Some(
                                crate::src::simd::x86_64::jsimd::jsimd_idct_islow
                                    as unsafe extern "C" fn(
                                        _: crate::jpeglib_h::j_decompress_ptr,
                                        _: *mut crate::jpeglib_h::jpeg_component_info,
                                        _: crate::jpeglib_h::JCOEFPTR,
                                        _: crate::jpeglib_h::JSAMPARRAY,
                                        _: crate::jmorecfg_h::JDIMENSION,
                                    )
                                        -> (),
                            )
                        } else {
                            method_ptr = Some(
                                crate::src::jidctint::jpeg_idct_islow
                                    as unsafe extern "C" fn(
                                        _: crate::jpeglib_h::j_decompress_ptr,
                                        _: *mut crate::jpeglib_h::jpeg_component_info,
                                        _: crate::jpeglib_h::JCOEFPTR,
                                        _: crate::jpeglib_h::JSAMPARRAY,
                                        _: crate::jmorecfg_h::JDIMENSION,
                                    )
                                        -> (),
                            )
                        } /* jidctint uses islow-style table */
                        method = crate::jpeglib_h::JDCT_ISLOW as libc::c_int
                    }
                    1 => {
                        if crate::src::simd::x86_64::jsimd::jsimd_can_idct_ifast() != 0 {
                            method_ptr = Some(
                                crate::src::simd::x86_64::jsimd::jsimd_idct_ifast
                                    as unsafe extern "C" fn(
                                        _: crate::jpeglib_h::j_decompress_ptr,
                                        _: *mut crate::jpeglib_h::jpeg_component_info,
                                        _: crate::jpeglib_h::JCOEFPTR,
                                        _: crate::jpeglib_h::JSAMPARRAY,
                                        _: crate::jmorecfg_h::JDIMENSION,
                                    )
                                        -> (),
                            )
                        } else {
                            method_ptr = Some(
                                crate::src::jidctfst::jpeg_idct_ifast
                                    as unsafe extern "C" fn(
                                        _: crate::jpeglib_h::j_decompress_ptr,
                                        _: *mut crate::jpeglib_h::jpeg_component_info,
                                        _: crate::jpeglib_h::JCOEFPTR,
                                        _: crate::jpeglib_h::JSAMPARRAY,
                                        _: crate::jmorecfg_h::JDIMENSION,
                                    )
                                        -> (),
                            )
                        } /* jidctint uses islow-style table */
                        method = crate::jpeglib_h::JDCT_IFAST as libc::c_int
                    }
                    2 => {
                        if crate::src::simd::x86_64::jsimd::jsimd_can_idct_float() != 0 {
                            method_ptr = Some(
                                crate::src::simd::x86_64::jsimd::jsimd_idct_float
                                    as unsafe extern "C" fn(
                                        _: crate::jpeglib_h::j_decompress_ptr,
                                        _: *mut crate::jpeglib_h::jpeg_component_info,
                                        _: crate::jpeglib_h::JCOEFPTR,
                                        _: crate::jpeglib_h::JSAMPARRAY,
                                        _: crate::jmorecfg_h::JDIMENSION,
                                    )
                                        -> (),
                            )
                        } else {
                            method_ptr = Some(
                                crate::src::jidctflt::jpeg_idct_float
                                    as unsafe extern "C" fn(
                                        _: crate::jpeglib_h::j_decompress_ptr,
                                        _: *mut crate::jpeglib_h::jpeg_component_info,
                                        _: crate::jpeglib_h::JCOEFPTR,
                                        _: crate::jpeglib_h::JSAMPARRAY,
                                        _: crate::jmorecfg_h::JDIMENSION,
                                    )
                                        -> (),
                            )
                        } /* jidctint uses islow-style table */
                        method = crate::jpeglib_h::JDCT_FLOAT as libc::c_int
                    }
                    _ => {
                        (*(*cinfo).err).msg_code =
                            crate::src::jerror::JERR_NOT_COMPILED as libc::c_int; /* jidctint uses islow-style table */
                        Some(
                            (*(*cinfo).err)
                                .error_exit
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as crate::jpeglib_h::j_common_ptr,
                        ); /* jidctint uses islow-style table */
                    }
                }
            }
            9 => {
                method_ptr = Some(
                    crate::src::jidctint::jpeg_idct_9x9
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JCOEFPTR,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as libc::c_int
            }
            10 => {
                method_ptr = Some(
                    crate::src::jidctint::jpeg_idct_10x10
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JCOEFPTR,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as libc::c_int
            }
            11 => {
                method_ptr = Some(
                    crate::src::jidctint::jpeg_idct_11x11
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JCOEFPTR,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                ); /* jidctint uses islow-style table */
                method = crate::jpeglib_h::JDCT_ISLOW as libc::c_int
            }
            12 => {
                method_ptr = Some(
                    crate::src::jidctint::jpeg_idct_12x12
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JCOEFPTR,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                );
                method = crate::jpeglib_h::JDCT_ISLOW as libc::c_int
            }
            13 => {
                method_ptr = Some(
                    crate::src::jidctint::jpeg_idct_13x13
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JCOEFPTR,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                );
                method = crate::jpeglib_h::JDCT_ISLOW as libc::c_int
            }
            14 => {
                method_ptr = Some(
                    crate::src::jidctint::jpeg_idct_14x14
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JCOEFPTR,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                );
                method = crate::jpeglib_h::JDCT_ISLOW as libc::c_int
            }
            15 => {
                method_ptr = Some(
                    crate::src::jidctint::jpeg_idct_15x15
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JCOEFPTR,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                );
                method = crate::jpeglib_h::JDCT_ISLOW as libc::c_int
            }
            16 => {
                method_ptr = Some(
                    crate::src::jidctint::jpeg_idct_16x16
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_decompress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JCOEFPTR,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jmorecfg_h::JDIMENSION,
                        ) -> (),
                );
                method = crate::jpeglib_h::JDCT_ISLOW as libc::c_int
            }
            _ => {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_DCTSIZE as libc::c_int;
                (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = (*compptr).DCT_scaled_size;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
        }
        // Allow custom idct function to be set dynamically
        master = (*cinfo).master as crate::src::jdmaster::my_master_ptr;
        if (*master).custom_idct_selector.is_some() {
            (*master)
                .custom_idct_selector
                .expect("non-null function pointer")(
                cinfo, compptr, &mut method_ptr, &mut method
            );
        }
        (*idct).pub_0.inverse_DCT[ci as usize] = method_ptr;
        /* Create multiplier table from quant table.
         * However, we can skip this if the component is uninteresting
         * or if we already built the table.  Also, if no quant table
         * has yet been saved for the component, we leave the
         * multiplier table all-zero; we'll be reading zeroes from the
         * coefficient controller's buffer anyway.
         */
        if !((*compptr).component_needed == 0 || (*idct).cur_method[ci as usize] == method) {
            qtbl = (*compptr).quant_table;
            if !qtbl.is_null() {
                (*idct).cur_method[ci as usize] = method;
                match method {
                    0 => {
                        /* For LL&M IDCT method, multipliers are equal to raw quantization
                         * coefficients, but are stored as ints to ensure access efficiency.
                         */
                        let mut ismtbl: *mut crate::jdct_h::ISLOW_MULT_TYPE =
                            (*compptr).dct_table as *mut crate::jdct_h::ISLOW_MULT_TYPE;
                        i = 0 as libc::c_int;
                        while i < crate::jpeglib_h::DCTSIZE2 {
                            *ismtbl.offset(i as isize) =
                                (*qtbl).quantval[i as usize] as crate::jdct_h::ISLOW_MULT_TYPE;
                            i += 1
                        }
                    }
                    1 => {
                        /* For AA&N IDCT method, multipliers are equal to quantization
                         * coefficients scaled by scalefactor[row]*scalefactor[col], where
                         *   scalefactor[0] = 1
                         *   scalefactor[k] = cos(k*PI/16) * sqrt(2)    for k=1..7
                         * For integer operation, the multiplier table is to be scaled by
                         * IFAST_SCALE_BITS.
                         */
                        let mut ifmtbl: *mut crate::jdct_h::IFAST_MULT_TYPE =
                            (*compptr).dct_table as *mut crate::jdct_h::IFAST_MULT_TYPE;
                        static mut aanscales: [crate::jmorecfg_h::INT16; 64] = [
                            16384 as libc::c_int as crate::jmorecfg_h::INT16,
                            22725 as libc::c_int as crate::jmorecfg_h::INT16,
                            21407 as libc::c_int as crate::jmorecfg_h::INT16,
                            19266 as libc::c_int as crate::jmorecfg_h::INT16,
                            16384 as libc::c_int as crate::jmorecfg_h::INT16,
                            12873 as libc::c_int as crate::jmorecfg_h::INT16,
                            8867 as libc::c_int as crate::jmorecfg_h::INT16,
                            4520 as libc::c_int as crate::jmorecfg_h::INT16,
                            22725 as libc::c_int as crate::jmorecfg_h::INT16,
                            31521 as libc::c_int as crate::jmorecfg_h::INT16,
                            29692 as libc::c_int as crate::jmorecfg_h::INT16,
                            26722 as libc::c_int as crate::jmorecfg_h::INT16,
                            22725 as libc::c_int as crate::jmorecfg_h::INT16,
                            17855 as libc::c_int as crate::jmorecfg_h::INT16,
                            12299 as libc::c_int as crate::jmorecfg_h::INT16,
                            6270 as libc::c_int as crate::jmorecfg_h::INT16,
                            21407 as libc::c_int as crate::jmorecfg_h::INT16,
                            29692 as libc::c_int as crate::jmorecfg_h::INT16,
                            27969 as libc::c_int as crate::jmorecfg_h::INT16,
                            25172 as libc::c_int as crate::jmorecfg_h::INT16,
                            21407 as libc::c_int as crate::jmorecfg_h::INT16,
                            16819 as libc::c_int as crate::jmorecfg_h::INT16,
                            11585 as libc::c_int as crate::jmorecfg_h::INT16,
                            5906 as libc::c_int as crate::jmorecfg_h::INT16,
                            19266 as libc::c_int as crate::jmorecfg_h::INT16,
                            26722 as libc::c_int as crate::jmorecfg_h::INT16,
                            25172 as libc::c_int as crate::jmorecfg_h::INT16,
                            22654 as libc::c_int as crate::jmorecfg_h::INT16,
                            19266 as libc::c_int as crate::jmorecfg_h::INT16,
                            15137 as libc::c_int as crate::jmorecfg_h::INT16,
                            10426 as libc::c_int as crate::jmorecfg_h::INT16,
                            5315 as libc::c_int as crate::jmorecfg_h::INT16,
                            16384 as libc::c_int as crate::jmorecfg_h::INT16,
                            22725 as libc::c_int as crate::jmorecfg_h::INT16,
                            21407 as libc::c_int as crate::jmorecfg_h::INT16,
                            19266 as libc::c_int as crate::jmorecfg_h::INT16,
                            16384 as libc::c_int as crate::jmorecfg_h::INT16,
                            12873 as libc::c_int as crate::jmorecfg_h::INT16,
                            8867 as libc::c_int as crate::jmorecfg_h::INT16,
                            4520 as libc::c_int as crate::jmorecfg_h::INT16,
                            12873 as libc::c_int as crate::jmorecfg_h::INT16,
                            17855 as libc::c_int as crate::jmorecfg_h::INT16,
                            16819 as libc::c_int as crate::jmorecfg_h::INT16,
                            15137 as libc::c_int as crate::jmorecfg_h::INT16,
                            12873 as libc::c_int as crate::jmorecfg_h::INT16,
                            10114 as libc::c_int as crate::jmorecfg_h::INT16,
                            6967 as libc::c_int as crate::jmorecfg_h::INT16,
                            3552 as libc::c_int as crate::jmorecfg_h::INT16,
                            8867 as libc::c_int as crate::jmorecfg_h::INT16,
                            12299 as libc::c_int as crate::jmorecfg_h::INT16,
                            11585 as libc::c_int as crate::jmorecfg_h::INT16,
                            10426 as libc::c_int as crate::jmorecfg_h::INT16,
                            8867 as libc::c_int as crate::jmorecfg_h::INT16,
                            6967 as libc::c_int as crate::jmorecfg_h::INT16,
                            4799 as libc::c_int as crate::jmorecfg_h::INT16,
                            2446 as libc::c_int as crate::jmorecfg_h::INT16,
                            4520 as libc::c_int as crate::jmorecfg_h::INT16,
                            6270 as libc::c_int as crate::jmorecfg_h::INT16,
                            5906 as libc::c_int as crate::jmorecfg_h::INT16,
                            5315 as libc::c_int as crate::jmorecfg_h::INT16,
                            4520 as libc::c_int as crate::jmorecfg_h::INT16,
                            3552 as libc::c_int as crate::jmorecfg_h::INT16,
                            2446 as libc::c_int as crate::jmorecfg_h::INT16,
                            1247 as libc::c_int as crate::jmorecfg_h::INT16,
                        ];
                        i = 0 as libc::c_int;
                        while i < crate::jpeglib_h::DCTSIZE2 {
                            *ifmtbl.offset(i as isize) = ((*qtbl).quantval[i as usize]
                                as crate::jpegint_h::JLONG
                                * aanscales[i as usize] as crate::jpegint_h::JLONG
                                + ((1 as libc::c_int as crate::jpegint_h::JLONG)
                                    << 14 as libc::c_int - 2 as libc::c_int - 1 as libc::c_int)
                                >> 14 as libc::c_int - 2 as libc::c_int)
                                as crate::jdct_h::IFAST_MULT_TYPE;
                            i += 1
                        }
                    }
                    2 => {
                        /* For float AA&N IDCT method, multipliers are equal to quantization
                         * coefficients scaled by scalefactor[row]*scalefactor[col], where
                         *   scalefactor[0] = 1
                         *   scalefactor[k] = cos(k*PI/16) * sqrt(2)    for k=1..7
                         */
                        let mut fmtbl: *mut crate::jdct_h::FLOAT_MULT_TYPE =
                            (*compptr).dct_table as *mut crate::jdct_h::FLOAT_MULT_TYPE;
                        let mut row: libc::c_int = 0;
                        let mut col: libc::c_int = 0;
                        static mut aanscalefactor: [libc::c_double; 8] = [
                            1.0f64,
                            1.387039845f64,
                            1.306562965f64,
                            1.175875602f64,
                            1.0f64,
                            0.785694958f64,
                            0.541196100f64,
                            0.275899379f64,
                        ];
                        i = 0 as libc::c_int;
                        row = 0 as libc::c_int;
                        while row < 8 as libc::c_int {
                            col = 0 as libc::c_int;
                            while col < 8 as libc::c_int {
                                *fmtbl.offset(i as isize) = ((*qtbl).quantval[i as usize]
                                    as libc::c_double
                                    * aanscalefactor[row as usize]
                                    * aanscalefactor[col as usize])
                                    as crate::jdct_h::FLOAT_MULT_TYPE;
                                i += 1;
                                col += 1
                            }
                            row += 1
                        }
                    }
                    _ => {
                        (*(*cinfo).err).msg_code =
                            crate::src::jerror::JERR_NOT_COMPILED as libc::c_int;
                        Some(
                            (*(*cinfo).err)
                                .error_exit
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as crate::jpeglib_h::j_common_ptr,
                        );
                    }
                }
            }
        }
        /* happens if no data yet for component */
        ci += 1;
        compptr = compptr.offset(1)
    }
}
/*
 * Initialize IDCT manager.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_inverse_dct(mut cinfo: crate::jpeglib_h::j_decompress_ptr) {
    let mut idct: my_idct_ptr = 0 as *mut my_idct_controller;
    let mut ci: libc::c_int = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    idct = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<my_idct_controller>() as libc::c_ulong,
    ) as my_idct_ptr;
    (*cinfo).idct = idct as *mut crate::jpegint_h::jpeg_inverse_dct;
    (*idct).pub_0.start_pass =
        Some(start_pass as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> ());
    ci = 0 as libc::c_int;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Allocate and pre-zero a multiplier table for each component */
        (*compptr).dct_table = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            ::std::mem::size_of::<multiplier_table>() as libc::c_ulong,
        );
        crate::stdlib::memset(
            (*compptr).dct_table,
            0 as libc::c_int,
            ::std::mem::size_of::<multiplier_table>() as libc::c_ulong,
        );
        /* Mark multiplier table not yet set up for any method */
        (*idct).cur_method[ci as usize] = -(1 as libc::c_int);
        ci += 1;
        compptr = compptr.offset(1)
    }
}
