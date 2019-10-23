use libc;

pub use crate::stddef_h::size_t;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::jcopy_sample_rows;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::JLONG;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_c_coef_controller;
pub use crate::jpeglib_h::jpeg_c_main_controller;
pub use crate::jpeglib_h::jpeg_c_prep_controller;
pub use crate::jpeglib_h::jpeg_color_converter;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_comp_master;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_downsampler;
pub use crate::jpeglib_h::jpeg_entropy_encoder;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_forward_dct;
pub use crate::jpeglib_h::jpeg_marker_writer;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::DCTSIZE;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
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
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
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
use crate::src::simd::x86_64::jsimd::jsimd_can_h2v1_downsample;
use crate::src::simd::x86_64::jsimd::jsimd_can_h2v2_downsample;
use crate::src::simd::x86_64::jsimd::jsimd_h2v1_downsample;
use crate::src::simd::x86_64::jsimd::jsimd_h2v2_downsample;
/*
 * jcsample.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
 * Copyright (C) 2014, MIPS Technologies, Inc., California.
 * Copyright (C) 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains downsampling routines.
 *
 * Downsampling input data is counted in "row groups".  A row group
 * is defined to be max_v_samp_factor pixel rows of each component,
 * from which the downsampler produces v_samp_factor sample rows.
 * A single row group is processed in each call to the downsampler module.
 *
 * The downsampler is responsible for edge-expansion of its output data
 * to fill an integral number of DCT blocks horizontally.  The source buffer
 * may be modified if it is helpful for this purpose (the source buffer is
 * allocated wide enough to correspond to the desired output width).
 * The caller (the prep controller) is responsible for vertical padding.
 *
 * The downsampler may request "context rows" by setting need_context_rows
 * during startup.  In this case, the input arrays will contain at least
 * one row group's worth of pixels above and below the passed-in data;
 * the caller will create dummy rows at image top and bottom by replicating
 * the first or last real pixel row.
 *
 * An excellent reference for image resampling is
 *   Digital Image Warping, George Wolberg, 1990.
 *   Pub. by IEEE Computer Society Press, Los Alamitos, CA. ISBN 0-8186-8944-7.
 *
 * The downsampling algorithm used here is a simple average of the source
 * pixels covered by the output pixel.  The hi-falutin sampling literature
 * refers to this as a "box filter".  In general the characteristics of a box
 * filter are not very good, but for the specific cases we normally use (1:1
 * and 2:1 ratios) the box is equivalent to a "triangle filter" which is not
 * nearly so bad.  If you intend to use other sampling ratios, you'd be well
 * advised to improve this code.
 *
 * A simple input-smoothing capability is provided.  This is mainly intended
 * for cleaning up color-dithered GIF input files (if you find it inadequate,
 * we suggest using an external filtering program such as pnmconvol).  When
 * enabled, each input pixel P is replaced by a weighted sum of itself and its
 * eight neighbors.  P's weight is 1-8*SF and each neighbor's weight is SF,
 * where SF = (smoothing_factor / 1024).
 * Currently, smoothing is only supported for 2h2v sampling factors.
 */
/* Pointer to routine to downsample a single component */

pub type downsample1_ptr = Option<
    unsafe extern "C" fn(
        _: crate::jpeglib_h::j_compress_ptr,
        _: *mut crate::jpeglib_h::jpeg_component_info,
        _: crate::jpeglib_h::JSAMPARRAY,
        _: crate::jpeglib_h::JSAMPARRAY,
    ) -> (),
>;

pub type my_downsample_ptr = *mut my_downsampler;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_downsampler {
    pub pub_0: crate::jpeglib_h::jpeg_downsampler,
    pub methods: [downsample1_ptr; 10],
}
/*
 * Initialize for a downsampling pass.
 */

unsafe extern "C" fn start_pass_downsample(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    /* no work for now */
}
/*
 * Expand a component horizontally from width input_cols to width output_cols,
 * by duplicating the rightmost samples.
 */

