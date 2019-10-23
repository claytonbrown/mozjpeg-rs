



































































































































































































































use crate::stdlib::{fprintf, free, memcpy, stderr};use libc::{c_uchar, c_ulong, c_char, c_long, c_uint, c_int, c_void, self};use std::ffi::CStr;pub use crate::jpegint_h::{jdiv_round_up, jpeg_mem_dest_internal,
                           JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT,
                           JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE, J_BUF_MODE};pub use crate::jpeglib_h::{jpeg_comp_master, jpeg_destination_mgr,
                           C2RustUnnamed_2, j_common_ptr, j_compress_ptr,
                           jpeg_c_coef_controller, jpeg_c_main_controller,
                           jpeg_c_prep_controller, jpeg_color_converter,
                           jpeg_common_struct, jpeg_component_info,
                           jpeg_compress_struct, jpeg_downsampler,
                           jpeg_entropy_encoder, jpeg_error_mgr,
                           jpeg_forward_dct, jpeg_marker_writer,
                           jpeg_memory_mgr, jpeg_progress_mgr, jpeg_scan_info,
                           jvirt_barray_control, jvirt_barray_ptr,
                           jvirt_sarray_control, jvirt_sarray_ptr, JCS_YCbCr,
                           C_MAX_BLOCKS_IN_MCU, DCTSIZE, DCTSIZE2, JBLOCK,
                           JBLOCKARRAY, JBLOCKROW, JCS_CMYK, JCS_EXT_ABGR,
                           JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA,
                           JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA,
                           JCS_EXT_RGBX, JCS_EXT_XBGR, JCS_EXT_XRGB,
                           JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN,
                           JCS_YCCK, JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW,
                           JHUFF_TBL, JPOOL_IMAGE, JQUANT_TBL, JSAMPARRAY,
                           JSAMPIMAGE, JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD,
                           MAX_COMPS_IN_SCAN, MAX_SAMP_FACTOR,
                           NUM_QUANT_TBLS};pub use crate::jmorecfg_h::{boolean, FALSE, JCOEF, JDIMENSION, JOCTET,
                            JPEG_MAX_DIMENSION, JSAMPLE, MAX_COMPONENTS, TRUE,
                            UINT16, UINT8};pub use crate::stddef_h::{size_t, NULL};pub use crate::stdlib::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data,
                        __off64_t, __off_t, FILE, _IO_FILE};pub use crate::jconfig_h::BITS_IN_JSAMPLE;pub use super::jerror::{C2RustUnnamed_3, JERR_ARITH_NOTIMPL,
                        JERR_BAD_ALIGN_TYPE, JERR_BAD_ALLOC_CHUNK,
                        JERR_BAD_BUFFER_MODE, JERR_BAD_COMPONENT_ID,
                        JERR_BAD_CROP_SPEC, JERR_BAD_DCTSIZE,
                        JERR_BAD_DCT_COEF, JERR_BAD_HUFF_TABLE,
                        JERR_BAD_IN_COLORSPACE, JERR_BAD_J_COLORSPACE,
                        JERR_BAD_LENGTH, JERR_BAD_LIB_VERSION,
                        JERR_BAD_MCU_SIZE, JERR_BAD_PARAM,
                        JERR_BAD_PARAM_VALUE, JERR_BAD_POOL_ID,
                        JERR_BAD_PRECISION, JERR_BAD_PROGRESSION,
                        JERR_BAD_PROG_SCRIPT, JERR_BAD_SAMPLING,
                        JERR_BAD_SCAN_SCRIPT, JERR_BAD_STATE,
                        JERR_BAD_STRUCT_SIZE, JERR_BAD_VIRTUAL_ACCESS,
                        JERR_BUFFER_SIZE, JERR_CANT_SUSPEND,
                        JERR_CCIR601_NOTIMPL, JERR_COMPONENT_COUNT,
                        JERR_CONVERSION_NOTIMPL, JERR_DAC_INDEX,
                        JERR_DAC_VALUE, JERR_DHT_INDEX, JERR_DQT_INDEX,
                        JERR_EMPTY_IMAGE, JERR_EMS_READ, JERR_EMS_WRITE,
                        JERR_EOI_EXPECTED, JERR_FILE_READ, JERR_FILE_WRITE,
                        JERR_FRACT_SAMPLE_NOTIMPL, JERR_HUFF_CLEN_OVERFLOW,
                        JERR_HUFF_MISSING_CODE, JERR_IMAGE_TOO_BIG,
                        JERR_INPUT_EMPTY, JERR_INPUT_EOF,
                        JERR_MISMATCHED_QUANT_TABLE, JERR_MISSING_DATA,
                        JERR_MODE_CHANGE, JERR_NOTIMPL, JERR_NOT_COMPILED,
                        JERR_NO_BACKING_STORE, JERR_NO_HUFF_TABLE,
                        JERR_NO_IMAGE, JERR_NO_QUANT_TABLE, JERR_NO_SOI,
                        JERR_OUT_OF_MEMORY, JERR_QUANT_COMPONENTS,
                        JERR_QUANT_FEW_COLORS, JERR_QUANT_MANY_COLORS,
                        JERR_SOF_DUPLICATE, JERR_SOF_NO_SOS,
                        JERR_SOF_UNSUPPORTED, JERR_SOI_DUPLICATE,
                        JERR_SOS_NO_SOF, JERR_TFILE_CREATE, JERR_TFILE_READ,
                        JERR_TFILE_SEEK, JERR_TFILE_WRITE,
                        JERR_TOO_LITTLE_DATA, JERR_UNKNOWN_MARKER,
                        JERR_UNSUPPORTED_SUSPEND, JERR_VIRTUAL_BUG,
                        JERR_WIDTH_OVERFLOW, JERR_XMS_READ, JERR_XMS_WRITE,
                        JMSG_COPYRIGHT, JMSG_LASTMSGCODE, JMSG_NOMESSAGE,
                        JMSG_VERSION, JTRC_16BIT_TABLES, JTRC_ADOBE,
                        JTRC_APP0, JTRC_APP14, JTRC_DAC, JTRC_DHT, JTRC_DQT,
                        JTRC_DRI, JTRC_EMS_CLOSE, JTRC_EMS_OPEN, JTRC_EOI,
                        JTRC_HUFFBITS, JTRC_JFIF, JTRC_JFIF_BADTHUMBNAILSIZE,
                        JTRC_JFIF_EXTENSION, JTRC_JFIF_THUMBNAIL,
                        JTRC_MISC_MARKER, JTRC_PARMLESS_MARKER,
                        JTRC_QUANTVALS, JTRC_QUANT_3_NCOLORS,
                        JTRC_QUANT_NCOLORS, JTRC_QUANT_SELECTED,
                        JTRC_RECOVERY_ACTION, JTRC_RST, JTRC_SMOOTH_NOTIMPL,
                        JTRC_SOF, JTRC_SOF_COMPONENT, JTRC_SOI, JTRC_SOS,
                        JTRC_SOS_COMPONENT, JTRC_SOS_PARAMS, JTRC_TFILE_CLOSE,
                        JTRC_TFILE_OPEN, JTRC_THUMB_JPEG, JTRC_THUMB_PALETTE,
                        JTRC_THUMB_RGB, JTRC_UNKNOWN_IDS, JTRC_XMS_CLOSE,
                        JTRC_XMS_OPEN, JWRN_ADOBE_XFORM, JWRN_BOGUS_ICC,
                        JWRN_BOGUS_PROGRESSION, JWRN_EXTRANEOUS_DATA,
                        JWRN_HIT_MARKER, JWRN_HUFF_BAD_CODE, JWRN_JFIF_MAJOR,
                        JWRN_JPEG_EOF, JWRN_MUST_RESYNC, JWRN_NOT_SEQUENTIAL,
                        JWRN_TOO_MUCH_DATA};
