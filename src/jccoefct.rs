use libc;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::jround_up;
pub use crate::jpegint_h::jzero_far;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
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
pub use crate::jpeglib_h::C_MAX_BLOCKS_IN_MCU;
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
pub use crate::src::jchuff::c_derived_tbl;
pub use crate::src::jchuff::jpeg_make_c_derived_tbl;
pub use crate::src::jchuff::quantize_trellis;
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

pub type my_coef_ptr = *mut my_coef_controller;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_coef_controller {
    pub pub_0: crate::jpeglib_h::jpeg_c_coef_controller,
    pub iMCU_row_num: crate::jmorecfg_h::JDIMENSION,
    pub mcu_ctr: crate::jmorecfg_h::JDIMENSION,
    pub MCU_vert_offset: libc::c_int,
    pub MCU_rows_per_iMCU_row: libc::c_int,
    pub MCU_buffer: [crate::jpeglib_h::JBLOCKROW; 10],
    pub whole_image: [crate::jpeglib_h::jvirt_barray_ptr; 10],
    pub whole_image_uq: [crate::jpeglib_h::jvirt_barray_ptr; 10],
}

unsafe extern "C" fn start_iMCU_row(mut cinfo: crate::jpeglib_h::j_compress_ptr)
/* Reset within-iMCU-row counters for a new row */
{
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    /* In an interleaved scan, an MCU row is the same as an iMCU row.
     * In a noninterleaved scan, an iMCU row has v_samp_factor MCU rows.
     * But at the bottom of the image, process only what's left.
     */
    if (*cinfo).comps_in_scan > 1i32 {
        (*coef).MCU_rows_per_iMCU_row = 1i32
    } else if (*coef).iMCU_row_num <  (*cinfo).total_iMCU_rows - 1i32 as libc::c_uint {
        (*coef).MCU_rows_per_iMCU_row = (*(*cinfo).cur_comp_info[0]).v_samp_factor
    } else {
        (*coef).MCU_rows_per_iMCU_row = (*(*cinfo).cur_comp_info[0]).last_row_height
    }
    (*coef).mcu_ctr = 0i32 as crate::jmorecfg_h::JDIMENSION;
    (*coef).MCU_vert_offset = 0i32;
}
/*
 * Initialize for a processing pass.
 */