unsafe extern "C" fn expand_right_edge(
    mut image_data: crate::jpeglib_h::JSAMPARRAY,
    mut num_rows: libc::c_int,
    mut input_cols: crate::jmorecfg_h::JDIMENSION,
    mut output_cols: crate::jmorecfg_h::JDIMENSION,
) {
    let mut ptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>(); /* don't need GETJSAMPLE() here */
    let mut pixval: crate::jmorecfg_h::JSAMPLE = 0;
    let mut count: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut numcols: libc::c_int = ( output_cols - input_cols) as libc::c_int;
    if numcols > 0i32 {
        row = 0i32;
        while row < num_rows {
            ptr = (*image_data.offset(row as isize)).offset(input_cols as isize);
            pixval = *ptr.offset(-1i32 as isize);
            count = numcols;
            while count > 0i32 {
                let fresh0 = ptr;
                ptr = ptr.offset(1);
                *fresh0 = pixval;
                count -= 1
            }
            row += 1
        }
    };
}
/*
 * Do downsampling for a whole row group (all components).
 *
 * In this version we simply downsample each component independently.
 */

unsafe extern "C" fn sep_downsample(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut in_row_index: crate::jmorecfg_h::JDIMENSION,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut out_row_group_index: crate::jmorecfg_h::JDIMENSION,
) {
    let mut downsample: my_downsample_ptr = (*cinfo).downsample as my_downsample_ptr;
    let mut ci: libc::c_int = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_component_info>();
    let mut in_ptr: crate::jpeglib_h::JSAMPARRAY = ::std::ptr::null_mut::< crate::jpeglib_h::JSAMPROW>();
    let mut out_ptr: crate::jpeglib_h::JSAMPARRAY = ::std::ptr::null_mut::< crate::jpeglib_h::JSAMPROW>();
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        in_ptr = (*input_buf.offset(ci as isize)).offset(in_row_index as isize);
        out_ptr = (*output_buf.offset(ci as isize)).offset(
            (
            out_row_group_index * (*compptr).v_samp_factor as libc::c_uint) as isize,
        );
        Some(
            (*(*downsample).methods.as_mut_ptr().offset(ci as isize))
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo, compptr, in_ptr, out_ptr);
        ci += 1;
        compptr = compptr.offset(1)
    }
}
/*
 * Downsample pixel values of a single component.
 * One row group is processed per call.
 * This version handles arbitrary integral sampling ratios, without smoothing.
 * Note that this version is not actually used for customary sampling ratios.
 */

unsafe extern "C" fn int_downsample(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut input_data: crate::jpeglib_h::JSAMPARRAY,
    mut output_data: crate::jpeglib_h::JSAMPARRAY,
) {
    let mut inrow: libc::c_int = 0; /* outcol_h == outcol*h_expand */
    let mut outrow: libc::c_int = 0;
    let mut h_expand: libc::c_int = 0;
    let mut v_expand: libc::c_int = 0;
    let mut numpix: libc::c_int = 0;
    let mut numpix2: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut outcol: crate::jmorecfg_h::JDIMENSION = 0;
    let mut outcol_h: crate::jmorecfg_h::JDIMENSION = 0;
    let mut output_cols: crate::jmorecfg_h::JDIMENSION =  (*compptr)
        .width_in_blocks * crate::jpeglib_h::DCTSIZE as libc::c_uint;
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outvalue: crate::jpegint_h::JLONG = 0;
    h_expand = (*cinfo).max_h_samp_factor / (*compptr).h_samp_factor;
    v_expand = (*cinfo).max_v_samp_factor / (*compptr).v_samp_factor;
    numpix = h_expand * v_expand;
    numpix2 = numpix / 2i32;
    /* Expand input data enough to let all the output samples be generated
     * by the standard loop.  Special-casing padded output would be more
     * efficient.
     */
    expand_right_edge(
        input_data,
        (*cinfo).max_v_samp_factor,
        (*cinfo).image_width,
        
        output_cols * h_expand as libc::c_uint,
    );
    inrow = 0i32;
    outrow = 0i32;
    while outrow < (*compptr).v_samp_factor {
        outptr = *output_data.offset(outrow as isize);
        outcol = 0i32 as crate::jmorecfg_h::JDIMENSION;
        outcol_h = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while outcol < output_cols {
            outvalue = 0i32 as crate::jpegint_h::JLONG;
            v = 0i32;
            while v < v_expand {
                inptr = (*input_data.offset((inrow + v) as isize)).offset(outcol_h as isize);
                h = 0i32;
                while h < h_expand {
                    let fresh1 = inptr;
                    inptr = inptr.offset(1);
                    outvalue += *fresh1 as libc::c_int as crate::jpegint_h::JLONG;
                    h += 1
                }
                v += 1
            }
            let fresh2 = outptr;
            outptr = outptr.offset(1);
            *fresh2 = ((outvalue + numpix2 as libc::c_long) / numpix as libc::c_long)
                as crate::jmorecfg_h::JSAMPLE;
            outcol =  outcol + 1;
            outcol_h = (outcol_h as libc::c_uint + h_expand as libc::c_uint)
                as crate::jmorecfg_h::JDIMENSION
                as crate::jmorecfg_h::JDIMENSION
        }
        inrow += v_expand;
        outrow += 1
    }
}
/*
 * Downsample pixel values of a single component.
 * This version handles the special case of a full-size component,
 * without smoothing.
 */

