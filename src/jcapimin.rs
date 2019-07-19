pub use crate::jcmaster::{
    c_pass_type, huff_opt_pass, main_pass, my_comp_master, output_pass, trellis_pass,
};
pub use crate::jconfig_h::JPEG_LIB_VERSION;
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
pub use crate::jmorecfg_h::{boolean, FALSE, JCOEF, JDIMENSION, JOCTET, JSAMPLE, UINT16, UINT8};
pub use crate::jpegint_h::{
    jinit_marker_writer, jinit_memory_mgr, jpeg_c_coef_controller, jpeg_c_main_controller,
    jpeg_c_prep_controller, jpeg_color_converter, jpeg_comp_master, jpeg_downsampler,
    jpeg_entropy_encoder, jpeg_forward_dct, jpeg_marker_writer, CSTATE_RAW_OK, CSTATE_SCANNING,
    CSTATE_START, CSTATE_WRCOEFS, JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT,
    JBUF_SAVE_AND_PASS, JBUF_SAVE_SOURCE, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_compress_ptr, jpeg_abort, jpeg_common_struct, jpeg_component_info,
    jpeg_compress_struct, jpeg_destination_mgr, jpeg_destroy, jpeg_error_mgr, jpeg_memory_mgr,
    jpeg_progress_mgr, jpeg_scan_info, jvirt_barray_control, jvirt_barray_ptr,
    jvirt_sarray_control, jvirt_sarray_ptr, C2RustUnnamed_1, C2RustUnnamed_2, JCS_YCbCr, JBLOCK,
    JBLOCKARRAY, JBLOCKROW, JCP_FASTEST, JCP_MAX_COMPRESSION, JCS_CMYK, JCS_EXT_ABGR, JCS_EXT_ARGB,
    JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX, JCS_EXT_XBGR,
    JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK, JDCT_FLOAT,
    JDCT_IFAST, JDCT_ISLOW, JHUFF_TBL, JPOOL_PERMANENT, JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE,
    JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD, NUM_HUFF_TBLS, NUM_QUANT_TBLS,
};
pub use crate::stddef_h::{size_t, NULL};
use crate::stdlib::memset;
use libc::{self, c_int, c_long, c_uint, c_ulong, c_void};
/*
 * jcapimin.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1998, Thomas G. Lane.
 * Modified 2003-2010 by Guido Vollbeding.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2014, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README file.
 *
 * This file contains application interface code for the compression half
 * of the JPEG library.  These are the "minimum" API routines that may be
 * needed in either the normal full-compression case or the transcoding-only
 * case.
 *
 * Most of the routines intended to be called directly by an application
 * are in this file or in jcapistd.c.  But also see jcparam.c for
 * parameter-setup helper routines, jcomapi.c for routines shared by
 * compression and decompression, and jctrans.c for the transcoding case.
 */
/* Initialization of JPEG compression objects.
 * jpeg_create_compress() and jpeg_create_decompress() are the exported
 * names that applications should call.  These expand to calls on
 * jpeg_CreateCompress and jpeg_CreateDecompress with additional information
 * passed for version mismatch checking.
 * NB: you must set up the error-manager BEFORE calling jpeg_create_xxx.
 */