unsafe extern "C" fn start_pass_coef(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut pass_mode: crate::jpegint_h::J_BUF_MODE,
) {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    (*coef).iMCU_row_num = 0i32 as crate::jmorecfg_h::JDIMENSION;
    start_iMCU_row(cinfo);
    match pass_mode as libc::c_uint {
        0 => {
            if !(*coef).whole_image[0].is_null() {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_BUFFER_MODE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            (*coef).pub_0.compress_data = Some(
                compress_data
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_compress_ptr,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                    ) -> crate::jmorecfg_h::boolean,
            )
        }
        3 => {
            if (*coef).whole_image[0].is_null() {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_BUFFER_MODE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            (*coef).pub_0.compress_data = Some(
                compress_first_pass
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_compress_ptr,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                    ) -> crate::jmorecfg_h::boolean,
            )
        }
        2 => {
            if (*coef).whole_image[0].is_null() {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_BUFFER_MODE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            (*coef).pub_0.compress_data = Some(
                compress_output
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_compress_ptr,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                    ) -> crate::jmorecfg_h::boolean,
            )
        }
        4 => {
            if (*coef).whole_image[0].is_null() {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_BUFFER_MODE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            (*coef).pub_0.compress_data = Some(
                compress_trellis_pass
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_compress_ptr,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                    ) -> crate::jmorecfg_h::boolean,
            )
        }
        _ => {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_BUFFER_MODE as libc::c_int;
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
/* Forward declarations */
/*
 * Process some data in the single-pass case.
 * We process the equivalent of one fully interleaved MCU row ("iMCU" row)
 * per call, ie, v_samp_factor block rows for each component in the image.
 * Returns TRUE if the iMCU row is completed, FALSE if suspended.
 *
 * NB: input_buf contains a plane for each component in image,
 * which we index according to the component's SOF position.
 */

unsafe extern "C" fn compress_data(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
) -> crate::jmorecfg_h::boolean {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr; /* index of current MCU within row */
    let mut MCU_col_num: crate::jmorecfg_h::JDIMENSION = 0;
    let mut last_MCU_col: crate::jmorecfg_h::JDIMENSION =
        
        (*cinfo).MCUs_per_row - 1i32 as libc::c_uint;
    let mut last_iMCU_row: crate::jmorecfg_h::JDIMENSION =
        
        (*cinfo).total_iMCU_rows - 1i32 as libc::c_uint;
    let mut blkn: libc::c_int = 0;
    let mut bi: libc::c_int = 0;
    let mut ci: libc::c_int = 0;
    let mut yindex: libc::c_int = 0;
    let mut yoffset: libc::c_int = 0;
    let mut blockcnt: libc::c_int = 0;
    let mut ypos: crate::jmorecfg_h::JDIMENSION = 0;
    let mut xpos: crate::jmorecfg_h::JDIMENSION = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_component_info>();
    /* Loop to write as much as one whole iMCU row */
    yoffset = (*coef).MCU_vert_offset;
    while yoffset < (*coef).MCU_rows_per_iMCU_row {
        MCU_col_num = (*coef).mcu_ctr;
        while MCU_col_num <= last_MCU_col {
            /* Determine where data comes from in input_buf and do the DCT thing.
             * Each call on forward_DCT processes a horizontal row of DCT blocks
             * as wide as an MCU; we rely on having allocated the MCU_buffer[] blocks
             * sequentially.  Dummy blocks at the right or bottom edge are filled in
             * specially.  The data in them does not matter for image reconstruction,
             * so we fill them with values that will encode to the smallest amount of
             * data, viz: all zeroes in the AC entries, DC entries equal to previous
             * block's DC value.  (Thanks to Thomas Kinsman for this idea.)
             */
            blkn = 0i32; /* ypos == (yoffset+yindex) * DCTSIZE */
            ci = 0i32;
            while ci < (*cinfo).comps_in_scan {
                compptr = (*cinfo).cur_comp_info[ci as usize];
                blockcnt = if MCU_col_num < last_MCU_col {
                    (*compptr).MCU_width
                } else {
                    (*compptr).last_col_width
                };
                xpos =  MCU_col_num * (*compptr).MCU_sample_width as libc::c_uint;
                ypos = (yoffset * crate::jpeglib_h::DCTSIZE) as crate::jmorecfg_h::JDIMENSION;
                yindex = 0i32;
                while yindex < (*compptr).MCU_height {
                    if (*coef).iMCU_row_num < last_iMCU_row
                        || yoffset + yindex < (*compptr).last_row_height
                    {
                        Some(
                            (*(*cinfo).fdct)
                                .forward_DCT
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo,
                            compptr,
                            *input_buf.offset((*compptr).component_index as isize),
                            (*coef).MCU_buffer[blkn as usize],
                            ypos,
                            xpos,
                            blockcnt as crate::jmorecfg_h::JDIMENSION,
                            crate::stddef_h::NULL as crate::jpeglib_h::JBLOCKROW,
                        );
                        if blockcnt < (*compptr).MCU_width {
                            /* Create some dummy blocks at the right edge of the image. */
                            crate::jpegint_h::jzero_far(
                                (*coef).MCU_buffer[(blkn + blockcnt) as usize] as *mut libc::c_void,
                                ((*compptr).MCU_width - blockcnt) as libc::c_ulong *
    
                                    ::std::mem::size_of::<crate::jpeglib_h::JBLOCK>()
                                        as libc::c_ulong,
                            );
                            bi = blockcnt;
                            while bi < (*compptr).MCU_width {
                                (*(*coef).MCU_buffer[(blkn + bi) as usize].offset(0))[0] =
                                    (*(*coef).MCU_buffer[(blkn + bi - 1i32) as usize].offset(0))[0];
                                bi += 1
                            }
                        }
                    } else {
                        /* Create a row of dummy blocks at the bottom of the image. */
                        crate::jpegint_h::jzero_far(
                            (*coef).MCU_buffer[blkn as usize] as *mut libc::c_void,
                            (*compptr).MCU_width as libc::c_ulong *
    ::std::mem::size_of::<crate::jpeglib_h::JBLOCK>()
                                    as libc::c_ulong,
                        );
                        bi = 0i32;
                        while bi < (*compptr).MCU_width {
                            (*(*coef).MCU_buffer[(blkn + bi) as usize].offset(0))[0] =
                                (*(*coef).MCU_buffer[(blkn - 1i32) as usize].offset(0))[0];
                            bi += 1
                        }
                    }
                    blkn += (*compptr).MCU_width;
                    ypos = (ypos as libc::c_uint + crate::jpeglib_h::DCTSIZE as libc::c_uint)
                        as crate::jmorecfg_h::JDIMENSION
                        as crate::jmorecfg_h::JDIMENSION;
                    yindex += 1
                }
                ci += 1
            }
            /* Try to write the MCU.  In event of a suspension failure, we will
             * re-DCT the MCU on restart (a bit inefficient, could be fixed...)
             */
            if Some(
                (*(*cinfo).entropy)
                    .encode_mcu
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo, (*coef).MCU_buffer.as_mut_ptr()
            ) == 0
            {
                /* Suspension forced; update state counters and exit */
                (*coef).MCU_vert_offset = yoffset;
                (*coef).mcu_ctr = MCU_col_num;
                return crate::jmorecfg_h::FALSE;
            }
            MCU_col_num =  MCU_col_num + 1
        }
        /* Completed an MCU row, but perhaps not an iMCU row */
        (*coef).mcu_ctr = 0i32 as crate::jmorecfg_h::JDIMENSION;
        yoffset += 1
    }
    /* Completed the iMCU row, advance counters for next one */
    (*coef).iMCU_row_num =  (*coef).iMCU_row_num + 1;
    start_iMCU_row(cinfo);
    return crate::jmorecfg_h::TRUE;
}
/*
 * Process some data in the first pass of a multi-pass case.
 * We process the equivalent of one fully interleaved MCU row ("iMCU" row)
 * per call, ie, v_samp_factor block rows for each component in the image.
 * This amount of data is read from the source buffer, DCT'd and quantized,
 * and saved into the virtual arrays.  We also generate suitable dummy blocks
 * as needed at the right and lower edges.  (The dummy blocks are constructed
 * in the virtual arrays, which have been padded appropriately.)  This makes
 * it possible for subsequent passes not to worry about real vs. dummy blocks.
 *
 * We must also emit the data to the entropy encoder.  This is conveniently
 * done by calling compress_output() after we've loaded the current strip
 * of the virtual arrays.
 *
 * NB: input_buf contains a plane for each component in image.  All
 * components are DCT'd and loaded into the virtual arrays in this pass.
 * However, it may be that only a subset of the components are emitted to
 * the entropy encoder during this first pass; be careful about looking
 * at the scan-dependent variables (MCU dimensions, etc).
 */

unsafe extern "C" fn compress_first_pass(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
) -> crate::jmorecfg_h::boolean {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    let mut last_iMCU_row: crate::jmorecfg_h::JDIMENSION =
        
        (*cinfo).total_iMCU_rows - 1i32 as libc::c_uint;
    let mut blocks_across: crate::jmorecfg_h::JDIMENSION = 0;
    let mut MCUs_across: crate::jmorecfg_h::JDIMENSION = 0;
    let mut MCUindex: crate::jmorecfg_h::JDIMENSION = 0;
    let mut bi: libc::c_int = 0;
    let mut ci: libc::c_int = 0;
    let mut h_samp_factor: libc::c_int = 0;
    let mut block_row: libc::c_int = 0;
    let mut block_rows: libc::c_int = 0;
    let mut ndummy: libc::c_int = 0;
    let mut lastDC: crate::jmorecfg_h::JCOEF = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_component_info>();
    let mut buffer: crate::jpeglib_h::JBLOCKARRAY = ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCKROW>();
    let mut thisblockrow: crate::jpeglib_h::JBLOCKROW = ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCK>();
    let mut lastblockrow: crate::jpeglib_h::JBLOCKROW = ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCK>();
    let mut buffer_dst: crate::jpeglib_h::JBLOCKARRAY = ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCKROW>();
    ci = 0i32;
    compptr = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        /* Align the virtual buffer for this component. */
        buffer = Some(
            (*(*cinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            (*coef).whole_image[ci as usize],
            
            (*coef)
                .iMCU_row_num * (*compptr).v_samp_factor as libc::c_uint,
            (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
            crate::jmorecfg_h::TRUE,
        );
        buffer_dst = Some(
            (*(*cinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            (*coef).whole_image_uq[ci as usize],
            
            (*coef)
                .iMCU_row_num * (*compptr).v_samp_factor as libc::c_uint,
            (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
            crate::jmorecfg_h::TRUE,
        );
        /* Count non-dummy DCT block rows in this iMCU row. */
        if (*coef).iMCU_row_num < last_iMCU_row {
            block_rows = (*compptr).v_samp_factor
        } else {
            /* NB: can't use last_row_height here, since may not be set! */
            block_rows = ( (*compptr)
                .height_in_blocks % (*compptr).v_samp_factor as libc::c_uint)
                as libc::c_int;
            if block_rows == 0i32 {
                block_rows = (*compptr).v_samp_factor
            }
        }
        blocks_across = (*compptr).width_in_blocks;
        h_samp_factor = (*compptr).h_samp_factor;
        /* Count number of dummy blocks to be added at the right margin. */
        ndummy = ( blocks_across % h_samp_factor as libc::c_uint) as libc::c_int;
        if ndummy > 0i32 {
            ndummy = h_samp_factor - ndummy
        }
        /* Perform DCT for all non-dummy blocks in this iMCU row.  Each call
         * on forward_DCT processes a complete horizontal row of DCT blocks.
         */
        block_row = 0i32;
        while block_row < block_rows {
            thisblockrow = *buffer.offset(block_row as isize);
            Some(
                (*(*cinfo).fdct)
                    .forward_DCT
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo,
                compptr,
                *input_buf.offset(ci as isize),
                thisblockrow,
                (block_row * crate::jpeglib_h::DCTSIZE) as crate::jmorecfg_h::JDIMENSION,
                0i32 as crate::jmorecfg_h::JDIMENSION,
                blocks_across,
                *buffer_dst.offset(block_row as isize),
            );
            if ndummy > 0i32 {
                /* Create dummy blocks at the right edge of the image. */
                thisblockrow = thisblockrow.offset(blocks_across as isize); /* => first dummy block */
                crate::jpegint_h::jzero_far(
                    thisblockrow as *mut libc::c_void,
                    ndummy as libc::c_ulong *
    
                            ::std::mem::size_of::<crate::jpeglib_h::JBLOCK>() as libc::c_ulong,
                );
                lastDC = (*thisblockrow.offset(-1i32 as isize))[0];
                bi = 0i32;
                while bi < ndummy {
                    (*thisblockrow.offset(bi as isize))[0] = lastDC;
                    bi += 1
                }
            }
            block_row += 1
        }
        /* If at end of image, create dummy block rows as needed.
         * The tricky part here is that within each MCU, we want the DC values
         * of the dummy blocks to match the last real block's DC value.
         * This squeezes a few more bytes out of the resulting file...
         */
        if (*coef).iMCU_row_num == last_iMCU_row {
            blocks_across = (blocks_across as libc::c_uint + ndummy as libc::c_uint)
                as crate::jmorecfg_h::JDIMENSION
                as crate::jmorecfg_h::JDIMENSION; /* include lower right corner */
            MCUs_across =  blocks_across / h_samp_factor as libc::c_uint; /* advance to next MCU in row */
            block_row = block_rows;
            while block_row < (*compptr).v_samp_factor {
                thisblockrow = *buffer.offset(block_row as isize);
                lastblockrow = *buffer.offset((block_row - 1i32) as isize);
                crate::jpegint_h::jzero_far(
                    thisblockrow as *mut libc::c_void,
                    blocks_across as libc::c_ulong *
    
                            ::std::mem::size_of::<crate::jpeglib_h::JBLOCK>() as libc::c_ulong,
                );
                MCUindex = 0i32 as crate::jmorecfg_h::JDIMENSION;
                while MCUindex < MCUs_across {
                    lastDC = (*lastblockrow.offset((h_samp_factor - 1i32) as isize))[0];
                    bi = 0i32;
                    while bi < h_samp_factor {
                        (*thisblockrow.offset(bi as isize))[0] = lastDC;
                        bi += 1
                    }
                    thisblockrow = thisblockrow.offset(h_samp_factor as isize);
                    lastblockrow = lastblockrow.offset(h_samp_factor as isize);
                    MCUindex =  MCUindex + 1
                }
                block_row += 1
            }
        }
        ci += 1;
        compptr = compptr.offset(1)
    }
    /* NB: compress_output will increment iMCU_row_num if successful.
     * A suspension return will result in redoing all the work above next time.
     */
    /* Emit data to the entropy encoder, sharing code with subsequent passes */
    return compress_output(cinfo, input_buf);
}

unsafe extern "C" fn compress_trellis_pass(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
) -> crate::jmorecfg_h::boolean {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr;
    let mut last_iMCU_row: crate::jmorecfg_h::JDIMENSION =
        
        (*cinfo).total_iMCU_rows - 1i32 as libc::c_uint;
    let mut blocks_across: crate::jmorecfg_h::JDIMENSION = 0;
    let mut MCUs_across: crate::jmorecfg_h::JDIMENSION = 0;
    let mut MCUindex: crate::jmorecfg_h::JDIMENSION = 0;
    let mut bi: libc::c_int = 0;
    let mut ci: libc::c_int = 0;
    let mut h_samp_factor: libc::c_int = 0;
    let mut block_row: libc::c_int = 0;
    let mut block_rows: libc::c_int = 0;
    let mut ndummy: libc::c_int = 0;
    let mut lastDC: crate::jmorecfg_h::JCOEF = 0;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_component_info>();
    let mut buffer: crate::jpeglib_h::JBLOCKARRAY = ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCKROW>();
    let mut thisblockrow: crate::jpeglib_h::JBLOCKROW = ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCK>();
    let mut lastblockrow: crate::jpeglib_h::JBLOCKROW = ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCK>();
    let mut buffer_dst: crate::jpeglib_h::JBLOCKARRAY = ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCKROW>();
    ci = 0i32;
    while ci < (*cinfo).comps_in_scan {
        let mut dctbl_data: crate::src::jchuff::c_derived_tbl = crate::src::jchuff::c_derived_tbl {
            ehufco: [0; 256],
            ehufsi: [0; 256],
        };
        let mut dctbl: *mut crate::src::jchuff::c_derived_tbl = &mut dctbl_data;
        let mut actbl_data: crate::src::jchuff::c_derived_tbl = crate::src::jchuff::c_derived_tbl {
            ehufco: [0; 256],
            ehufsi: [0; 256],
        };
        let mut actbl: *mut crate::src::jchuff::c_derived_tbl = &mut actbl_data;
        compptr = (*cinfo).cur_comp_info[ci as usize];
        crate::src::jchuff::jpeg_make_c_derived_tbl(
            cinfo,
            crate::jmorecfg_h::TRUE,
            (*compptr).dc_tbl_no,
            &mut dctbl,
        );
        crate::src::jchuff::jpeg_make_c_derived_tbl(
            cinfo,
            crate::jmorecfg_h::FALSE,
            (*compptr).ac_tbl_no,
            &mut actbl,
        );
        /* Align the virtual buffer for this component. */
        buffer = Some(
            (*(*cinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            (*coef).whole_image[(*compptr).component_index as usize],
            
            (*coef)
                .iMCU_row_num * (*compptr).v_samp_factor as libc::c_uint,
            (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
            crate::jmorecfg_h::TRUE,
        );
        buffer_dst = Some(
            (*(*cinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            (*coef).whole_image_uq[(*compptr).component_index as usize],
            
            (*coef)
                .iMCU_row_num * (*compptr).v_samp_factor as libc::c_uint,
            (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
            crate::jmorecfg_h::TRUE,
        );
        /* Count non-dummy DCT block rows in this iMCU row. */
        if (*coef).iMCU_row_num < last_iMCU_row {
            block_rows = (*compptr).v_samp_factor
        } else {
            /* NB: can't use last_row_height here, since may not be set! */
            block_rows = ( (*compptr)
                .height_in_blocks % (*compptr).v_samp_factor as libc::c_uint)
                as libc::c_int;
            if block_rows == 0i32 {
                block_rows = (*compptr).v_samp_factor
            }
        }
        blocks_across = (*compptr).width_in_blocks;
        h_samp_factor = (*compptr).h_samp_factor;
        /* Count number of dummy blocks to be added at the right margin. */
        ndummy = ( blocks_across % h_samp_factor as libc::c_uint) as libc::c_int;
        if ndummy > 0i32 {
            ndummy = h_samp_factor - ndummy
        }
        lastDC = 0i32 as crate::jmorecfg_h::JCOEF;
        /* Perform DCT for all non-dummy blocks in this iMCU row.  Each call
         * on forward_DCT processes a complete horizontal row of DCT blocks.
         */
        block_row = 0i32;
        while block_row < block_rows {
            thisblockrow = *buffer.offset(block_row as isize);
            lastblockrow = if block_row > 0i32 {
                *buffer.offset((block_row - 1i32) as isize)
            } else {
                crate::stddef_h::NULL as crate::jpeglib_h::JBLOCKROW
            };
            crate::src::jchuff::quantize_trellis(
                cinfo,
                dctbl,
                actbl,
                thisblockrow,
                *buffer_dst.offset(block_row as isize),
                blocks_across,
                (*cinfo).quant_tbl_ptrs[(*compptr).quant_tbl_no as usize],
                (*(*cinfo).master).norm_src[(*compptr).quant_tbl_no as usize].as_mut_ptr(),
                (*(*cinfo).master).norm_coef[(*compptr).quant_tbl_no as usize].as_mut_ptr(),
                &mut lastDC,
                lastblockrow,
                *buffer_dst.offset((block_row - 1i32) as isize),
            );
            if ndummy > 0i32 {
                /* Create dummy blocks at the right edge of the image. */
                thisblockrow = thisblockrow.offset(blocks_across as isize); /* => first dummy block */
                crate::jpegint_h::jzero_far(
                    thisblockrow as *mut libc::c_void,
                    ndummy as libc::c_ulong *
    
                            ::std::mem::size_of::<crate::jpeglib_h::JBLOCK>() as libc::c_ulong,
                );
                lastDC = (*thisblockrow.offset(-1i32 as isize))[0];
                bi = 0i32;
                while bi < ndummy {
                    (*thisblockrow.offset(bi as isize))[0] = lastDC;
                    bi += 1
                }
            }
            block_row += 1
        }
        /* If at end of image, create dummy block rows as needed.
         * The tricky part here is that within each MCU, we want the DC values
         * of the dummy blocks to match the last real block's DC value.
         * This squeezes a few more bytes out of the resulting file...
         */
        if (*coef).iMCU_row_num == last_iMCU_row {
            blocks_across = (blocks_across as libc::c_uint + ndummy as libc::c_uint)
                as crate::jmorecfg_h::JDIMENSION
                as crate::jmorecfg_h::JDIMENSION; /* include lower right corner */
            MCUs_across =  blocks_across / h_samp_factor as libc::c_uint; /* advance to next MCU in row */
            block_row = block_rows;
            while block_row < (*compptr).v_samp_factor {
                thisblockrow = *buffer.offset(block_row as isize);
                lastblockrow = *buffer.offset((block_row - 1i32) as isize);
                crate::jpegint_h::jzero_far(
                    thisblockrow as *mut libc::c_void,
                    blocks_across as libc::c_ulong *
    
                            ::std::mem::size_of::<crate::jpeglib_h::JBLOCK>() as libc::c_ulong,
                );
                MCUindex = 0i32 as crate::jmorecfg_h::JDIMENSION;
                while MCUindex < MCUs_across {
                    lastDC = (*lastblockrow.offset((h_samp_factor - 1i32) as isize))[0];
                    bi = 0i32;
                    while bi < h_samp_factor {
                        (*thisblockrow.offset(bi as isize))[0] = lastDC;
                        bi += 1
                    }
                    thisblockrow = thisblockrow.offset(h_samp_factor as isize);
                    lastblockrow = lastblockrow.offset(h_samp_factor as isize);
                    MCUindex =  MCUindex + 1
                }
                block_row += 1
            }
        }
        ci += 1
    }
    /* NB: compress_output will increment iMCU_row_num if successful.
     * A suspension return will result in redoing all the work above next time.
     */
    /* Emit data to the entropy encoder, sharing code with subsequent passes */
    return compress_output(cinfo, input_buf);
}
/*
 * Process some data in subsequent passes of a multi-pass case.
 * We process the equivalent of one fully interleaved MCU row ("iMCU" row)
 * per call, ie, v_samp_factor block rows for each component in the scan.
 * The data is obtained from the virtual arrays and fed to the entropy coder.
 * Returns TRUE if the iMCU row is completed, FALSE if suspended.
 *
 * NB: input_buf is ignored; it is likely to be a NULL pointer.
 */

unsafe extern "C" fn compress_output(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPIMAGE,
) -> crate::jmorecfg_h::boolean {
    let mut coef: my_coef_ptr = (*cinfo).coef as my_coef_ptr; /* index of current MCU within row */
    let mut MCU_col_num: crate::jmorecfg_h::JDIMENSION = 0;
    let mut blkn: libc::c_int = 0;
    let mut ci: libc::c_int = 0;
    let mut xindex: libc::c_int = 0;
    let mut yindex: libc::c_int = 0;
    let mut yoffset: libc::c_int = 0;
    let mut start_col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut buffer: [crate::jpeglib_h::JBLOCKARRAY; 4] = [::std::ptr::null_mut::< crate::jpeglib_h::JBLOCKROW>(); 4];
    let mut buffer_ptr: crate::jpeglib_h::JBLOCKROW = ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCK>();
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_component_info>();
    /* Align the virtual buffers for the components used in this scan.
     * NB: during first pass, this is safe only because the buffers will
     * already be aligned properly, so jmemmgr.c won't need to do any I/O.
     */
    ci = 0i32;
    while ci < (*cinfo).comps_in_scan {
        compptr = (*cinfo).cur_comp_info[ci as usize];
        buffer[ci as usize] = Some(
            (*(*cinfo).mem)
                .access_virt_barray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            (*coef).whole_image[(*compptr).component_index as usize],
            
            (*coef)
                .iMCU_row_num * (*compptr).v_samp_factor as libc::c_uint,
            (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
            crate::jmorecfg_h::FALSE,
        );
        ci += 1
    }
    /* Loop to process one whole iMCU row */
    yoffset = (*coef).MCU_vert_offset;
    while yoffset < (*coef).MCU_rows_per_iMCU_row {
        MCU_col_num = (*coef).mcu_ctr;
        while MCU_col_num < (*cinfo).MCUs_per_row {
            /* Construct list of pointers to DCT blocks belonging to this MCU */
            blkn = 0i32; /* index of current DCT block within MCU */
            ci = 0i32;
            while ci < (*cinfo).comps_in_scan {
                compptr = (*cinfo).cur_comp_info[ci as usize];
                start_col =  MCU_col_num * (*compptr).MCU_width as libc::c_uint;
                yindex = 0i32;
                while yindex < (*compptr).MCU_height {
                    buffer_ptr = (*buffer[ci as usize].offset((yindex + yoffset) as isize))
                        .offset(start_col as isize);
                    xindex = 0i32;
                    while xindex < (*compptr).MCU_width {
                        let fresh0 = buffer_ptr;
                        buffer_ptr = buffer_ptr.offset(1);
                        let fresh1 = blkn;
                        blkn = blkn + 1;
                        (*coef).MCU_buffer[fresh1 as usize] = fresh0;
                        xindex += 1
                    }
                    yindex += 1
                }
                ci += 1
            }
            /* Try to write the MCU. */
            if Some(
                (*(*cinfo).entropy)
                    .encode_mcu
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo, (*coef).MCU_buffer.as_mut_ptr()
            ) == 0
            {
                /* Suspension forced; update state counters and exit */
                (*coef).MCU_vert_offset = yoffset;
                (*coef).mcu_ctr = MCU_col_num;
                return crate::jmorecfg_h::FALSE;
            }
            MCU_col_num =  MCU_col_num + 1
        }
        /* Completed an MCU row, but perhaps not an iMCU row */
        (*coef).mcu_ctr = 0i32 as crate::jmorecfg_h::JDIMENSION;
        yoffset += 1
    }
    /* Completed the iMCU row, advance counters for next one */
    (*coef).iMCU_row_num =  (*coef).iMCU_row_num + 1;
    start_iMCU_row(cinfo);
    return crate::jmorecfg_h::TRUE;
}
/* FULL_COEF_BUFFER_SUPPORTED */
/*
 * Initialize coefficient buffer controller.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_c_coef_controller(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut need_full_buffer: crate::jmorecfg_h::boolean,
) {
    let mut coef: my_coef_ptr = ::std::ptr::null_mut::< my_coef_controller>();
    coef = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<my_coef_controller>() as libc::c_ulong,
    ) as my_coef_ptr;
    (*cinfo).coef = coef as *mut crate::jpeglib_h::jpeg_c_coef_controller;
    (*coef).pub_0.start_pass = Some(
        start_pass_coef
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_compress_ptr,
                _: crate::jpegint_h::J_BUF_MODE,
            ) -> (),
    );
    /* Create the coefficient buffer. */
    if need_full_buffer != 0 {
        /* Allocate a full-image virtual array for each component, */
        /* padded to a multiple of samp_factor DCT blocks in each direction. */
        let mut ci: libc::c_int = 0;
        let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
            ::std::ptr::null_mut::< crate::jpeglib_h::jpeg_component_info>();
        ci = 0i32;
        compptr = (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
            (*coef).whole_image[ci as usize] = Some(
                (*(*cinfo).mem)
                    .request_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr,
                crate::jpeglib_h::JPOOL_IMAGE,
                crate::jmorecfg_h::FALSE,
                crate::jpegint_h::jround_up(
                    (*compptr).width_in_blocks as libc::c_long,
                    (*compptr).h_samp_factor as libc::c_long,
                ) as crate::jmorecfg_h::JDIMENSION,
                crate::jpegint_h::jround_up(
                    (*compptr).height_in_blocks as libc::c_long,
                    (*compptr).v_samp_factor as libc::c_long,
                ) as crate::jmorecfg_h::JDIMENSION,
                (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
            );
            (*coef).whole_image_uq[ci as usize] = Some(
                (*(*cinfo).mem)
                    .request_virt_barray
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr,
                crate::jpeglib_h::JPOOL_IMAGE,
                crate::jmorecfg_h::FALSE,
                crate::jpegint_h::jround_up(
                    (*compptr).width_in_blocks as libc::c_long,
                    (*compptr).h_samp_factor as libc::c_long,
                ) as crate::jmorecfg_h::JDIMENSION,
                crate::jpegint_h::jround_up(
                    (*compptr).height_in_blocks as libc::c_long,
                    (*compptr).v_samp_factor as libc::c_long,
                ) as crate::jmorecfg_h::JDIMENSION,
                (*compptr).v_samp_factor as crate::jmorecfg_h::JDIMENSION,
            );
            ci += 1;
            compptr = compptr.offset(1)
        }
    } else {
        /* We only need a single-MCU buffer. */
        let mut buffer: crate::jpeglib_h::JBLOCKROW = ::std::ptr::null_mut::< crate::jpeglib_h::JBLOCK>();
        let mut i: libc::c_int = 0;
        buffer = Some(
            (*(*cinfo).mem)
                .alloc_large
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            crate::jpeglib_h::C_MAX_BLOCKS_IN_MCU as libc::c_ulong *
    ::std::mem::size_of::<crate::jpeglib_h::JBLOCK>() as libc::c_ulong,
        ) as crate::jpeglib_h::JBLOCKROW;
        i = 0i32;
        while i < crate::jpeglib_h::C_MAX_BLOCKS_IN_MCU {
            (*coef).MCU_buffer[i as usize] = buffer.offset(i as isize);
            i += 1
        }
        (*coef).whole_image[0] = crate::stddef_h::NULL as crate::jpeglib_h::jvirt_barray_ptr
    };
}