unsafe extern "C" fn fullsize_downsample(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut input_data: crate::jpeglib_h::JSAMPARRAY,
    mut output_data: crate::jpeglib_h::JSAMPARRAY,
) {
    /* Copy the data */
    crate::jpegint_h::jcopy_sample_rows(
        input_data,
        0i32,
        output_data,
        0i32,
        (*cinfo).max_v_samp_factor,
        (*cinfo).image_width,
    );
    /* Edge-expand */
    expand_right_edge(
        output_data,
        (*cinfo).max_v_samp_factor,
        (*cinfo).image_width,
        
        (*compptr)
            .width_in_blocks * crate::jpeglib_h::DCTSIZE as libc::c_uint,
    );
}
/*
 * Downsample pixel values of a single component.
 * This version handles the common case of 2:1 horizontal and 1:1 vertical,
 * without smoothing.
 *
 * A note about the "bias" calculations: when rounding fractional values to
 * integer, we do not want to always round 0.5 up to the next integer.
 * If we did that, we'd introduce a noticeable bias towards larger values.
 * Instead, this code is arranged so that 0.5 will be rounded up or down at
 * alternate pixel locations (a simple ordered dither pattern).
 */

unsafe extern "C" fn h2v1_downsample(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut input_data: crate::jpeglib_h::JSAMPARRAY,
    mut output_data: crate::jpeglib_h::JSAMPARRAY,
) {
    let mut outrow: libc::c_int = 0;
    let mut outcol: crate::jmorecfg_h::JDIMENSION = 0;
    let mut output_cols: crate::jmorecfg_h::JDIMENSION =  (*compptr)
        .width_in_blocks * crate::jpeglib_h::DCTSIZE as libc::c_uint;
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut bias: libc::c_int = 0;
    /* Expand input data enough to let all the output samples be generated
     * by the standard loop.  Special-casing padded output would be more
     * efficient.
     */
    expand_right_edge(
        input_data,
        (*cinfo).max_v_samp_factor,
        (*cinfo).image_width,
        
        output_cols * 2i32 as libc::c_uint,
    ); /* bias = 0,1,0,1,... for successive samples */
    outrow = 0i32; /* 0=>1, 1=>0 */
    while outrow < (*compptr).v_samp_factor {
        outptr = *output_data.offset(outrow as isize);
        inptr = *input_data.offset(outrow as isize);
        bias = 0i32;
        outcol = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while outcol < output_cols {
            let fresh3 = outptr;
            outptr = outptr.offset(1);
            *fresh3 = (*inptr as libc::c_int + *inptr.offset(1) as libc::c_int + bias >> 1i32)
                as crate::jmorecfg_h::JSAMPLE;
            bias ^= 1i32;
            inptr = inptr.offset(2);
            outcol =  outcol + 1
        }
        outrow += 1
    }
}
/*
 * Downsample pixel values of a single component.
 * This version handles the standard case of 2:1 horizontal and 2:1 vertical,
 * without smoothing.
 */