/*
 * Initialization of a JPEG compression object.
 * The error manager must already be set up (in case memory manager fails).
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_CreateCompress(
    mut cinfo: j_compress_ptr,
    mut version: c_int,
    mut structsize: size_t,
) {
    let mut i: c_int = 0;
    (*cinfo).mem = NULL as *mut jpeg_memory_mgr;
    if version != JPEG_LIB_VERSION {
        (*(*cinfo).err).msg_code = JERR_BAD_LIB_VERSION as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = 62i32;
        (*(*cinfo).err).msg_parm.i[1usize] = version;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if structsize != ::std::mem::size_of::<jpeg_compress_struct>() as c_ulong {
        (*(*cinfo).err).msg_code = JERR_BAD_STRUCT_SIZE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] =
            ::std::mem::size_of::<jpeg_compress_struct>() as c_ulong as c_int;
        (*(*cinfo).err).msg_parm.i[1usize] = structsize as c_int;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    let mut err: *mut jpeg_error_mgr = (*cinfo).err;
    let mut client_data: *mut c_void = (*cinfo).client_data;
    memset(
        cinfo as *mut c_void,
        0i32,
        ::std::mem::size_of::<jpeg_compress_struct>() as c_ulong,
    );
    (*cinfo).err = err;
    (*cinfo).client_data = client_data;
    (*cinfo).is_decompressor = FALSE;
    jinit_memory_mgr(cinfo as j_common_ptr);
    (*cinfo).progress = NULL as *mut jpeg_progress_mgr;
    (*cinfo).dest = NULL as *mut jpeg_destination_mgr;
    (*cinfo).comp_info = NULL as *mut jpeg_component_info;
    i = 0i32;
    while i < NUM_QUANT_TBLS {
        (*cinfo).quant_tbl_ptrs[i as usize] = NULL as *mut JQUANT_TBL;
        i += 1
    }
    i = 0i32;
    while i < NUM_HUFF_TBLS {
        (*cinfo).dc_huff_tbl_ptrs[i as usize] = NULL as *mut JHUFF_TBL;
        (*cinfo).ac_huff_tbl_ptrs[i as usize] = NULL as *mut JHUFF_TBL;
        i += 1
    }
    (*cinfo).script_space = NULL as *mut jpeg_scan_info;
    (*cinfo).input_gamma = 1.0f64;
    (*cinfo).global_state = CSTATE_START;
    (*cinfo).master = (*(*cinfo).mem)
        .alloc_small
        .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_PERMANENT,
        ::std::mem::size_of::<my_comp_master>() as c_ulong,
    ) as *mut jpeg_comp_master;
    memset(
        (*cinfo).master as *mut c_void,
        0i32,
        ::std::mem::size_of::<my_comp_master>() as c_ulong,
    );
    (*(*cinfo).master).compress_profile = JCP_MAX_COMPRESSION as c_int;
}
/* Destruction of JPEG compression objects */
/*
 * Destruction of a JPEG compression object
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_destroy_compress(mut cinfo: j_compress_ptr) {
    jpeg_destroy(cinfo as j_common_ptr);
}
/* If you choose to abort compression or decompression before completing
 * jpeg_finish_(de)compress, then you need to clean up to release memory,
 * temporary files, etc.  You can just call jpeg_destroy_(de)compress
 * if you're done with the JPEG object, but if you want to clean it up and
 * reuse it, call this:
 */