// =============== BEGIN jcmaster_h ================
pub type my_master_ptr = *mut my_comp_master;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_comp_master {
    pub pub_0: jpeg_comp_master,
    pub pass_type: c_pass_type,
    pub pass_number: c_int,
    pub total_passes: c_int,
    pub scan_number: c_int,
    pub pass_number_scan_opt_base: c_int,
    pub scan_buffer: [*mut c_uchar; 64],
    pub scan_size: [c_ulong; 64],
    pub actual_Al: [c_int; 64],
    pub best_cost: c_ulong,
    pub best_freq_split_idx_luma: c_int,
    pub best_freq_split_idx_chroma: c_int,
    pub best_Al_luma: c_int,
    pub best_Al_chroma: c_int,
    pub interleave_chroma_dc: boolean,
    pub saved_dest: *mut jpeg_destination_mgr,
    pub jpeg_version: *const c_char,
}

pub type c_pass_type = c_uint;

pub const trellis_pass: c_pass_type = 3;

pub const output_pass: c_pass_type = 2;

pub const huff_opt_pass: c_pass_type = 1;

pub const main_pass: c_pass_type = 0;
/*
 * jcmaster.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1991-1997, Thomas G. Lane.
 * Modified 2003-2010 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2010, 2016, 2018, D. R. Commander.
 * mozjpeg Modifications:
 * Copyright (C) 2014, Mozilla Corporation.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains master control logic for the JPEG compressor.
 * These routines are concerned with parameter validation, initial setup,
 * and inter-pass control (determining the number of passes and the work
 * to be done in each pass).
 */
/*
 * Support routines that do various essential calculations.
 */