unsafe extern "C" fn h2v2_downsample(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut input_data: crate::jpeglib_h::JSAMPARRAY,
    mut output_data: crate::jpeglib_h::JSAMPARRAY,
) {
    let mut inrow: libc::c_int = 0;
    let mut outrow: libc::c_int = 0;
    let mut outcol: crate::jmorecfg_h::JDIMENSION = 0;
    let mut output_cols: crate::jmorecfg_h::JDIMENSION =  (*compptr)
        .width_in_blocks * crate::jpeglib_h::DCTSIZE as libc::c_uint;
    let mut inptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut bias: libc::c_int = 0;
    /* Expand input data enough to let all the output samples be generated
     * by the standard loop.  Special-casing padded output would be more
     * efficient.
     */
    expand_right_edge(
        input_data,
        (*cinfo).max_v_samp_factor,
        (*cinfo).image_width,
        
        output_cols * 2i32 as libc::c_uint,
    ); /* bias = 1,2,1,2,... for successive samples */
    inrow = 0i32; /* 1=>2, 2=>1 */
    outrow = 0i32;
    while outrow < (*compptr).v_samp_factor {
        outptr = *output_data.offset(outrow as isize);
        inptr0 = *input_data.offset(inrow as isize);
        inptr1 = *input_data.offset((inrow + 1i32) as isize);
        bias = 1i32;
        outcol = 0i32 as crate::jmorecfg_h::JDIMENSION;
        while outcol < output_cols {
            let fresh4 = outptr;
            outptr = outptr.offset(1);
            *fresh4 = (*inptr0 as libc::c_int
                + *inptr0.offset(1) as libc::c_int
                + *inptr1 as libc::c_int
                + *inptr1.offset(1) as libc::c_int
                + bias
                >> 2i32) as crate::jmorecfg_h::JSAMPLE;
            bias ^= 3i32;
            inptr0 = inptr0.offset(2);
            inptr1 = inptr1.offset(2);
            outcol =  outcol + 1
        }
        inrow += 2i32;
        outrow += 1
    }
}
/*
 * Downsample pixel values of a single component.
 * This version handles the standard case of 2:1 horizontal and 2:1 vertical,
 * with smoothing.  One row of context is required.
 */