/*
 * Abort processing of a JPEG compression operation,
 * but don't destroy the object itself.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_abort_compress(mut cinfo: j_compress_ptr) {
    jpeg_abort(cinfo as j_common_ptr);
}
/*
 * Forcibly suppress or un-suppress all quantization and Huffman tables.
 * Marks all currently defined tables as already written (if suppress)
 * or not written (if !suppress).  This will control whether they get emitted
 * by a subsequent jpeg_start_compress call.
 *
 * This routine is exported for use by applications that want to produce
 * abbreviated JPEG datastreams.  It logically belongs in jcparam.c, but
 * since it is called by jpeg_start_compress, we put it here --- otherwise
 * jcparam.o would be linked whether the application used it or not.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_suppress_tables(mut cinfo: j_compress_ptr, mut suppress: boolean) {
    let mut i: c_int = 0;
    let mut qtbl: *mut JQUANT_TBL = 0 as *mut JQUANT_TBL;
    let mut htbl: *mut JHUFF_TBL = 0 as *mut JHUFF_TBL;
    i = 0i32;
    while i < NUM_QUANT_TBLS {
        qtbl = (*cinfo).quant_tbl_ptrs[i as usize];
        if !qtbl.is_null() {
            (*qtbl).sent_table = suppress
        }
        i += 1
    }
    i = 0i32;
    while i < NUM_HUFF_TBLS {
        htbl = (*cinfo).dc_huff_tbl_ptrs[i as usize];
        if !htbl.is_null() {
            (*htbl).sent_table = suppress
        }
        htbl = (*cinfo).ac_huff_tbl_ptrs[i as usize];
        if !htbl.is_null() {
            (*htbl).sent_table = suppress
        }
        i += 1
    }
}
/*
 * Finish JPEG compression.
 *
 * If a multipass operating mode was selected, this may do a great deal of
 * work including most of the actual output.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_finish_compress(mut cinfo: j_compress_ptr) {
    let mut iMCU_row: JDIMENSION = 0;
    if (*cinfo).global_state == CSTATE_SCANNING || (*cinfo).global_state == CSTATE_RAW_OK {
        if (*cinfo).next_scanline < (*cinfo).image_height {
            (*(*cinfo).err).msg_code = JERR_TOO_LITTLE_DATA as c_int;
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
        (*(*cinfo).master)
            .finish_pass
            .expect("non-null function pointer")(cinfo);
    } else if (*cinfo).global_state != CSTATE_WRCOEFS {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    while 0 == (*(*cinfo).master).is_last_pass {
        (*(*cinfo).master)
            .prepare_for_pass
            .expect("non-null function pointer")(cinfo);
        iMCU_row = 0i32 as JDIMENSION;
        while iMCU_row < (*cinfo).total_iMCU_rows {
            if !(*cinfo).progress.is_null() {
                (*(*cinfo).progress).pass_counter = iMCU_row as c_long;
                (*(*cinfo).progress).pass_limit = (*cinfo).total_iMCU_rows as c_long;
                (*(*cinfo).progress)
                    .progress_monitor
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if 0 == (*(*cinfo).coef)
                .compress_data
                .expect("non-null function pointer")(
                cinfo, NULL as *mut c_void as JSAMPIMAGE
            ) {
                (*(*cinfo).err).msg_code = JERR_CANT_SUSPEND as c_int;
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            iMCU_row = iMCU_row.wrapping_add(1)
        }
        (*(*cinfo).master)
            .finish_pass
            .expect("non-null function pointer")(cinfo);
    }
    (*(*cinfo).marker)
        .write_file_trailer
        .expect("non-null function pointer")(cinfo);
    (*(*cinfo).dest)
        .term_destination
        .expect("non-null function pointer")(cinfo);
    jpeg_abort(cinfo as j_common_ptr);
}
/* Write a special marker.  See libjpeg.txt concerning safe usage. */
/*
 * Write a special marker.
 * This is only recommended for writing COM or APPn markers.
 * Must be called after jpeg_start_compress() and before
 * first call to jpeg_write_scanlines() or jpeg_write_raw_data().
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_write_marker(
    mut cinfo: j_compress_ptr,
    mut marker: c_int,
    mut dataptr: *const JOCTET,
    mut datalen: c_uint,
) {
    let mut write_marker_byte: Option<unsafe extern "C" fn(_: j_compress_ptr, _: c_int) -> ()> =
        None;
    if (*cinfo).next_scanline != 0i32 as c_uint
        || (*cinfo).global_state != CSTATE_SCANNING
            && (*cinfo).global_state != CSTATE_RAW_OK
            && (*cinfo).global_state != CSTATE_WRCOEFS
    {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*(*cinfo).marker)
        .write_marker_header
        .expect("non-null function pointer")(cinfo, marker, datalen);
    write_marker_byte = (*(*cinfo).marker).write_marker_byte;
    loop {
        let fresh0 = datalen;
        datalen = datalen.wrapping_sub(1);
        if !(0 != fresh0) {
            break;
        }
        write_marker_byte.expect("non-null function pointer")(cinfo, *dataptr as c_int);
        dataptr = dataptr.offset(1isize)
    }
}
/* Same, but piecemeal. */
/* Same, but piecemeal. */
#[no_mangle]
pub unsafe extern "C" fn jpeg_write_m_header(
    mut cinfo: j_compress_ptr,
    mut marker: c_int,
    mut datalen: c_uint,
) {
    if (*cinfo).next_scanline != 0i32 as c_uint
        || (*cinfo).global_state != CSTATE_SCANNING
            && (*cinfo).global_state != CSTATE_RAW_OK
            && (*cinfo).global_state != CSTATE_WRCOEFS
    {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*(*cinfo).marker)
        .write_marker_header
        .expect("non-null function pointer")(cinfo, marker, datalen);
}
#[no_mangle]
pub unsafe extern "C" fn jpeg_write_m_byte(mut cinfo: j_compress_ptr, mut val: c_int) {
    (*(*cinfo).marker)
        .write_marker_byte
        .expect("non-null function pointer")(cinfo, val);
}
/* Alternate compression function: just write an abbreviated table file */
/*
 * Alternate compression function: just write an abbreviated table file.
 * Before calling this, all parameters and a data destination must be set up.
 *
 * To produce a pair of files containing abbreviated tables and abbreviated
 * image data, one would proceed as follows:
 *
 *              initialize JPEG object
 *              set JPEG parameters
 *              set destination to table file
 *              jpeg_write_tables(cinfo);
 *              set destination to image file
 *              jpeg_start_compress(cinfo, FALSE);
 *              write data...
 *              jpeg_finish_compress(cinfo);
 *
 * jpeg_write_tables has the side effect of marking all tables written
 * (same as jpeg_suppress_tables(..., TRUE)).  Thus a subsequent start_compress
 * will not re-emit the tables unless it is passed write_all_tables=TRUE.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_write_tables(mut cinfo: j_compress_ptr) {
    if (*cinfo).global_state != CSTATE_START {
        (*(*cinfo).err).msg_code = JERR_BAD_STATE as c_int;
        (*(*cinfo).err).msg_parm.i[0usize] = (*cinfo).global_state;
        (*(*cinfo).err)
            .error_exit
            .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    (*(*cinfo).err)
        .reset_error_mgr
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    (*(*cinfo).dest)
        .init_destination
        .expect("non-null function pointer")(cinfo);
    jinit_marker_writer(cinfo);
    (*(*cinfo).marker)
        .write_tables_only
        .expect("non-null function pointer")(cinfo);
    (*(*cinfo).dest)
        .term_destination
        .expect("non-null function pointer")(cinfo);
}