unsafe extern "C" fn initial_setup(
    mut cinfo: j_compress_ptr,
    mut _transcode_only: boolean,
)
/* Do computations that are needed before master selection phase */
{
    
    
    
        
    /* Sanity check on image dimensions */
    if (*cinfo).image_height <= 0u32
        || (*cinfo).image_width <= 0u32
        || (*cinfo).num_components <= 0i32
        || (*cinfo).input_components <= 0i32
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_EMPTY_IMAGE as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Make sure image isn't bigger than I can handle */
    if (*cinfo).image_height as c_long > JPEG_MAX_DIMENSION
        || (*cinfo).image_width as c_long > JPEG_MAX_DIMENSION
    {
        (*(*cinfo).err).msg_code = super::jerror::JERR_IMAGE_TOO_BIG as c_int;
        (*(*cinfo).err).msg_parm.i[0] = 65500i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    
     let mut samplesperrow:   c_long =
    
        (*cinfo).image_width as c_long * (*cinfo).input_components as c_long; let mut jd_samplesperrow:   JDIMENSION =
     samplesperrow as JDIMENSION;
    if jd_samplesperrow as c_long != samplesperrow {
        (*(*cinfo).err).msg_code = super::jerror::JERR_WIDTH_OVERFLOW as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* For now, precision must match compiled-in value... */
    if (*cinfo).data_precision != BITS_IN_JSAMPLE {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_PRECISION as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).data_precision;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Check that number of components won't exceed internal array sizes */
    if (*cinfo).num_components > MAX_COMPONENTS {
        (*(*cinfo).err).msg_code = super::jerror::JERR_COMPONENT_COUNT as c_int;
        (*(*cinfo).err).msg_parm.i[0] = (*cinfo).num_components;
        (*(*cinfo).err).msg_parm.i[1] = 10i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* Compute maximum sampling factors; check factor validity */
    (*cinfo).max_h_samp_factor = 1i32;
    (*cinfo).max_v_samp_factor = 1i32;
    
     let mut ci:   c_int =  0i32; let mut compptr:   *mut jpeg_component_info =
     (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        if (*compptr).h_samp_factor <= 0i32
            || (*compptr).h_samp_factor > MAX_SAMP_FACTOR
            || (*compptr).v_samp_factor <= 0i32
            || (*compptr).v_samp_factor > MAX_SAMP_FACTOR
        {
            (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_SAMPLING as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        (*cinfo).max_h_samp_factor = if (*cinfo).max_h_samp_factor > (*compptr).h_samp_factor {
            (*cinfo).max_h_samp_factor
        } else {
            (*compptr).h_samp_factor
        };
        (*cinfo).max_v_samp_factor = if (*cinfo).max_v_samp_factor > (*compptr).v_samp_factor {
            (*cinfo).max_v_samp_factor
        } else {
            (*compptr).v_samp_factor
        };
        ci += 1;
        compptr = compptr.offset(1)
    }
    /* Compute dimensions of components */
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Fill in the correct component_index value; don't rely on application */
        (*compptr).component_index = ci;
        /* For compression, we never do DCT scaling. */
        (*compptr).DCT_scaled_size = DCTSIZE;
        /* Size in DCT blocks */
        (*compptr).width_in_blocks = jdiv_round_up(
            (*cinfo).image_width as c_long * (*compptr).h_samp_factor as c_long,
            ((*cinfo).max_h_samp_factor * DCTSIZE) as c_long,
        ) as JDIMENSION;
        (*compptr).height_in_blocks = jdiv_round_up(
            (*cinfo).image_height as c_long * (*compptr).v_samp_factor as c_long,
            ((*cinfo).max_v_samp_factor * DCTSIZE) as c_long,
        ) as JDIMENSION;
        /* Size in samples */
        (*compptr).downsampled_width = jdiv_round_up(
            (*cinfo).image_width as c_long * (*compptr).h_samp_factor as c_long,
            (*cinfo).max_h_samp_factor as c_long,
        ) as JDIMENSION;
        (*compptr).downsampled_height = jdiv_round_up(
            (*cinfo).image_height as c_long * (*compptr).v_samp_factor as c_long,
            (*cinfo).max_v_samp_factor as c_long,
        ) as JDIMENSION;
        /* Mark component needed (this flag isn't actually used for compression) */
        (*compptr).component_needed = TRUE;
        ci += 1;
        compptr = compptr.offset(1)
    }
    /* Compute number of fully interleaved MCU rows (number of times that
     * main controller will call coefficient controller).
     */
    (*cinfo).total_iMCU_rows = jdiv_round_up(
        (*cinfo).image_height as c_long,
        ((*cinfo).max_v_samp_factor * DCTSIZE) as c_long,
    ) as JDIMENSION;
}

unsafe extern "C" fn validate_script(mut cinfo: j_compress_ptr)
/* Verify that the scan script in cinfo->scan_info[] is valid; also
 * determine whether it uses progressive JPEG, and set cinfo->progressive_mode.
 */
{
    
    
    
    
    
    
    
    
    
    
    
    
       let mut ci:  c_int =  0; let mut coefi:  c_int =  0; let mut component_sent:  [boolean; 10] =  [0; 10]; let mut last_bitpos_ptr:  *mut c_int =
     ::std::ptr::null_mut::< c_int>(); let mut last_bitpos:  [[c_int; 64]; 10] =  [[0; 64]; 10];
    /* -1 until that coefficient has been seen; then last Al for it */
    if (*(*cinfo).master).optimize_scans != 0 {
        (*cinfo).progressive_mode = TRUE;
        /* When we optimize scans, there is redundancy in the scan list
         * and this function will fail. Therefore skip all this checking
         */
        return;
    }
    if (*cinfo).num_scans <= 0i32 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_SCAN_SCRIPT as c_int;
        (*(*cinfo).err).msg_parm.i[0] = 0i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* For sequential JPEG, all scans must have Ss=0, Se=DCTSIZE2-1;
     * for progressive JPEG, no scan can have this.
     */
     let mut scanptr:   *const jpeg_scan_info =  (*cinfo).scan_info;
    if (*scanptr).Ss != 0i32 || (*scanptr).Se != DCTSIZE2 - 1i32 {
        (*cinfo).progressive_mode = TRUE;
        last_bitpos_ptr =
            &mut *(*last_bitpos.as_mut_ptr().offset(0)).as_mut_ptr().offset(0) as *mut c_int;
        ci = 0i32;
        while ci < (*cinfo).num_components {
            coefi = 0i32;
            while coefi < DCTSIZE2 {
                let fresh0 = last_bitpos_ptr;
                last_bitpos_ptr = last_bitpos_ptr.offset(1);
                *fresh0 = -1i32;
                coefi += 1
            }
            ci += 1
        }
    } else {
        (*cinfo).progressive_mode = FALSE;
        ci = 0i32;
        while ci < (*cinfo).num_components {
            component_sent[ci as usize] = FALSE;
            ci += 1
        }
    }
     let mut scanno:   c_int =  1i32;
    while scanno <= (*cinfo).num_scans {
         let mut thisi:  c_int =  0;     let mut ncomps:   c_int =  (*scanptr).comps_in_scan;
        if ncomps <= 0i32 || ncomps > MAX_COMPS_IN_SCAN {
            (*(*cinfo).err).msg_code = super::jerror::JERR_COMPONENT_COUNT as c_int;
            (*(*cinfo).err).msg_parm.i[0] = ncomps;
            (*(*cinfo).err).msg_parm.i[1] = 4i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        ci = 0i32;
        while ci < ncomps {
            thisi = (*scanptr).component_index[ci as usize];
            if thisi < 0i32 || thisi >= (*cinfo).num_components {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_SCAN_SCRIPT as c_int;
                (*(*cinfo).err).msg_parm.i[0] = scanno;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            /* Components must appear in SOF order within each scan */
            if ci > 0i32 && thisi <= (*scanptr).component_index[(ci - 1i32) as usize] {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_SCAN_SCRIPT as c_int;
                (*(*cinfo).err).msg_parm.i[0] = scanno;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            ci += 1
        }
        
        
        
         let mut Ss:   c_int =  (*scanptr).Ss; let mut Se:   c_int =  (*scanptr).Se; let mut Ah:   c_int =  (*scanptr).Ah; let mut Al:   c_int =  (*scanptr).Al;
        if (*cinfo).progressive_mode != 0 {
            /* Rec. ITU-T T.81 | ISO/IEC 10918-1 simply gives the ranges 0..13 for Ah
             * and Al, but that seems wrong: the upper bound ought to depend on data
             * precision.  Perhaps they really meant 0..N+1 for N-bit precision.
             * Here we allow 0..10 for 8-bit data; Al larger than 10 results in
             * out-of-range reconstructed DC values during the first DC scan,
             * which might cause problems for some decoders.
             */
            if Ss < 0i32
                || Ss >= DCTSIZE2
                || Se < Ss
                || Se >= DCTSIZE2
                || Ah < 0i32
                || Ah > MAX_AH_AL
                || Al < 0i32
                || Al > MAX_AH_AL
            {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_PROG_SCRIPT as c_int;
                (*(*cinfo).err).msg_parm.i[0] = scanno;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            if Ss == 0i32 {
                if Se != 0i32 {
                    /* DC and AC together not OK */
                    (*(*cinfo).err).msg_code =
                        super::jerror::JERR_BAD_PROG_SCRIPT as c_int;
                    (*(*cinfo).err).msg_parm.i[0] = scanno;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr
                    );
                }
            } else if ncomps != 1i32 {
                /* AC scans must be for only one component */
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_PROG_SCRIPT as c_int;
                (*(*cinfo).err).msg_parm.i[0] = scanno;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            ci = 0i32;
            while ci < ncomps {
                last_bitpos_ptr = &mut *(*last_bitpos
                    .as_mut_ptr()
                    .offset(*(*scanptr).component_index.as_ptr().offset(ci as isize) as isize))
                .as_mut_ptr()
                .offset(0) as *mut c_int;
                if Ss != 0i32 && *last_bitpos_ptr.offset(0) < 0i32 {
                    /* AC without prior DC scan */
                    (*(*cinfo).err).msg_code =
                        super::jerror::JERR_BAD_PROG_SCRIPT as c_int;
                    (*(*cinfo).err).msg_parm.i[0] = scanno;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr
                    );
                }
                coefi = Ss;
                while coefi <= Se {
                    if *last_bitpos_ptr.offset(coefi as isize) < 0i32 {
                        /* first scan of this coefficient */
                        if Ah != 0i32 {
                            (*(*cinfo).err).msg_code =
                                super::jerror::JERR_BAD_PROG_SCRIPT as c_int;
                            (*(*cinfo).err).msg_parm.i[0] = scanno;
                            Some(
                                (*(*cinfo).err)
                                    .error_exit
                                    .expect("non-null function pointer"),
                            )
                            .expect("non-null function pointer")(
                                cinfo as j_common_ptr,
                            );
                        }
                    } else if Ah != *last_bitpos_ptr.offset(coefi as isize) || Al != Ah - 1i32 {
                        (*(*cinfo).err).msg_code =
                            super::jerror::JERR_BAD_PROG_SCRIPT as c_int;
                        (*(*cinfo).err).msg_parm.i[0] = scanno;
                        Some(
                            (*(*cinfo).err)
                                .error_exit
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as j_common_ptr,
                        );
                    }
                    *last_bitpos_ptr.offset(coefi as isize) = Al;
                    coefi += 1
                }
                ci += 1
            }
        } else {
            /* not first scan */
            /* For sequential JPEG, all progression parameters must be these: */
            if Ss != 0i32 || Se != DCTSIZE2 - 1i32 || Ah != 0i32 || Al != 0i32 {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_PROG_SCRIPT as c_int;
                (*(*cinfo).err).msg_parm.i[0] = scanno;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            /* Make sure components are not sent twice */
            ci = 0i32;
            while ci < ncomps {
                thisi = (*scanptr).component_index[ci as usize];
                if component_sent[thisi as usize] != 0 {
                    (*(*cinfo).err).msg_code =
                        super::jerror::JERR_BAD_SCAN_SCRIPT as c_int;
                    (*(*cinfo).err).msg_parm.i[0] = scanno;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as j_common_ptr
                    );
                }
                component_sent[thisi as usize] = TRUE;
                ci += 1
            }
        }
        scanptr = scanptr.offset(1);
        scanno += 1
    }
    /* Now verify that everything got sent. */
    if (*cinfo).progressive_mode != 0 {
        /* For progressive mode, we only check that at least some DC data
         * got sent for each component; the spec does not require that all bits
         * of all coefficients be transmitted.  Would it be wiser to enforce
         * transmission of all coefficient bits??
         */
        ci = 0i32;
        while ci < (*cinfo).num_components {
            if last_bitpos[ci as usize][0] < 0i32 {
                (*(*cinfo).err).msg_code = super::jerror::JERR_MISSING_DATA as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            ci += 1
        }
    } else {
        ci = 0i32;
        while ci < (*cinfo).num_components {
            if component_sent[ci as usize] == 0 {
                (*(*cinfo).err).msg_code = super::jerror::JERR_MISSING_DATA as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            ci += 1
        }
    };
}

pub const MAX_AH_AL: c_int = 10i32;
/* C_MULTISCAN_FILES_SUPPORTED */

unsafe extern "C" fn select_scan_parameters(mut cinfo: j_compress_ptr)
/* Set up the scan parameters for the current scan */
{
     let mut ci:  c_int =  0;
    let mut master: my_master_ptr =
        (*cinfo).master as my_master_ptr;
    if (*master).pass_number < (*master).pass_number_scan_opt_base {
        (*cinfo).comps_in_scan = 1i32;
        if (*(*cinfo).master).use_scans_in_trellis != 0 {
            (*cinfo).cur_comp_info[0] = &mut *(*cinfo).comp_info.offset(
                ((*master).pass_number / (4i32 * (*(*cinfo).master).trellis_num_loops)) as isize,
            ) as *mut jpeg_component_info;
            (*cinfo).Ss = if (*master).pass_number % 4i32 < 2i32 {
                1i32
            } else {
                ((*(*cinfo).master).trellis_freq_split) + 1i32
            };
            (*cinfo).Se = if (*master).pass_number % 4i32 < 2i32 {
                (*(*cinfo).master).trellis_freq_split
            } else {
                (DCTSIZE2) - 1i32
            }
        } else {
            (*cinfo).cur_comp_info[0] = &mut *(*cinfo).comp_info.offset(
                ((*master).pass_number / (2i32 * (*(*cinfo).master).trellis_num_loops)) as isize,
            ) as *mut jpeg_component_info;
            (*cinfo).Ss = 1i32;
            (*cinfo).Se = DCTSIZE2 - 1i32
        }
    } else if !(*cinfo).scan_info.is_null() {
        /* Prepare for current scan --- the script is already validated */
        let mut scanptr: *const jpeg_scan_info =
            (*cinfo).scan_info.offset((*master).scan_number as isize);
        (*cinfo).comps_in_scan = (*scanptr).comps_in_scan;
        ci = 0i32;
        while ci < (*scanptr).comps_in_scan {
            (*cinfo).cur_comp_info[ci as usize] = &mut *(*cinfo)
                .comp_info
                .offset(*(*scanptr).component_index.as_ptr().offset(ci as isize) as isize)
                as *mut jpeg_component_info;
            ci += 1
        }
        (*cinfo).Ss = (*scanptr).Ss;
        (*cinfo).Se = (*scanptr).Se;
        (*cinfo).Ah = (*scanptr).Ah;
        (*cinfo).Al = (*scanptr).Al;
        if (*(*cinfo).master).optimize_scans != 0 {
            /* luma frequency split passes */
            if (*master).scan_number
                >= (*(*cinfo).master).num_scans_luma_dc
                    + 3i32 * (*(*cinfo).master).Al_max_luma
                    + 2i32
                && (*master).scan_number < (*(*cinfo).master).num_scans_luma
            {
                (*cinfo).Al = (*master).best_Al_luma
            }
            /* chroma frequency split passes */
            if (*master).scan_number
                >= (*(*cinfo).master).num_scans_luma
                    + (*(*cinfo).master).num_scans_chroma_dc
                    + (6i32 * (*(*cinfo).master).Al_max_chroma + 4i32)
                && (*master).scan_number < (*cinfo).num_scans
            {
                (*cinfo).Al = (*master).best_Al_chroma
            }
        }
        /* save value for later retrieval during printout of scans */
        (*master).actual_Al[(*master).scan_number as usize] = (*cinfo).Al
    } else {
        /* Prepare for single sequential-JPEG scan containing all components */
        if (*cinfo).num_components > MAX_COMPS_IN_SCAN {
            (*(*cinfo).err).msg_code = super::jerror::JERR_COMPONENT_COUNT as c_int;
            (*(*cinfo).err).msg_parm.i[0] = (*cinfo).num_components;
            (*(*cinfo).err).msg_parm.i[1] = 4i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        (*cinfo).comps_in_scan = (*cinfo).num_components;
        ci = 0i32;
        while ci < (*cinfo).num_components {
            (*cinfo).cur_comp_info[ci as usize] = &mut *(*cinfo).comp_info.offset(ci as isize)
                as *mut jpeg_component_info;
            ci += 1
        }
        (*cinfo).Ss = 0i32;
        (*cinfo).Se = DCTSIZE2 - 1i32;
        (*cinfo).Ah = 0i32;
        (*cinfo).Al = 0i32
    };
}

unsafe extern "C" fn per_scan_setup(mut cinfo: j_compress_ptr)
/* Do computations that are needed before processing a JPEG scan */
/* cinfo->comps_in_scan and cinfo->cur_comp_info[] are already set */
{
    
    
    
     let mut tmp:  c_int =  0; let mut compptr:  *mut jpeg_component_info =
    
        ::std::ptr::null_mut::< jpeg_component_info>();
    if (*cinfo).comps_in_scan == 1i32 {
        /* Noninterleaved (single-component) scan */
        compptr = (*cinfo).cur_comp_info[0];
        /* Overall image size in MCUs */
        (*cinfo).MCUs_per_row = (*compptr).width_in_blocks;
        (*cinfo).MCU_rows_in_scan = (*compptr).height_in_blocks;
        /* For noninterleaved scan, always one block per MCU */
        (*compptr).MCU_width = 1i32;
        (*compptr).MCU_height = 1i32;
        (*compptr).MCU_blocks = 1i32;
        (*compptr).MCU_sample_width = DCTSIZE;
        (*compptr).last_col_width = 1i32;
        /* For noninterleaved scans, it is convenient to define last_row_height
         * as the number of block rows present in the last iMCU row.
         */
        tmp = ( (*compptr)
            .height_in_blocks % (*compptr).v_samp_factor as c_uint) as c_int;
        if tmp == 0i32 {
            tmp = (*compptr).v_samp_factor
        }
        (*compptr).last_row_height = tmp;
        /* Prepare array describing MCU composition */
        (*cinfo).blocks_in_MCU = 1i32;
        (*cinfo).MCU_membership[0] = 0i32
    } else {
        if (*cinfo).comps_in_scan <= 0i32
            || (*cinfo).comps_in_scan > MAX_COMPS_IN_SCAN
        {
            (*(*cinfo).err).msg_code = super::jerror::JERR_COMPONENT_COUNT as c_int;
            (*(*cinfo).err).msg_parm.i[0] = (*cinfo).comps_in_scan;
            (*(*cinfo).err).msg_parm.i[1] = 4i32;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        /* Overall image size in MCUs */
        (*cinfo).MCUs_per_row = jdiv_round_up(
            (*cinfo).image_width as c_long,
            ((*cinfo).max_h_samp_factor * DCTSIZE) as c_long,
        ) as JDIMENSION;
        (*cinfo).MCU_rows_in_scan = jdiv_round_up(
            (*cinfo).image_height as c_long,
            ((*cinfo).max_v_samp_factor * DCTSIZE) as c_long,
        ) as JDIMENSION;
        (*cinfo).blocks_in_MCU = 0i32;
         let mut ci:   c_int =  0i32;
        while ci < (*cinfo).comps_in_scan {
             compptr = (*cinfo).cur_comp_info[ci as usize];
            /* Sampling factors give # of blocks of component in each MCU */
            (*compptr).MCU_width = (*compptr).h_samp_factor;
            (*compptr).MCU_height = (*compptr).v_samp_factor;
            (*compptr).MCU_blocks = (*compptr).MCU_width * (*compptr).MCU_height;
            (*compptr).MCU_sample_width = (*compptr).MCU_width * DCTSIZE;
            /* Figure number of non-dummy blocks in last MCU column & row */
            tmp = ( (*compptr)
                .width_in_blocks % (*compptr).MCU_width as c_uint)
                as c_int;
            if tmp == 0i32 {
                tmp = (*compptr).MCU_width
            }
            (*compptr).last_col_width = tmp;
            tmp = ( (*compptr)
                .height_in_blocks % (*compptr).MCU_height as c_uint)
                as c_int;
            if tmp == 0i32 {
                tmp = (*compptr).MCU_height
            }
            (*compptr).last_row_height = tmp;
             let mut mcublks:   c_int =  (*compptr).MCU_blocks;
            if (*cinfo).blocks_in_MCU + mcublks > C_MAX_BLOCKS_IN_MCU {
                (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_MCU_SIZE as c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as j_common_ptr
                );
            }
            loop {
                let fresh1 = mcublks;
                mcublks -=  1;
                if !(fresh1 > 0i32) {
                    break;
                }
                let fresh2 = (*cinfo).blocks_in_MCU;
                (*cinfo).blocks_in_MCU = (*cinfo).blocks_in_MCU + 1;
                (*cinfo).MCU_membership[fresh2 as usize] = ci
            }
            ci += 1
        }
    }
    /* Convert restart specified in rows to actual MCU count. */
    /* Note that count must fit in 16 bits, so we provide limiting. */
    if (*cinfo).restart_in_rows > 0i32 {
        let mut nominal: c_long =
            (*cinfo).restart_in_rows as c_long * (*cinfo).MCUs_per_row as c_long;
        (*cinfo).restart_interval = if nominal < 65535i64 {
            nominal
        } else {
            65535i64
        } as c_uint
    };
}
/*
 * Per-pass setup.
 * This is called at the beginning of each pass.  We determine which modules
 * will be active during this pass and give them appropriate start_pass calls.
 * We also set is_last_pass to indicate whether any more passes will be
 * required.
 */

unsafe extern "C" fn prepare_for_pass(mut cinfo: j_compress_ptr) {
     let mut current_block_54:  u64;let mut master: my_master_ptr =
        (*cinfo).master as my_master_ptr;
    (*(*cinfo).master).trellis_passes =
        ((*master).pass_number < (*master).pass_number_scan_opt_base) as c_int;
    
    match  (*master).pass_type {
        0 => {
            /* Initial pass: will collect input data, and do either Huffman
             * optimization or data output for the first scan.
             */
            select_scan_parameters(cinfo);
            per_scan_setup(cinfo);
            if (*cinfo).raw_data_in == 0 {
                Some(
                    (*(*cinfo).cconvert)
                        .start_pass
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
                Some(
                    (*(*cinfo).downsample)
                        .start_pass
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
                Some(
                    (*(*cinfo).prep)
                        .start_pass
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo, JBUF_PASS_THRU
                );
            }
            Some(
                (*(*cinfo).fdct)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
            Some(
                (*(*cinfo).entropy)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                (((*cinfo).optimize_coding != 0 || (*(*cinfo).master).trellis_quant != 0)
                    && (*cinfo).arith_code == 0) as c_int,
            );
            Some(
                (*(*cinfo).coef)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                if (*master).total_passes > 1i32 {
                    JBUF_SAVE_AND_PASS as c_int
                } else {
                    JBUF_PASS_THRU as c_int
                } as J_BUF_MODE,
            );
            Some(
                (*(*cinfo).main)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo, JBUF_PASS_THRU
            );
            if (*cinfo).optimize_coding != 0 || (*(*cinfo).master).trellis_quant != 0 {
                /* No immediate data output; postpone writing frame/scan headers */
                (*master).pub_0.call_pass_startup = FALSE
            } else {
                /* Will write frame/scan headers at first jpeg_write_scanlines call */
                (*master).pub_0.call_pass_startup = TRUE
            }
            current_block_54 = 2500484646272006982;
        }
        1 => {
            /* Do Huffman optimization for a scan after the first one. */
            select_scan_parameters(cinfo);
            per_scan_setup(cinfo);
            if (*cinfo).Ss != 0i32 || (*cinfo).Ah == 0i32 || (*cinfo).arith_code != 0 {
                Some(
                    (*(*cinfo).entropy)
                        .start_pass
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo, TRUE);
                Some(
                    (*(*cinfo).coef)
                        .start_pass
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo, JBUF_CRANK_DEST
                );
                (*master).pub_0.call_pass_startup = FALSE;
                current_block_54 = 2500484646272006982;
            } else {
                /* Special case: Huffman DC refinement scans need no Huffman table
                 * and therefore we can skip the optimization pass for them.
                 */
                (*master).pass_type = output_pass;
                (*master).pass_number += 1;
                current_block_54 = 12235309447780442549;
            }
        }
        2 => {
            current_block_54 = 12235309447780442549;
        }
        3 => {
            if (*master).pass_number
                % ((*cinfo).num_components
                    * (if (*(*cinfo).master).use_scans_in_trellis != 0 {
                        4i32
                    } else {
                        2i32
                    }))
                == 1i32
                && (*(*cinfo).master).trellis_q_opt != 0
            {
                
                 
                 let mut i:   c_int =  0i32;
                while i < NUM_QUANT_TBLS {
                      let mut j:   c_int =  1i32;
                    while j < DCTSIZE2 {
                        (*(*cinfo).master).norm_src[i as usize][j as usize] = 0.0f64;
                        (*(*cinfo).master).norm_coef[i as usize][j as usize] = 0.0f64;
                        j += 1
                    }
                    i += 1
                }
            }
            Some(
                (*(*cinfo).entropy)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo, ((*cinfo).arith_code == 0) as c_int
            );
            Some(
                (*(*cinfo).coef)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo, JBUF_REQUANT);
            (*master).pub_0.call_pass_startup = FALSE;
            current_block_54 = 2500484646272006982;
        }
        _ => {
            (*(*cinfo).err).msg_code = super::jerror::JERR_NOT_COMPILED as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
            current_block_54 = 2500484646272006982;
        }
    }
    match current_block_54 {
        12235309447780442549 =>
        /*FALLTHROUGH*/
        /* Do a data-output pass. */
        /* We need not repeat per-scan setup if prior optimization pass did it. */
        {
            if (*cinfo).optimize_coding == 0 {
                select_scan_parameters(cinfo);
                per_scan_setup(cinfo);
            }
            if (*(*cinfo).master).optimize_scans != 0 {
                (*master).saved_dest = (*cinfo).dest;
                (*cinfo).dest =
                    NULL as *mut jpeg_destination_mgr;
                (*master).scan_size[(*master).scan_number as usize] = 0u64;
                jpeg_mem_dest_internal(
                    cinfo,
                    &mut *(*master)
                        .scan_buffer
                        .as_mut_ptr()
                        .offset((*master).scan_number as isize),
                    &mut *(*master)
                        .scan_size
                        .as_mut_ptr()
                        .offset((*master).scan_number as isize),
                    JPOOL_IMAGE,
                );
                Some(
                    (*(*cinfo).dest)
                        .init_destination
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
            }
            Some(
                (*(*cinfo).entropy)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo, FALSE);
            Some(
                (*(*cinfo).coef)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo, JBUF_CRANK_DEST
            );
            /* We emit frame/scan headers now */
            if (*master).scan_number == 0i32 {
                Some(
                    (*(*cinfo).marker)
                        .write_frame_header
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
            }
            Some(
                (*(*cinfo).marker)
                    .write_scan_header
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo);
            (*master).pub_0.call_pass_startup = FALSE
        }
        _ => {}
    }
    (*master).pub_0.is_last_pass =
        ((*master).pass_number == (*master).total_passes - 1i32) as c_int;
    /* Set up progress monitor's pass info if present */
    if !(*cinfo).progress.is_null() {
        (*(*cinfo).progress).completed_passes = (*master).pass_number;
        (*(*cinfo).progress).total_passes = (*master).total_passes
    };
}
/*
 * Special start-of-pass hook.
 * This is called by jpeg_write_scanlines if call_pass_startup is TRUE.
 * In single-pass processing, we need this hook because we don't want to
 * write frame/scan headers during jpeg_start_compress; we want to let the
 * application write COM markers etc. between jpeg_start_compress and the
 * jpeg_write_scanlines loop.
 * In multi-pass processing, this routine is not used.
 */

unsafe extern "C" fn pass_startup(mut cinfo: j_compress_ptr) {
    (*(*cinfo).master).call_pass_startup = FALSE; /* reset flag so call only once */
    Some(
        (*(*cinfo).marker)
            .write_frame_header
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    Some(
        (*(*cinfo).marker)
            .write_scan_header
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
}

unsafe extern "C" fn copy_buffer(
    mut cinfo: j_compress_ptr,
    mut scan_idx: c_int,
) {
    let mut master: my_master_ptr =
        (*cinfo).master as my_master_ptr;
    let mut size: c_ulong = (*master).scan_size[scan_idx as usize];
    let mut src: *mut c_uchar = (*master).scan_buffer[scan_idx as usize];
    
    if (*(*cinfo).err).trace_level > 0i32 {
          eprint!("SCAN ");
         let mut i:   c_int =  0i32;
        while i < (*(*cinfo).scan_info.offset(scan_idx as isize)).comps_in_scan {
                     eprint!("{:}{:}",
        {
    CStr::from_ptr(if i == 0i32 {
                       b"\x00".as_ptr() as *const c_char
                   } else {
                       b",\x00".as_ptr() as *const c_char
                   }).to_str().unwrap()
},
        (*(*cinfo).scan_info.offset(scan_idx as isize)).component_index[i as usize]);
            i += 1
        }
        
        
                 eprint!(": {:} {:}", (*(*cinfo).scan_info.offset(scan_idx as isize)).Ss,
        (*(*cinfo).scan_info.offset(scan_idx as isize)).Se);         eprint!(" {:} {:}", (*(*cinfo).scan_info.offset(scan_idx as isize)).Ah,
        (*master).actual_Al[scan_idx as usize]); eprintln!("");
    }
    while size >= (*(*cinfo).dest).free_in_buffer {
        memcpy(
            (*(*cinfo).dest).next_output_byte as *mut c_void,
            src as *const c_void,
            (*(*cinfo).dest).free_in_buffer,
        );
        src = src.offset((*(*cinfo).dest).free_in_buffer as isize);
        size -=  (*(*cinfo).dest).free_in_buffer;
        (*(*cinfo).dest).next_output_byte = (*(*cinfo).dest)
            .next_output_byte
            .offset((*(*cinfo).dest).free_in_buffer as isize);
        (*(*cinfo).dest).free_in_buffer = 0u64;
        if Some(
            (*(*cinfo).dest)
                .empty_output_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            (*(*cinfo).err).msg_code = super::jerror::JERR_UNSUPPORTED_SUSPEND as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
    }
    memcpy(
        (*(*cinfo).dest).next_output_byte as *mut c_void,
        src as *const c_void,
        size,
    );
    (*(*cinfo).dest).next_output_byte = (*(*cinfo).dest).next_output_byte.offset(size as isize);
    (*(*cinfo).dest).free_in_buffer = (*(*cinfo).dest).free_in_buffer - size;
}

unsafe extern "C" fn select_scans(
    mut cinfo: j_compress_ptr,
    mut next_scan_number: c_int,
) {
     let mut base_scan_idx:  c_int =  0i32;let mut master: my_master_ptr =
        (*cinfo).master as my_master_ptr;
    
    let mut luma_freq_split_scan_start: c_int =
        (*(*cinfo).master).num_scans_luma_dc + 3i32 * (*(*cinfo).master).Al_max_luma + 2i32;
    let mut chroma_freq_split_scan_start: c_int = (*(*cinfo).master).num_scans_luma
        + (*(*cinfo).master).num_scans_chroma_dc
        + (6i32 * (*(*cinfo).master).Al_max_chroma + 4i32);
    let mut passes_per_scan: c_int = if (*cinfo).optimize_coding != 0 {
        2i32
    } else {
        1i32
    };
    if next_scan_number > 1i32 && next_scan_number <= luma_freq_split_scan_start {
        if (next_scan_number - 1i32) % 3i32 == 2i32 {
              let mut cost:  c_ulong =  0u64;let mut Al: c_int = (next_scan_number - 1i32) / 3i32;
            
            
            cost +=  (*master).scan_size[(next_scan_number - 2i32) as usize];
            cost +=  (*master).scan_size[(next_scan_number - 1i32) as usize];
             let mut i:   c_int =  0i32;
            while i < Al {
                cost +=  (*master).scan_size[(3i32 + 3i32 * i) as usize];
                i += 1
            }
            if Al == 0i32 || cost < (*master).best_cost {
                (*master).best_cost = cost;
                (*master).best_Al_luma = Al
            } else {
                (*master).scan_number = luma_freq_split_scan_start - 1i32;
                (*master).pass_number = passes_per_scan * ((*master).scan_number + 1i32) - 1i32
                    + (*master).pass_number_scan_opt_base
            }
        }
    } else if next_scan_number > luma_freq_split_scan_start
        && next_scan_number <= (*(*cinfo).master).num_scans_luma
    {
        if next_scan_number == luma_freq_split_scan_start + 1i32 {
            (*master).best_freq_split_idx_luma = 0i32;
            (*master).best_cost = (*master).scan_size[(next_scan_number - 1i32) as usize]
        } else if (next_scan_number - luma_freq_split_scan_start) % 2i32 == 1i32 {
             let mut cost_0:  c_ulong =  0u64;let mut idx: c_int = next_scan_number - luma_freq_split_scan_start >> 1i32;
            
            cost_0 +=  (*master).scan_size[(next_scan_number - 2i32) as usize];
            cost_0 +=  (*master).scan_size[(next_scan_number - 1i32) as usize];
            if cost_0 < (*master).best_cost {
                (*master).best_cost = cost_0;
                (*master).best_freq_split_idx_luma = idx
            }
            /* if after testing first 3, no split is the best, don't search further */
            if idx == 2i32 && (*master).best_freq_split_idx_luma == 0i32
                || idx == 3i32 && (*master).best_freq_split_idx_luma != 2i32
                || idx == 4i32 && (*master).best_freq_split_idx_luma != 4i32
            {
                (*master).scan_number = (*(*cinfo).master).num_scans_luma - 1i32;
                (*master).pass_number = passes_per_scan * ((*master).scan_number + 1i32) - 1i32
                    + (*master).pass_number_scan_opt_base;
                (*master).pub_0.is_last_pass =
                    ((*master).pass_number == (*master).total_passes - 1i32) as c_int
            }
        }
    } else if (*cinfo).num_scans > (*(*cinfo).master).num_scans_luma {
        if next_scan_number
            == (*(*cinfo).master).num_scans_luma + (*(*cinfo).master).num_scans_chroma_dc
        {
            base_scan_idx = (*(*cinfo).master).num_scans_luma;
            (*master).interleave_chroma_dc = ((*master).scan_size[base_scan_idx as usize]
                <=  (*master).scan_size[(base_scan_idx + 1i32) as usize] +
    (*master).scan_size[(base_scan_idx + 2i32) as usize])
                as c_int
        } else if next_scan_number
            > (*(*cinfo).master).num_scans_luma + (*(*cinfo).master).num_scans_chroma_dc
            && next_scan_number <= chroma_freq_split_scan_start
        {
            base_scan_idx =
                (*(*cinfo).master).num_scans_luma + (*(*cinfo).master).num_scans_chroma_dc;
            if (next_scan_number - base_scan_idx) % 6i32 == 4i32 {
                  let mut cost_1:  c_ulong =  0u64;let mut Al_0: c_int = (next_scan_number - base_scan_idx) / 6i32;
                
                
                cost_1 +=  (*master).scan_size[(next_scan_number - 4i32) as usize];
                cost_1 +=  (*master).scan_size[(next_scan_number - 3i32) as usize];
                cost_1 +=  (*master).scan_size[(next_scan_number - 2i32) as usize];
                cost_1 +=  (*master).scan_size[(next_scan_number - 1i32) as usize];
                 let mut i_0:   c_int =  0i32;
                while i_0 < Al_0 {
                    cost_1 +=  
                        (*master).scan_size[(base_scan_idx + 4i32 + 6i32 * i_0) as usize];
                    cost_1 +=  
                        (*master).scan_size[(base_scan_idx + 5i32 + 6i32 * i_0) as usize];
                    i_0 += 1
                }
                if Al_0 == 0i32 || cost_1 < (*master).best_cost {
                    (*master).best_cost = cost_1;
                    (*master).best_Al_chroma = Al_0
                } else {
                    (*master).scan_number = chroma_freq_split_scan_start - 1i32;
                    (*master).pass_number = passes_per_scan * ((*master).scan_number + 1i32) - 1i32
                        + (*master).pass_number_scan_opt_base
                }
            }
        } else if next_scan_number > chroma_freq_split_scan_start
            && next_scan_number <= (*cinfo).num_scans
        {
            if next_scan_number == chroma_freq_split_scan_start + 2i32 {
                (*master).best_freq_split_idx_chroma = 0i32;
                (*master).best_cost = (*master).scan_size[(next_scan_number - 2i32) as usize];
                (*master).best_cost =  (*master)
                    .best_cost + (*master).scan_size[(next_scan_number - 1i32) as usize]
            } else if (next_scan_number - chroma_freq_split_scan_start) % 4i32 == 2i32 {
                 let mut cost_2:  c_ulong =  0u64;let mut idx_0: c_int =
                    next_scan_number - chroma_freq_split_scan_start >> 2i32;
                
                cost_2 +=  (*master).scan_size[(next_scan_number - 4i32) as usize];
                cost_2 +=  (*master).scan_size[(next_scan_number - 3i32) as usize];
                cost_2 +=  (*master).scan_size[(next_scan_number - 2i32) as usize];
                cost_2 +=  (*master).scan_size[(next_scan_number - 1i32) as usize];
                if cost_2 < (*master).best_cost {
                    (*master).best_cost = cost_2;
                    (*master).best_freq_split_idx_chroma = idx_0
                }
                /* if after testing first 3, no split is the best, don't search further */
                if idx_0 == 2i32 && (*master).best_freq_split_idx_chroma == 0i32
                    || idx_0 == 3i32 && (*master).best_freq_split_idx_chroma != 2i32
                    || idx_0 == 4i32 && (*master).best_freq_split_idx_chroma != 4i32
                {
                    (*master).scan_number = (*cinfo).num_scans - 1i32;
                    (*master).pass_number = passes_per_scan * ((*master).scan_number + 1i32) - 1i32
                        + (*master).pass_number_scan_opt_base;
                    (*master).pub_0.is_last_pass =
                        ((*master).pass_number == (*master).total_passes - 1i32) as c_int
                }
            }
        }
    }
    if (*master).scan_number == (*cinfo).num_scans - 1i32 {
        
          
        let mut min_Al: c_int = if (*master).best_Al_luma < (*master).best_Al_chroma {
            (*master).best_Al_luma
        } else {
            (*master).best_Al_chroma
        };
        copy_buffer(cinfo, 0i32);
        if (*cinfo).num_scans > (*(*cinfo).master).num_scans_luma
            && (*(*cinfo).master).dc_scan_opt_mode != 0i32
        {
            base_scan_idx = (*(*cinfo).master).num_scans_luma;
            if (*master).interleave_chroma_dc != 0 && (*(*cinfo).master).dc_scan_opt_mode != 1i32 {
                copy_buffer(cinfo, base_scan_idx);
            } else {
                copy_buffer(cinfo, base_scan_idx + 1i32);
                copy_buffer(cinfo, base_scan_idx + 2i32);
            }
        }
        if (*master).best_freq_split_idx_luma == 0i32 {
            copy_buffer(cinfo, luma_freq_split_scan_start);
        } else {
            copy_buffer(
                cinfo,
                luma_freq_split_scan_start
                    + 2i32 * ((*master).best_freq_split_idx_luma - 1i32)
                    + 1i32,
            );
            copy_buffer(
                cinfo,
                luma_freq_split_scan_start
                    + 2i32 * ((*master).best_freq_split_idx_luma - 1i32)
                    + 2i32,
            );
        }
         let mut Al_1:   c_int =  (*master).best_Al_luma - 1i32;
        while Al_1 >= min_Al {
            copy_buffer(cinfo, 3i32 + 3i32 * Al_1);
            Al_1 -= 1
        }
        if (*cinfo).num_scans > (*(*cinfo).master).num_scans_luma {
            if (*master).best_freq_split_idx_chroma == 0i32 {
                copy_buffer(cinfo, chroma_freq_split_scan_start);
                copy_buffer(cinfo, chroma_freq_split_scan_start + 1i32);
            } else {
                copy_buffer(
                    cinfo,
                    chroma_freq_split_scan_start
                        + 4i32 * ((*master).best_freq_split_idx_chroma - 1i32)
                        + 2i32,
                );
                copy_buffer(
                    cinfo,
                    chroma_freq_split_scan_start
                        + 4i32 * ((*master).best_freq_split_idx_chroma - 1i32)
                        + 3i32,
                );
                copy_buffer(
                    cinfo,
                    chroma_freq_split_scan_start
                        + 4i32 * ((*master).best_freq_split_idx_chroma - 1i32)
                        + 4i32,
                );
                copy_buffer(
                    cinfo,
                    chroma_freq_split_scan_start
                        + 4i32 * ((*master).best_freq_split_idx_chroma - 1i32)
                        + 5i32,
                );
            }
            base_scan_idx =
                (*(*cinfo).master).num_scans_luma + (*(*cinfo).master).num_scans_chroma_dc;
            Al_1 = (*master).best_Al_chroma - 1i32;
            while Al_1 >= min_Al {
                copy_buffer(cinfo, base_scan_idx + 6i32 * Al_1 + 4i32);
                copy_buffer(cinfo, base_scan_idx + 6i32 * Al_1 + 5i32);
                Al_1 -= 1
            }
        }
        Al_1 = min_Al - 1i32;
        while Al_1 >= 0i32 {
            copy_buffer(cinfo, 3i32 + 3i32 * Al_1);
            if (*cinfo).num_scans > (*(*cinfo).master).num_scans_luma {
                copy_buffer(cinfo, base_scan_idx + 6i32 * Al_1 + 4i32);
                copy_buffer(cinfo, base_scan_idx + 6i32 * Al_1 + 5i32);
            }
            Al_1 -= 1
        }
         let mut i_1:   c_int =  0i32;
        while i_1 < (*cinfo).num_scans {
            if !(*master).scan_buffer[i_1 as usize].is_null() {
                free((*master).scan_buffer[i_1 as usize] as *mut c_void);
            }
            i_1 += 1
        }
    };
}
/*
 * Finish up at end of pass.
 */

unsafe extern "C" fn finish_pass_master(mut cinfo: j_compress_ptr) {
    let mut master: my_master_ptr =
        (*cinfo).master as my_master_ptr;
    /* The entropy coder always needs an end-of-pass call,
     * either to analyze statistics or to flush its output buffer.
     */
    Some(
        (*(*cinfo).entropy)
            .finish_pass
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    /* Update state for next pass */
    match  (*master).pass_type {
        0 => {
            /* next pass is either output of scan 0 (after optimization)
             * or output of scan 1 (if no optimization).
             */
            if (*(*cinfo).master).trellis_quant != 0 {
                (*master).pass_type = trellis_pass
            } else {
                (*master).pass_type = output_pass;
                if (*cinfo).optimize_coding == 0 {
                    (*master).scan_number += 1
                }
            }
        }
        1 => {
            /* next pass is always output of current scan */
            (*master).pass_type =
                if (*master).pass_number < (*master).pass_number_scan_opt_base - 1i32 {
                    trellis_pass as c_int
                } else {
                    output_pass as c_int
                } as c_pass_type
        }
        2 => {
            /* next pass is either optimization or output of next scan */
            if (*cinfo).optimize_coding != 0 {
                (*master).pass_type = huff_opt_pass
            }
            if (*(*cinfo).master).optimize_scans != 0 {
                Some(
                    (*(*cinfo).dest)
                        .term_destination
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
                (*cinfo).dest = (*master).saved_dest;
                select_scans(cinfo, (*master).scan_number + 1i32);
            }
            (*master).scan_number += 1
        }
        3 => {
            if (*cinfo).optimize_coding != 0 {
                (*master).pass_type = huff_opt_pass
            } else {
                (*master).pass_type =
                    if (*master).pass_number < (*master).pass_number_scan_opt_base - 1i32 {
                        trellis_pass as c_int
                    } else {
                        output_pass as c_int
                    } as c_pass_type
            }
            if ((*master).pass_number + 1i32)
                % ((*cinfo).num_components
                    * (if (*(*cinfo).master).use_scans_in_trellis != 0 {
                        4i32
                    } else {
                        2i32
                    }))
                == 0i32
                && (*(*cinfo).master).trellis_q_opt != 0
            {
                
                 
                 let mut i:   c_int =  0i32;
                while i < NUM_QUANT_TBLS {
                      let mut j:   c_int =  1i32;
                    while j < DCTSIZE2 {
                        if (*(*cinfo).master).norm_coef[i as usize][j as usize] != 0.0f64 {
                            let mut q: c_int =
                                ((*(*cinfo).master).norm_src[i as usize][j as usize]
                                    / (*(*cinfo).master).norm_coef[i as usize][j as usize]
                                    + 0.5f64) as c_int;
                            if q > 254i32 {
                                q = 254i32
                            }
                            if q < 1i32 {
                                q = 1i32
                            }
                            (*(*cinfo).quant_tbl_ptrs[i as usize]).quantval[j as usize] =
                                q as UINT16
                        }
                        j += 1
                    }
                    i += 1
                }
            }
        }
        _ => {}
    }
    (*master).pass_number += 1;
}
/*
 * Initialize master compression control.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_c_master_control(
    mut cinfo: j_compress_ptr,
    mut transcode_only: boolean,
) {
    let mut master: my_master_ptr =
        (*cinfo).master as my_master_ptr;
    (*master).pub_0.prepare_for_pass =
        Some(prepare_for_pass as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    (*master).pub_0.pass_startup =
        Some(pass_startup as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    (*master).pub_0.finish_pass =
        Some(finish_pass_master as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    (*master).pub_0.is_last_pass = FALSE;
    (*master).pub_0.call_pass_startup = FALSE;
    /* Validate parameters, determine derived values */
    initial_setup(cinfo, transcode_only); /* assume default tables no good for progressive mode */
    if !(*cinfo).scan_info.is_null() {
        validate_script(cinfo);
    } else {
        (*cinfo).progressive_mode = FALSE;
        (*cinfo).num_scans = 1i32
    }
    if (*cinfo).progressive_mode != 0 && (*cinfo).arith_code == 0 {
        /*  TEMPORARY HACK ??? */
        (*cinfo).optimize_coding = TRUE
    }
    /* Initialize my private state */
    if transcode_only != 0 {
        /* no main pass in transcoding */
        if (*cinfo).optimize_coding != 0 {
            (*master).pass_type = huff_opt_pass
        } else {
            (*master).pass_type = output_pass
        }
    } else {
        /* for normal compression, first pass is always this type: */
        (*master).pass_type = main_pass
    }
    (*master).scan_number = 0i32;
    (*master).pass_number = 0i32;
    if (*cinfo).optimize_coding != 0 {
        (*master).total_passes = (*cinfo).num_scans * 2i32
    } else {
        (*master).total_passes = (*cinfo).num_scans
    }
    (*master).jpeg_version =
        
        b"mozjpeg version 4.0.0 (build 20191022)\x00".as_ptr() as *const c_char;
    (*master).pass_number_scan_opt_base = 0i32;
    if (*(*cinfo).master).trellis_quant != 0 {
        if (*cinfo).optimize_coding != 0 {
            (*master).pass_number_scan_opt_base = (if (*(*cinfo).master).use_scans_in_trellis != 0 {
                4i32
            } else {
                2i32
            }) * (*cinfo).num_components
                * (*(*cinfo).master).trellis_num_loops
        } else {
            (*master).pass_number_scan_opt_base = (if (*(*cinfo).master).use_scans_in_trellis != 0 {
                2i32
            } else {
                1i32
            }) * (*cinfo).num_components
                * (*(*cinfo).master).trellis_num_loops
                + 1i32
        }
        (*master).total_passes += (*master).pass_number_scan_opt_base
    }
    if (*(*cinfo).master).optimize_scans != 0 {
         
        (*master).best_Al_chroma = 0i32;
         let mut i:   c_int =  0i32;
        while i < (*cinfo).num_scans {
            (*master).scan_buffer[i as usize] = NULL as *mut c_uchar;
            i += 1
        }
    };
}