unsafe extern "C" fn h2v2_smooth_downsample(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut input_data: crate::jpeglib_h::JSAMPARRAY,
    mut output_data: crate::jpeglib_h::JSAMPARRAY,
) {
    let mut inrow: libc::c_int = 0;
    let mut outrow: libc::c_int = 0;
    let mut colctr: crate::jmorecfg_h::JDIMENSION = 0;
    let mut output_cols: crate::jmorecfg_h::JDIMENSION =  (*compptr)
        .width_in_blocks * crate::jpeglib_h::DCTSIZE as libc::c_uint;
    let mut inptr0: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut inptr1: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut above_ptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut below_ptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut membersum: crate::jpegint_h::JLONG = 0;
    let mut neighsum: crate::jpegint_h::JLONG = 0;
    let mut memberscale: crate::jpegint_h::JLONG = 0;
    let mut neighscale: crate::jpegint_h::JLONG = 0;
    /* Expand input data enough to let all the output samples be generated
     * by the standard loop.  Special-casing padded output would be more
     * efficient.
     */
    expand_right_edge(
        input_data.offset(-1),
        (*cinfo).max_v_samp_factor + 2i32,
        (*cinfo).image_width,
        
        output_cols * 2i32 as libc::c_uint,
    );
    /* We don't bother to form the individual "smoothed" input pixel values;
     * we can directly compute the output which is the average of the four
     * smoothed values.  Each of the four member pixels contributes a fraction
     * (1-8*SF) to its own smoothed image and a fraction SF to each of the three
     * other smoothed pixels, therefore a total fraction (1-5*SF)/4 to the final
     * output.  The four corner-adjacent neighbor pixels contribute a fraction
     * SF to just one smoothed pixel, or SF/4 to the final output; while the
     * eight edge-adjacent neighbors contribute SF to each of two smoothed
     * pixels, or SF/2 overall.  In order to use integer arithmetic, these
     * factors are scaled by 2^16 = 65536.
     * Also recall that SF = smoothing_factor / 1024.
     */
    memberscale = (16384i32 - (*cinfo).smoothing_factor * 80i32) as crate::jpegint_h::JLONG; /* scaled (1-5*SF)/4 */
    neighscale = ((*cinfo).smoothing_factor * 16i32) as crate::jpegint_h::JLONG; /* scaled SF/4 */
    inrow = 0i32;
    outrow = 0i32;
    while outrow < (*compptr).v_samp_factor {
        outptr = *output_data.offset(outrow as isize);
        inptr0 = *input_data.offset(inrow as isize);
        inptr1 = *input_data.offset((inrow + 1i32) as isize);
        above_ptr = *input_data.offset((inrow - 1i32) as isize);
        below_ptr = *input_data.offset((inrow + 2i32) as isize);
        /* Special case for first column: pretend column -1 is same as column 0 */
        membersum = (*inptr0 as libc::c_int
            + *inptr0.offset(1) as libc::c_int
            + *inptr1 as libc::c_int
            + *inptr1.offset(1) as libc::c_int) as crate::jpegint_h::JLONG;
        neighsum = (*above_ptr as libc::c_int
            + *above_ptr.offset(1) as libc::c_int
            + *below_ptr as libc::c_int
            + *below_ptr.offset(1) as libc::c_int
            + *inptr0 as libc::c_int
            + *inptr0.offset(2) as libc::c_int
            + *inptr1 as libc::c_int
            + *inptr1.offset(2) as libc::c_int) as crate::jpegint_h::JLONG;
        neighsum += neighsum;
        neighsum += (*above_ptr as libc::c_int
            + *above_ptr.offset(2) as libc::c_int
            + *below_ptr as libc::c_int
            + *below_ptr.offset(2) as libc::c_int) as libc::c_long;
        membersum = membersum * memberscale + neighsum * neighscale;
        let fresh5 = outptr;
        outptr = outptr.offset(1);
        *fresh5 = (membersum + 32768i32 as libc::c_long >> 16i32) as crate::jmorecfg_h::JSAMPLE;
        inptr0 = inptr0.offset(2);
        inptr1 = inptr1.offset(2);
        above_ptr = above_ptr.offset(2);
        below_ptr = below_ptr.offset(2);
        colctr =  output_cols - 2i32 as libc::c_uint;
        while colctr > 0i32 as libc::c_uint {
            /* sum of pixels directly mapped to this output element */
            membersum = (*inptr0 as libc::c_int
                + *inptr0.offset(1) as libc::c_int
                + *inptr1 as libc::c_int
                + *inptr1.offset(1) as libc::c_int)
                as crate::jpegint_h::JLONG;
            /* sum of edge-neighbor pixels */
            neighsum = (*above_ptr as libc::c_int
                + *above_ptr.offset(1) as libc::c_int
                + *below_ptr as libc::c_int
                + *below_ptr.offset(1) as libc::c_int
                + *inptr0.offset(-1i32 as isize) as libc::c_int
                + *inptr0.offset(2) as libc::c_int
                + *inptr1.offset(-1i32 as isize) as libc::c_int
                + *inptr1.offset(2) as libc::c_int)
                as crate::jpegint_h::JLONG;
            /* The edge-neighbors count twice as much as corner-neighbors */
            neighsum += neighsum;
            /* Add in the corner-neighbors */
            neighsum += (*above_ptr.offset(-1i32 as isize) as libc::c_int
                + *above_ptr.offset(2) as libc::c_int
                + *below_ptr.offset(-1i32 as isize) as libc::c_int
                + *below_ptr.offset(2) as libc::c_int) as libc::c_long;
            /* form final output scaled up by 2^16 */
            membersum = membersum * memberscale + neighsum * neighscale;
            /* round, descale and output it */
            let fresh6 = outptr;
            outptr = outptr.offset(1);
            *fresh6 = (membersum + 32768i32 as libc::c_long >> 16i32) as crate::jmorecfg_h::JSAMPLE;
            inptr0 = inptr0.offset(2);
            inptr1 = inptr1.offset(2);
            above_ptr = above_ptr.offset(2);
            below_ptr = below_ptr.offset(2);
            colctr =  colctr - 1
        }
        /* Special case for last column */
        membersum = (*inptr0 as libc::c_int
            + *inptr0.offset(1) as libc::c_int
            + *inptr1 as libc::c_int
            + *inptr1.offset(1) as libc::c_int) as crate::jpegint_h::JLONG;
        neighsum = (*above_ptr as libc::c_int
            + *above_ptr.offset(1) as libc::c_int
            + *below_ptr as libc::c_int
            + *below_ptr.offset(1) as libc::c_int
            + *inptr0.offset(-1i32 as isize) as libc::c_int
            + *inptr0.offset(1) as libc::c_int
            + *inptr1.offset(-1i32 as isize) as libc::c_int
            + *inptr1.offset(1) as libc::c_int) as crate::jpegint_h::JLONG;
        neighsum += neighsum;
        neighsum += (*above_ptr.offset(-1i32 as isize) as libc::c_int
            + *above_ptr.offset(1) as libc::c_int
            + *below_ptr.offset(-1i32 as isize) as libc::c_int
            + *below_ptr.offset(1) as libc::c_int) as libc::c_long;
        membersum = membersum * memberscale + neighsum * neighscale;
        *outptr = (membersum + 32768i32 as libc::c_long >> 16i32) as crate::jmorecfg_h::JSAMPLE;
        inrow += 2i32;
        outrow += 1
    }
}
/*
 * Downsample pixel values of a single component.
 * This version handles the special case of a full-size component,
 * with smoothing.  One row of context is required.
 */

unsafe extern "C" fn fullsize_smooth_downsample(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut compptr: *mut crate::jpeglib_h::jpeg_component_info,
    mut input_data: crate::jpeglib_h::JSAMPARRAY,
    mut output_data: crate::jpeglib_h::JSAMPARRAY,
) {
    let mut outrow: libc::c_int = 0;
    let mut colctr: crate::jmorecfg_h::JDIMENSION = 0;
    let mut output_cols: crate::jmorecfg_h::JDIMENSION =  (*compptr)
        .width_in_blocks * crate::jpeglib_h::DCTSIZE as libc::c_uint;
    let mut inptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut above_ptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut below_ptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut outptr: crate::jpeglib_h::JSAMPROW = ::std::ptr::null_mut::< crate::jmorecfg_h::JSAMPLE>();
    let mut membersum: crate::jpegint_h::JLONG = 0;
    let mut neighsum: crate::jpegint_h::JLONG = 0;
    let mut memberscale: crate::jpegint_h::JLONG = 0;
    let mut neighscale: crate::jpegint_h::JLONG = 0;
    let mut colsum: libc::c_int = 0;
    let mut lastcolsum: libc::c_int = 0;
    let mut nextcolsum: libc::c_int = 0;
    /* Expand input data enough to let all the output samples be generated
     * by the standard loop.  Special-casing padded output would be more
     * efficient.
     */
    expand_right_edge(
        input_data.offset(-1),
        (*cinfo).max_v_samp_factor + 2i32,
        (*cinfo).image_width,
        output_cols,
    );
    /* Each of the eight neighbor pixels contributes a fraction SF to the
     * smoothed pixel, while the main pixel contributes (1-8*SF).  In order
     * to use integer arithmetic, these factors are multiplied by 2^16 = 65536.
     * Also recall that SF = smoothing_factor / 1024.
     */
    memberscale = 65536i64 - (*cinfo).smoothing_factor as libc::c_long * 512i64; /* scaled 1-8*SF */
    neighscale = ((*cinfo).smoothing_factor * 64i32) as crate::jpegint_h::JLONG; /* scaled SF */
    outrow = 0i32;
    while outrow < (*compptr).v_samp_factor {
        outptr = *output_data.offset(outrow as isize);
        inptr = *input_data.offset(outrow as isize);
        above_ptr = *input_data.offset((outrow - 1i32) as isize);
        below_ptr = *input_data.offset((outrow + 1i32) as isize);
        /* Special case for first column */
        let fresh7 = above_ptr;
        above_ptr = above_ptr.offset(1);
        let fresh8 = below_ptr;
        below_ptr = below_ptr.offset(1);
        colsum = *fresh7 as libc::c_int + *fresh8 as libc::c_int + *inptr as libc::c_int;
        let fresh9 = inptr;
        inptr = inptr.offset(1);
        membersum = *fresh9 as libc::c_int as crate::jpegint_h::JLONG;
        nextcolsum = *above_ptr as libc::c_int + *below_ptr as libc::c_int + *inptr as libc::c_int;
        neighsum = colsum as libc::c_long
            + (colsum as libc::c_long - membersum)
            + nextcolsum as libc::c_long;
        membersum = membersum * memberscale + neighsum * neighscale;
        let fresh10 = outptr;
        outptr = outptr.offset(1);
        *fresh10 = (membersum + 32768i32 as libc::c_long >> 16i32) as crate::jmorecfg_h::JSAMPLE;
        lastcolsum = colsum;
        colsum = nextcolsum;
        colctr =  output_cols - 2i32 as libc::c_uint;
        while colctr > 0i32 as libc::c_uint {
            let fresh11 = inptr;
            inptr = inptr.offset(1);
            membersum = *fresh11 as libc::c_int as crate::jpegint_h::JLONG;
            above_ptr = above_ptr.offset(1);
            below_ptr = below_ptr.offset(1);
            nextcolsum =
                *above_ptr as libc::c_int + *below_ptr as libc::c_int + *inptr as libc::c_int;
            neighsum = lastcolsum as libc::c_long
                + (colsum as libc::c_long - membersum)
                + nextcolsum as libc::c_long;
            membersum = membersum * memberscale + neighsum * neighscale;
            let fresh12 = outptr;
            outptr = outptr.offset(1);
            *fresh12 =
                (membersum + 32768i32 as libc::c_long >> 16i32) as crate::jmorecfg_h::JSAMPLE;
            lastcolsum = colsum;
            colsum = nextcolsum;
            colctr =  colctr - 1
        }
        /* Special case for last column */
        membersum = *inptr as libc::c_int as crate::jpegint_h::JLONG;
        neighsum = lastcolsum as libc::c_long
            + (colsum as libc::c_long - membersum)
            + colsum as libc::c_long;
        membersum = membersum * memberscale + neighsum * neighscale;
        *outptr = (membersum + 32768i32 as libc::c_long >> 16i32) as crate::jmorecfg_h::JSAMPLE;
        outrow += 1
    }
}
/* It is useful to allow each component to have a separate IDCT method. */
/* Upsampling (note that upsampler must also call color converter) */
/* TRUE if need rows above & below */
/* Colorspace conversion */
/* Color quantization or color precision reduction */
/* Miscellaneous useful macros */
/* We assume that right shift corresponds to signed division by 2 with
 * rounding towards minus infinity.  This is correct for typical "arithmetic
 * shift" instructions that shift in copies of the sign bit.  But some
 * C compilers implement >> with an unsigned shift.  For these machines you
 * must define RIGHT_SHIFT_IS_UNSIGNED.
 * RIGHT_SHIFT provides a proper signed right shift of a JLONG quantity.
 * It is only applied with constant shift counts.  SHIFT_TEMPS must be
 * included in the variables of any routine using RIGHT_SHIFT.
 */
/* Compression module initialization routines */
/* INPUT_SMOOTHING_SUPPORTED */
/*
 * Module initialization routine for downsampling.
 * Note that we must select a routine for each component.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_downsampler(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    let mut downsample: my_downsample_ptr = ::std::ptr::null_mut::< my_downsampler>();
    let mut ci: libc::c_int = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_component_info>();
    let mut smoothok: crate::jmorecfg_h::boolean = crate::jmorecfg_h::TRUE;
    downsample = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<my_downsampler>() as libc::c_ulong,
    ) as my_downsample_ptr;
    (*cinfo).downsample = downsample as *mut crate::jpeglib_h::jpeg_downsampler;
    (*downsample).pub_0.start_pass = Some(
        start_pass_downsample as unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> (),
    );
    (*downsample).pub_0.downsample = Some(
        sep_downsample
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_compress_ptr,
                _: crate::jpeglib_h::JSAMPIMAGE,
                _: crate::jmorecfg_h::JDIMENSION,
                _: crate::jpeglib_h::JSAMPIMAGE,
                _: crate::jmorecfg_h::JDIMENSION,
            ) -> (),
    );
    (*downsample).pub_0.need_context_rows = crate::jmorecfg_h::FALSE;
    if (*cinfo).CCIR601_sampling != 0 {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_CCIR601_NOTIMPL as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Verify we can handle the sampling factors, and set up method pointers */
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        if (*compptr).h_samp_factor == (*cinfo).max_h_samp_factor
            && (*compptr).v_samp_factor == (*cinfo).max_v_samp_factor
        {
            if (*cinfo).smoothing_factor != 0 {
                (*downsample).methods[ci as usize] = Some(
                    fullsize_smooth_downsample
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPARRAY,
                        ) -> (),
                );
                (*downsample).pub_0.need_context_rows = crate::jmorecfg_h::TRUE
            } else {
                (*downsample).methods[ci as usize] = Some(
                    fullsize_downsample
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPARRAY,
                        ) -> (),
                )
            }
        } else if (*compptr).h_samp_factor * 2i32 == (*cinfo).max_h_samp_factor
            && (*compptr).v_samp_factor == (*cinfo).max_v_samp_factor
        {
            smoothok = crate::jmorecfg_h::FALSE;
            if crate::src::simd::x86_64::jsimd::jsimd_can_h2v1_downsample() != 0 {
                (*downsample).methods[ci as usize] = Some(
                    crate::src::simd::x86_64::jsimd::jsimd_h2v1_downsample
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPARRAY,
                        ) -> (),
                )
            } else {
                (*downsample).methods[ci as usize] = Some(
                    h2v1_downsample
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPARRAY,
                        ) -> (),
                )
            }
        } else if (*compptr).h_samp_factor * 2i32 == (*cinfo).max_h_samp_factor
            && (*compptr).v_samp_factor * 2i32 == (*cinfo).max_v_samp_factor
        {
            if (*cinfo).smoothing_factor != 0 {
                (*downsample).methods[ci as usize] = Some(
                    h2v2_smooth_downsample
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPARRAY,
                        ) -> (),
                );
                (*downsample).pub_0.need_context_rows = crate::jmorecfg_h::TRUE
            } else if crate::src::simd::x86_64::jsimd::jsimd_can_h2v2_downsample() != 0 {
                (*downsample).methods[ci as usize] = Some(
                    crate::src::simd::x86_64::jsimd::jsimd_h2v2_downsample
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPARRAY,
                        ) -> (),
                )
            } else {
                (*downsample).methods[ci as usize] = Some(
                    h2v2_downsample
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: *mut crate::jpeglib_h::jpeg_component_info,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPARRAY,
                        ) -> (),
                )
            }
        } else if (*cinfo).max_h_samp_factor % (*compptr).h_samp_factor == 0i32
            && (*cinfo).max_v_samp_factor % (*compptr).v_samp_factor == 0i32
        {
            smoothok = crate::jmorecfg_h::FALSE;
            (*downsample).methods[ci as usize] = Some(
                int_downsample
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_compress_ptr,
                        _: *mut crate::jpeglib_h::jpeg_component_info,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPARRAY,
                    ) -> (),
            )
        } else {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_FRACT_SAMPLE_NOTIMPL as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        ci += 1;
        compptr = compptr.offset(1)
    }
    if (*cinfo).smoothing_factor != 0 && smoothok == 0 {
        (*(*cinfo).err).msg_code = crate::src::jerror::JTRC_SMOOTH_NOTIMPL as libc::c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr, 0i32);
    };
}
