pub use super::jerror::{
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
    boolean, FALSE, JCOEF, JDIMENSION, JOCTET, JSAMPLE, TRUE, UINT16, UINT8,
};
pub use crate::jpegint_h::{
    jpeg_natural_order, JBUF_CRANK_DEST, JBUF_PASS_THRU, JBUF_REQUANT, JBUF_SAVE_AND_PASS,
    JBUF_SAVE_SOURCE, J_BUF_MODE,
};
pub use crate::jpeglib_h::{
    j_common_ptr, j_compress_ptr, jpeg_c_coef_controller, jpeg_c_main_controller,
    jpeg_c_prep_controller, jpeg_color_converter, jpeg_common_struct, jpeg_comp_master,
    jpeg_component_info, jpeg_compress_struct, jpeg_destination_mgr, jpeg_downsampler,
    jpeg_entropy_encoder, jpeg_error_mgr, jpeg_forward_dct, jpeg_marker_writer, jpeg_memory_mgr,
    jpeg_progress_mgr, jpeg_scan_info, jvirt_barray_control, jvirt_barray_ptr,
    jvirt_sarray_control, jvirt_sarray_ptr, C2RustUnnamed_1, C2RustUnnamed_2, JCS_YCbCr, DCTSIZE2,
    JBLOCK, JBLOCKARRAY, JBLOCKROW, JCP_FASTEST, JCP_MAX_COMPRESSION, JCS_CMYK, JCS_EXT_ABGR,
    JCS_EXT_ARGB, JCS_EXT_BGR, JCS_EXT_BGRA, JCS_EXT_BGRX, JCS_EXT_RGB, JCS_EXT_RGBA, JCS_EXT_RGBX,
    JCS_EXT_XBGR, JCS_EXT_XRGB, JCS_GRAYSCALE, JCS_RGB, JCS_RGB565, JCS_UNKNOWN, JCS_YCCK,
    JDCT_FLOAT, JDCT_IFAST, JDCT_ISLOW, JHUFF_TBL, JPOOL_IMAGE, JQUANT_TBL, JSAMPARRAY, JSAMPIMAGE,
    JSAMPROW, J_COLOR_SPACE, J_DCT_METHOD, NUM_HUFF_TBLS, NUM_QUANT_TBLS,
};
pub use crate::stddef_h::{size_t, NULL};
use libc::{self, c_int, c_long, c_uint, c_ulong};

pub type my_marker_ptr = *mut my_marker_writer;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_marker_writer {
    pub pub_0: jpeg_marker_writer,
    pub last_restart_interval: c_uint,
}

pub type JPEG_MARKER = c_uint;

pub const M_ERROR: JPEG_MARKER = 256;

pub const M_TEM: JPEG_MARKER = 1;

pub const M_COM: JPEG_MARKER = 254;

pub const M_JPG13: JPEG_MARKER = 253;

pub const M_JPG0: JPEG_MARKER = 240;

pub const M_APP15: JPEG_MARKER = 239;

pub const M_APP14: JPEG_MARKER = 238;

pub const M_APP13: JPEG_MARKER = 237;

pub const M_APP12: JPEG_MARKER = 236;

pub const M_APP11: JPEG_MARKER = 235;

pub const M_APP10: JPEG_MARKER = 234;

pub const M_APP9: JPEG_MARKER = 233;

pub const M_APP8: JPEG_MARKER = 232;

pub const M_APP7: JPEG_MARKER = 231;

pub const M_APP6: JPEG_MARKER = 230;

pub const M_APP5: JPEG_MARKER = 229;

pub const M_APP4: JPEG_MARKER = 228;

pub const M_APP3: JPEG_MARKER = 227;

pub const M_APP2: JPEG_MARKER = 226;

pub const M_APP1: JPEG_MARKER = 225;

pub const M_APP0: JPEG_MARKER = 224;

pub const M_EXP: JPEG_MARKER = 223;

pub const M_DHP: JPEG_MARKER = 222;

pub const M_DRI: JPEG_MARKER = 221;

pub const M_DNL: JPEG_MARKER = 220;

pub const M_DQT: JPEG_MARKER = 219;

pub const M_SOS: JPEG_MARKER = 218;

pub const M_EOI: JPEG_MARKER = 217;

pub const M_SOI: JPEG_MARKER = 216;

pub const M_RST7: JPEG_MARKER = 215;

pub const M_RST6: JPEG_MARKER = 214;

pub const M_RST5: JPEG_MARKER = 213;

pub const M_RST4: JPEG_MARKER = 212;

pub const M_RST3: JPEG_MARKER = 211;

pub const M_RST2: JPEG_MARKER = 210;

pub const M_RST1: JPEG_MARKER = 209;

pub const M_RST0: JPEG_MARKER = 208;

pub const M_DAC: JPEG_MARKER = 204;

pub const M_DHT: JPEG_MARKER = 196;

pub const M_SOF15: JPEG_MARKER = 207;

pub const M_SOF14: JPEG_MARKER = 206;

pub const M_SOF13: JPEG_MARKER = 205;

pub const M_SOF11: JPEG_MARKER = 203;

pub const M_SOF10: JPEG_MARKER = 202;

pub const M_SOF9: JPEG_MARKER = 201;

pub const M_JPG: JPEG_MARKER = 200;

pub const M_SOF7: JPEG_MARKER = 199;

pub const M_SOF6: JPEG_MARKER = 198;

pub const M_SOF5: JPEG_MARKER = 197;

pub const M_SOF3: JPEG_MARKER = 195;

pub const M_SOF2: JPEG_MARKER = 194;

pub const M_SOF1: JPEG_MARKER = 193;

pub const M_SOF0: JPEG_MARKER = 192;
/*
 * Basic output routines.
 *
 * Note that we do not support suspension while writing a marker.
 * Therefore, an application using suspension must ensure that there is
 * enough buffer space for the initial markers (typ. 600-700 bytes) before
 * calling jpeg_start_compress, and enough space to write the trailing EOI
 * (a few bytes) before calling jpeg_finish_compress.  Multipass compression
 * modes are not supported at all with suspension, so those two are the only
 * points where markers will be written.
 */

unsafe extern "C" fn emit_byte(mut cinfo: j_compress_ptr, mut val: c_int)
/* Emit a byte */
{
    let mut dest: *mut jpeg_destination_mgr = (*cinfo).dest;
    let fresh0 = (*dest).next_output_byte;
    (*dest).next_output_byte = (*dest).next_output_byte.offset(1);
    *fresh0 = val as JOCTET;
    (*dest).free_in_buffer = (*dest).free_in_buffer - 1;
    if (*dest).free_in_buffer == 0u64 {
        if Some(
            (*dest)
                .empty_output_buffer
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo)
            == 0
        {
            (*(*cinfo).err).msg_code = super::jerror::JERR_CANT_SUSPEND as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr);
        }
    };
}

unsafe extern "C" fn emit_marker(mut cinfo: j_compress_ptr, mut mark: JPEG_MARKER)
/* Emit a marker code */
{
    emit_byte(cinfo, 0xffi32);
    emit_byte(cinfo, mark as c_int);
}

unsafe extern "C" fn emit_2bytes(mut cinfo: j_compress_ptr, mut value: c_int)
/* Emit a 2-byte integer; these are always MSB first in JPEG files */
{
    emit_byte(cinfo, value >> 8i32 & 0xffi32);
    emit_byte(cinfo, value & 0xffi32);
}
/*
 * Routines to write specific marker types.
 */

unsafe extern "C" fn emit_dqt(mut cinfo: j_compress_ptr, mut index: c_int) -> c_int
/* Emit a DQT marker */
/* Returns the precision used (0 = 8bits, 1 = 16bits) for baseline checking */ {
    let mut qtbl: *mut JQUANT_TBL = (*cinfo).quant_tbl_ptrs[index as usize];

    if qtbl.is_null() {
        (*(*cinfo).err).msg_code = super::jerror::JERR_NO_QUANT_TABLE as c_int;
        (*(*cinfo).err).msg_parm.i[0] = index;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }

    let mut prec: c_int = 0i32;
    let mut i: c_int = 0i32;
    while i < DCTSIZE2 {
        if (*qtbl).quantval[i as usize] as c_int > 255i32 {
            prec = 1i32
        }
        i += 1
    }
    if (*qtbl).sent_table == 0 {
        emit_marker(cinfo, M_DQT);
        emit_2bytes(
            cinfo,
            if prec != 0 {
                (DCTSIZE2 * 2i32 + 1i32) + 2i32
            } else {
                (DCTSIZE2 + 1i32) + 2i32
            },
        );
        emit_byte(cinfo, index + (prec << 4i32));
        i = 0i32;
        while i < DCTSIZE2 {
            /* The table entries must be emitted in zigzag order. */
            let mut qval: c_uint = (*qtbl).quantval
                [*jpeg_natural_order.as_ptr().offset(i as isize) as usize]
                as c_uint;
            if prec != 0 {
                emit_byte(cinfo, (qval >> 8i32) as c_int);
            }
            emit_byte(cinfo, (qval & 0xffu32) as c_int);
            i += 1
        }
        (*qtbl).sent_table = TRUE
    }
    return prec;
}

unsafe extern "C" fn emit_multi_dqt(mut cinfo: j_compress_ptr) -> c_int
/* Emits a DQT marker containing all quantization tables */
/* Returns number of emitted 16-bit tables, or -1 for failed for baseline checking. */ {
    let mut prec: [c_int; 10] = [0; 10];
    let mut fin_prec: c_int = 0i32;
    let mut size: c_int = 0i32;
    if (*(*cinfo).master).compress_profile == JCP_FASTEST as c_int {
        return -1i32;
    }
    let mut ci: c_int = 0i32;
    while ci < (*cinfo).num_components {
        let mut tbl_num: c_int = (*(*cinfo).comp_info.offset(ci as isize)).quant_tbl_no;

        let mut qtbl: *mut JQUANT_TBL = (*cinfo).quant_tbl_ptrs[tbl_num as usize];
        if qtbl.is_null() || (*qtbl).sent_table == TRUE {
            return -1i32;
        }
        prec[ci as usize] = 0i32;
        let mut i: c_int = 0i32;
        while i < DCTSIZE2 {
            prec[ci as usize] = (prec[ci as usize]
                + ((*qtbl).quantval[i as usize] as c_int > 255i32) as c_int
                != 0) as c_int;
            i += 1
        }
        fin_prec += prec[ci as usize];
        ci += 1
    }
    emit_marker(cinfo, M_DQT);
    ci = 0i32;
    while ci < (*cinfo).num_components {
        let mut seen: [c_int; 10] = [0i32, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut tbl_num_0: c_int = (*(*cinfo).comp_info.offset(ci as isize)).quant_tbl_no;
        if seen[tbl_num_0 as usize] == 0 {
            size += DCTSIZE2 * (prec[ci as usize] + 1i32) + 1i32;
            seen[tbl_num_0 as usize] = 1i32
        }
        ci += 1
    }
    size += 2i32;
    emit_2bytes(cinfo, size);
    ci = 0i32;
    while ci < (*cinfo).num_components {
        let mut tbl_num_1: c_int = (*(*cinfo).comp_info.offset(ci as isize)).quant_tbl_no;

        let mut qtbl_0: *mut JQUANT_TBL = (*cinfo).quant_tbl_ptrs[tbl_num_1 as usize];
        if !((*qtbl_0).sent_table == TRUE) {
            emit_byte(cinfo, tbl_num_1 + (prec[ci as usize] << 4i32));
            let mut i_0: c_int = 0i32;
            while i_0 < DCTSIZE2 {
                let mut qval: c_uint = (*qtbl_0).quantval
                    [*jpeg_natural_order.as_ptr().offset(i_0 as isize) as usize]
                    as c_uint;
                if prec[ci as usize] != 0 {
                    emit_byte(cinfo, (qval >> 8i32) as c_int);
                }
                emit_byte(cinfo, (qval & 0xffu32) as c_int);
                i_0 += 1
            }
            (*qtbl_0).sent_table = TRUE
        }
        ci += 1
    }
    return fin_prec;
}

unsafe extern "C" fn emit_dht(mut cinfo: j_compress_ptr, mut index: c_int, mut is_ac: boolean)
/* Emit a DHT marker */
{
    let mut htbl: *mut JHUFF_TBL = ::std::ptr::null_mut::<JHUFF_TBL>();
    if is_ac != 0 {
        htbl = (*cinfo).ac_huff_tbl_ptrs[index as usize];
        index += 0x10i32
    /* output index has AC bit set */
    } else {
        htbl = (*cinfo).dc_huff_tbl_ptrs[index as usize]
    }
    if htbl.is_null() {
        (*(*cinfo).err).msg_code = super::jerror::JERR_NO_HUFF_TABLE as c_int;
        (*(*cinfo).err).msg_parm.i[0] = index;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    if (*htbl).sent_table == 0 {
        emit_marker(cinfo, M_DHT);

        let mut length: c_int = 0i32;
        let mut i: c_int = 1i32;
        while i <= 16i32 {
            length += (*htbl).bits[i as usize] as c_int;
            i += 1
        }
        emit_2bytes(cinfo, length + 2i32 + 1i32 + 16i32);
        emit_byte(cinfo, index);
        i = 1i32;
        while i <= 16i32 {
            emit_byte(cinfo, (*htbl).bits[i as usize] as c_int);
            i += 1
        }
        i = 0i32;
        while i < length {
            emit_byte(cinfo, (*htbl).huffval[i as usize] as c_int);
            i += 1
        }
        (*htbl).sent_table = TRUE
    };
}

unsafe extern "C" fn emit_multi_dht(mut cinfo: j_compress_ptr) -> boolean
/* Emit all DHT markers */
/* Returns FALSE on failure, TRUE otherwise. */ {
    let mut j: c_int = 0;
    let mut length: c_int = 2i32;
    let mut dclens: [c_int; 4] = [0i32, 0, 0, 0];
    let mut aclens: [c_int; 4] = [0i32, 0, 0, 0];
    let mut dcseen: [*mut JHUFF_TBL; 4] = [
        NULL as *mut JHUFF_TBL,
        ::std::ptr::null_mut::<JHUFF_TBL>(),
        ::std::ptr::null_mut::<JHUFF_TBL>(),
        ::std::ptr::null_mut::<JHUFF_TBL>(),
    ];
    let mut acseen: [*mut JHUFF_TBL; 4] = [
        NULL as *mut JHUFF_TBL,
        ::std::ptr::null_mut::<JHUFF_TBL>(),
        ::std::ptr::null_mut::<JHUFF_TBL>(),
        ::std::ptr::null_mut::<JHUFF_TBL>(),
    ];
    if (*(*cinfo).master).compress_profile == JCP_FASTEST as c_int {
        return 0i32;
    }

    let mut i: c_int = 0i32;
    while i < (*cinfo).comps_in_scan {
        let mut current_block_23: u64;
        let mut seen: c_int = 0i32;
        let mut compptr: *mut jpeg_component_info = (*cinfo).cur_comp_info[i as usize];
        let mut dcidx: c_int = (*compptr).dc_tbl_no;
        let mut acidx: c_int = (*compptr).ac_tbl_no;
        let mut dctbl: *mut JHUFF_TBL = (*cinfo).dc_huff_tbl_ptrs[dcidx as usize];
        let mut actbl: *mut JHUFF_TBL = (*cinfo).ac_huff_tbl_ptrs[acidx as usize];

        /* Handle DC table lenghts */
        if (*cinfo).Ss == 0i32 && (*cinfo).Ah == 0i32 {
            if dctbl.is_null() {
                (*(*cinfo).err).msg_code = super::jerror::JERR_NO_HUFF_TABLE as c_int;
                (*(*cinfo).err).msg_parm.i[0] = dcidx;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo as j_common_ptr);
            }
            if (*dctbl).sent_table != 0 {
                current_block_23 = 11875828834189669668;
            } else {
                j = 0i32;
                while j < NUM_HUFF_TBLS {
                    seen += (dctbl == dcseen[j as usize]) as c_int;
                    j += 1
                }
                if seen != 0 {
                    current_block_23 = 11875828834189669668;
                } else {
                    dcseen[i as usize] = dctbl;
                    j = 1i32;
                    while j <= 16i32 {
                        dclens[i as usize] += (*dctbl).bits[j as usize] as c_int;
                        j += 1
                    }
                    length += dclens[i as usize] + 16i32 + 1i32;
                    current_block_23 = 11194104282611034094;
                }
            }
        } else {
            current_block_23 = 11194104282611034094;
        }
        match current_block_23 {
            11194104282611034094 =>
            /* Handle AC table lengths */
            {
                if (*cinfo).Se != 0 {
                    if actbl.is_null() {
                        (*(*cinfo).err).msg_code = super::jerror::JERR_NO_HUFF_TABLE as c_int;
                        (*(*cinfo).err).msg_parm.i[0] = acidx + 0x10i32;
                        Some(
                            (*(*cinfo).err)
                                .error_exit
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as j_common_ptr
                        );
                    }
                    if !((*actbl).sent_table != 0) {
                        seen = 0i32;
                        j = 0i32;
                        while j < NUM_HUFF_TBLS {
                            seen += (actbl == acseen[j as usize]) as c_int;
                            j += 1
                        }
                        if !(seen != 0) {
                            acseen[i as usize] = actbl;
                            j = 1i32;
                            while j <= 16i32 {
                                aclens[i as usize] += (*actbl).bits[j as usize] as c_int;
                                j += 1
                            }
                            length += aclens[i as usize] + 16i32 + 1i32
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1
    }
    /* Make sure we can fit it all into one DHT marker */
    if length > (1i32 << 16i32) - 1i32 {
        return FALSE;
    }
    emit_marker(cinfo, M_DHT);
    emit_2bytes(cinfo, length);
    i = 0i32;
    while i < (*cinfo).comps_in_scan {
        let mut compptr_0: *mut jpeg_component_info = (*cinfo).cur_comp_info[i as usize];
        let mut dcidx_0: c_int = (*compptr_0).dc_tbl_no;
        let mut acidx_0: c_int = (*compptr_0).ac_tbl_no;
        let mut dctbl_0: *mut JHUFF_TBL = (*cinfo).dc_huff_tbl_ptrs[dcidx_0 as usize];
        let mut actbl_0: *mut JHUFF_TBL = (*cinfo).ac_huff_tbl_ptrs[acidx_0 as usize];
        acidx_0 += 0x10i32;
        /* DC */
        if (*cinfo).Ss == 0i32 && (*cinfo).Ah == 0i32 && (*dctbl_0).sent_table == 0 {
            emit_byte(cinfo, dcidx_0);
            j = 1i32;
            while j <= 16i32 {
                emit_byte(cinfo, (*dctbl_0).bits[j as usize] as c_int);
                j += 1
            }
            j = 0i32;
            while j < dclens[i as usize] {
                emit_byte(cinfo, (*dctbl_0).huffval[j as usize] as c_int);
                j += 1
            }
            (*dctbl_0).sent_table = TRUE
        }
        if (*cinfo).Se != 0 && (*actbl_0).sent_table == 0 {
            emit_byte(cinfo, acidx_0);
            j = 1i32;
            while j <= 16i32 {
                emit_byte(cinfo, (*actbl_0).bits[j as usize] as c_int);
                j += 1
            }
            j = 0i32;
            while j < aclens[i as usize] {
                emit_byte(cinfo, (*actbl_0).huffval[j as usize] as c_int);
                j += 1
            }
            (*actbl_0).sent_table = TRUE
        }
        i += 1
    }
    return TRUE;
}

unsafe extern "C" fn emit_dac(mut _cinfo: j_compress_ptr)
/* Emit a DAC marker */
/* Since the useful info is so small, we want to emit all the tables in */
/* one DAC marker.  Therefore this routine does its own scan of the table. */
{
    /* C_ARITH_CODING_SUPPORTED */
}

unsafe extern "C" fn emit_dri(mut cinfo: j_compress_ptr)
/* Emit a DRI marker */
{
    emit_marker(cinfo, M_DRI); /* fixed length */
    emit_2bytes(cinfo, 4i32);
    emit_2bytes(cinfo, (*cinfo).restart_interval as c_int);
}

unsafe extern "C" fn emit_sof(mut cinfo: j_compress_ptr, mut code: JPEG_MARKER)
/* Emit a SOF marker */
{
    emit_marker(cinfo, code);
    emit_2bytes(cinfo, 3i32 * (*cinfo).num_components + 2i32 + 5i32 + 1i32);
    /* Make sure image isn't bigger than SOF field can handle */
    if (*cinfo).image_height as c_long > 65535i64 || (*cinfo).image_width as c_long > 65535i64 {
        (*(*cinfo).err).msg_code = super::jerror::JERR_IMAGE_TOO_BIG as c_int;
        (*(*cinfo).err).msg_parm.i[0] = 65535i32;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    emit_byte(cinfo, (*cinfo).data_precision);
    emit_2bytes(cinfo, (*cinfo).image_height as c_int);
    emit_2bytes(cinfo, (*cinfo).image_width as c_int);
    emit_byte(cinfo, (*cinfo).num_components);

    let mut ci: c_int = 0i32;
    let mut compptr: *mut jpeg_component_info = (*cinfo).comp_info;
    while ci < (*cinfo).num_components {
        emit_byte(cinfo, (*compptr).component_id);
        emit_byte(
            cinfo,
            ((*compptr).h_samp_factor << 4i32) + (*compptr).v_samp_factor,
        );
        emit_byte(cinfo, (*compptr).quant_tbl_no);
        ci += 1;
        compptr = compptr.offset(1)
    }
}

unsafe extern "C" fn emit_sos(mut cinfo: j_compress_ptr)
/* Emit a SOS marker */
{
    emit_marker(cinfo, M_SOS);
    emit_2bytes(cinfo, 2i32 * (*cinfo).comps_in_scan + 2i32 + 1i32 + 3i32);
    emit_byte(cinfo, (*cinfo).comps_in_scan);
    let mut i: c_int = 0i32;
    while i < (*cinfo).comps_in_scan {
        let mut compptr: *mut jpeg_component_info = (*cinfo).cur_comp_info[i as usize];
        emit_byte(cinfo, (*compptr).component_id);
        /* We emit 0 for unused field(s); this is recommended by the P&M text
         * but does not seem to be specified in the standard.
         */

        let mut td: c_int = if (*cinfo).Ss == 0i32 && (*cinfo).Ah == 0i32 {
            (*compptr).dc_tbl_no
        } else {
            0i32
        };
        let mut ta: c_int = if (*cinfo).Se != 0 {
            (*compptr).ac_tbl_no
        } else {
            0i32
        };
        emit_byte(cinfo, (td << 4i32) + ta);
        i += 1
    }
    emit_byte(cinfo, (*cinfo).Ss);
    emit_byte(cinfo, (*cinfo).Se);
    emit_byte(cinfo, ((*cinfo).Ah << 4i32) + (*cinfo).Al);
}

unsafe extern "C" fn emit_jfif_app0(mut cinfo: j_compress_ptr)
/* Emit a JFIF-compliant APP0 marker */
{
    /*
     * Length of APP0 block       (2 bytes)
     * Block ID                   (4 bytes - ASCII "JFIF")
     * Zero byte                  (1 byte to terminate the ID string)
     * Version Major, Minor       (2 bytes - major first)
     * Units                      (1 byte - 0x00 = none, 0x01 = inch, 0x02 = cm)
     * Xdpu                       (2 bytes - dots per unit horizontal)
     * Ydpu                       (2 bytes - dots per unit vertical)
     * Thumbnail X size           (1 byte)
     * Thumbnail Y size           (1 byte)
     */
    emit_marker(cinfo, M_APP0); /* length */
    emit_2bytes(
        cinfo,
        2i32 + 4i32 + 1i32 + 2i32 + 1i32 + 2i32 + 2i32 + 1i32 + 1i32,
    ); /* Identifier: ASCII "JFIF" */
    emit_byte(cinfo, 0x4ai32); /* Version fields */
    emit_byte(cinfo, 0x46i32); /* Pixel size information */
    emit_byte(cinfo, 0x49i32); /* No thumbnail image */
    emit_byte(cinfo, 0x46i32);
    emit_byte(cinfo, 0i32);
    emit_byte(cinfo, (*cinfo).JFIF_major_version as c_int);
    emit_byte(cinfo, (*cinfo).JFIF_minor_version as c_int);
    emit_byte(cinfo, (*cinfo).density_unit as c_int);
    emit_2bytes(cinfo, (*cinfo).X_density as c_int);
    emit_2bytes(cinfo, (*cinfo).Y_density as c_int);
    emit_byte(cinfo, 0i32);
    emit_byte(cinfo, 0i32);
}

unsafe extern "C" fn emit_adobe_app14(mut cinfo: j_compress_ptr)
/* Emit an Adobe APP14 marker */
{
    /*
     * Length of APP14 block      (2 bytes)
     * Block ID                   (5 bytes - ASCII "Adobe")
     * Version Number             (2 bytes - currently 100)
     * Flags0                     (2 bytes - currently 0)
     * Flags1                     (2 bytes - currently 0)
     * Color transform            (1 byte)
     *
     * Although Adobe TN 5116 mentions Version = 101, all the Adobe files
     * now in circulation seem to use Version = 100, so that's what we write.
     *
     * We write the color transform byte as 1 if the JPEG color space is
     * YCbCr, 2 if it's YCCK, 0 otherwise.  Adobe's definition has to do with
     * whether the encoder performed a transformation, which is pretty useless.
     */
    emit_marker(cinfo, M_APP14); /* length */
    emit_2bytes(cinfo, 2i32 + 5i32 + 2i32 + 2i32 + 2i32 + 1i32); /* Identifier: ASCII "Adobe" */
    emit_byte(cinfo, 0x41i32); /* Version */
    emit_byte(cinfo, 0x64i32); /* Flags0 */
    emit_byte(cinfo, 0x6fi32); /* Flags1 */
    emit_byte(cinfo, 0x62i32); /* Color transform = 1 */
    emit_byte(cinfo, 0x65i32); /* Color transform = 2 */
    emit_2bytes(cinfo, 100i32); /* Color transform = 0 */
    emit_2bytes(cinfo, 0i32);
    emit_2bytes(cinfo, 0i32);
    match (*cinfo).jpeg_color_space {
        3 => {
            emit_byte(cinfo, 1i32);
        }
        5 => {
            emit_byte(cinfo, 2i32);
        }
        _ => {
            emit_byte(cinfo, 0i32);
        }
    };
}
/*
 * These routines allow writing an arbitrary marker with parameters.
 * The only intended use is to emit COM or APPn markers after calling
 * write_file_header and before calling write_frame_header.
 * Other uses are not guaranteed to produce desirable results.
 * Counting the parameter bytes properly is the caller's responsibility.
 */

unsafe extern "C" fn write_marker_header(
    mut cinfo: j_compress_ptr,
    mut marker: c_int,
    mut datalen: c_uint,
)
/* Emit an arbitrary marker header */
{
    if datalen > 65533u32 {
        /* safety check */
        (*(*cinfo).err).msg_code = super::jerror::JERR_BAD_LENGTH as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    emit_marker(cinfo, marker as JPEG_MARKER);
    emit_2bytes(cinfo, (datalen + 2u32) as c_int);
    /* total length */
}

unsafe extern "C" fn write_marker_byte(mut cinfo: j_compress_ptr, mut val: c_int)
/* Emit one byte of marker parameters following write_marker_header */
{
    emit_byte(cinfo, val);
}
/*
 * Write datastream header.
 * This consists of an SOI and optional APPn markers.
 * We recommend use of the JFIF marker, but not the Adobe marker,
 * when using YCbCr or grayscale data.  The JFIF marker should NOT
 * be used for any other JPEG colorspace.  The Adobe marker is helpful
 * to distinguish RGB, CMYK, and YCCK colorspaces.
 * Note that an application can write additional header markers after
 * jpeg_start_compress returns.
 */

unsafe extern "C" fn write_file_header(mut cinfo: j_compress_ptr) {
    let mut marker: my_marker_ptr = (*cinfo).marker as my_marker_ptr; /* first the SOI */
    emit_marker(cinfo, M_SOI);
    /* SOI is defined to reset restart interval to 0 */
    (*marker).last_restart_interval = 0u32;
    if (*cinfo).write_JFIF_header != 0 {
        /* next an optional JFIF APP0 */
        emit_jfif_app0(cinfo);
    }
    if (*cinfo).write_Adobe_marker != 0 {
        /* next an optional Adobe APP14 */
        emit_adobe_app14(cinfo);
    };
}
/*
 * Write frame header.
 * This consists of DQT and SOFn markers.
 * Note that we do not emit the SOF until we have emitted the DQT(s).
 * This avoids compatibility problems with incorrect implementations that
 * try to error-check the quant table numbers as soon as they see the SOF.
 */

unsafe extern "C" fn write_frame_header(mut cinfo: j_compress_ptr) {
    let mut ci: c_int = 0;
    let mut is_baseline: boolean = 0;
    let mut compptr: *mut jpeg_component_info = ::std::ptr::null_mut::<jpeg_component_info>();
    /* Emit DQT for each quantization table.
     * Note that emit_dqt() suppresses any duplicate tables.
     */
    let mut prec: c_int = emit_multi_dqt(cinfo);
    if prec == -1i32 {
        prec = 0i32;
        ci = 0i32;
        compptr = (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
            prec += emit_dqt(cinfo, (*compptr).quant_tbl_no);
            ci += 1;
            compptr = compptr.offset(1)
        }
    }
    /* now prec is nonzero iff there are any 16-bit quant tables. */
    /* Check for a non-baseline specification.
     * Note we assume that Huffman table numbers won't be changed later.
     */
    if (*cinfo).arith_code != 0 || (*cinfo).progressive_mode != 0 || (*cinfo).data_precision != 8i32
    {
        is_baseline = FALSE
    } else {
        is_baseline = TRUE;
        ci = 0i32;
        compptr = (*cinfo).comp_info;
        while ci < (*cinfo).num_components {
            if (*compptr).dc_tbl_no > 1i32 || (*compptr).ac_tbl_no > 1i32 {
                is_baseline = FALSE
            }
            ci += 1;
            compptr = compptr.offset(1)
        }
        if prec != 0 && is_baseline != 0 {
            is_baseline = FALSE;
            /* If it's baseline except for quantizer size, warn the user */
            (*(*cinfo).err).msg_code = super::jerror::JTRC_16BIT_TABLES as c_int;
            Some(
                (*(*cinfo).err)
                    .emit_message
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(cinfo as j_common_ptr, 0i32);
        }
    }
    /* Emit the proper SOF marker */
    if (*cinfo).arith_code != 0 {
        if (*cinfo).progressive_mode != 0 {
            emit_sof(cinfo, M_SOF10); /* SOF code for progressive arithmetic */
        } else {
            emit_sof(cinfo, M_SOF9);
        }
    /* SOF code for sequential arithmetic */
    } else if (*cinfo).progressive_mode != 0 {
        emit_sof(cinfo, M_SOF2); /* SOF code for progressive Huffman */
    } else if is_baseline != 0 {
        emit_sof(cinfo, M_SOF0); /* SOF code for baseline implementation */
    } else {
        emit_sof(cinfo, M_SOF1);
    };
}
/* SOF code for non-baseline Huffman file */
/*
 * Write scan header.
 * This consists of DHT or DAC markers, optional DRI, and SOS.
 * Compressed data will be written following the SOS.
 */

unsafe extern "C" fn write_scan_header(mut cinfo: j_compress_ptr) {
    let mut marker: my_marker_ptr = (*cinfo).marker as my_marker_ptr;

    if (*cinfo).arith_code != 0 {
        /* Emit arith conditioning info.  We may have some duplication
         * if the file has multiple scans, but it's so small it's hardly
         * worth worrying about.
         */
        emit_dac(cinfo);
    } else if emit_multi_dht(cinfo) == 0 {
        let mut i: c_int = 0i32;
        while i < (*cinfo).comps_in_scan {
            let mut compptr: *mut jpeg_component_info = (*cinfo).cur_comp_info[i as usize];
            /* Emit Huffman tables.
             * Note that emit_dht() suppresses any duplicate tables.
             */
            /* DC needs no table for refinement scan */
            if (*cinfo).Ss == 0i32 && (*cinfo).Ah == 0i32 {
                emit_dht(cinfo, (*compptr).dc_tbl_no, FALSE);
            }
            /* AC needs no table when not present */
            if (*cinfo).Se != 0 {
                emit_dht(cinfo, (*compptr).ac_tbl_no, TRUE);
            }
            i += 1
        }
    }
    /* Emit DRI if required --- note that DRI value could change for each scan.
     * We avoid wasting space with unnecessary DRIs, however.
     */
    if (*cinfo).restart_interval != (*marker).last_restart_interval {
        emit_dri(cinfo);
        (*marker).last_restart_interval = (*cinfo).restart_interval
    }
    emit_sos(cinfo);
}
/*
 * Write datastream trailer.
 */

unsafe extern "C" fn write_file_trailer(mut cinfo: j_compress_ptr) {
    emit_marker(cinfo, M_EOI);
}
/*
 * Write an abbreviated table-specification datastream.
 * This consists of SOI, DQT and DHT tables, and EOI.
 * Any table that is defined and not marked sent_table = TRUE will be
 * emitted.  Note that all tables will be marked sent_table = TRUE at exit.
 */

unsafe extern "C" fn write_tables_only(mut cinfo: j_compress_ptr) {
    emit_marker(cinfo, M_SOI);
    let mut i: c_int = 0i32;
    while i < NUM_QUANT_TBLS {
        if !(*cinfo).quant_tbl_ptrs[i as usize].is_null() {
            emit_dqt(cinfo, i);
        }
        i += 1
    }
    if (*cinfo).arith_code == 0 {
        i = 0i32;
        while i < NUM_HUFF_TBLS {
            if !(*cinfo).dc_huff_tbl_ptrs[i as usize].is_null() {
                emit_dht(cinfo, i, FALSE);
            }
            if !(*cinfo).ac_huff_tbl_ptrs[i as usize].is_null() {
                emit_dht(cinfo, i, TRUE);
            }
            i += 1
        }
    }
    emit_marker(cinfo, M_EOI);
}
/*
 * Initialize the marker writer module.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_marker_writer(mut cinfo: j_compress_ptr) {
    let mut marker: my_marker_ptr = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as j_common_ptr,
        JPOOL_IMAGE,
        ::std::mem::size_of::<my_marker_writer>() as c_ulong,
    ) as my_marker_ptr;
    (*cinfo).marker = marker as *mut jpeg_marker_writer;
    /* Initialize method pointers */
    (*marker).pub_0.write_file_header =
        Some(write_file_header as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    (*marker).pub_0.write_frame_header =
        Some(write_frame_header as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    (*marker).pub_0.write_scan_header =
        Some(write_scan_header as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    (*marker).pub_0.write_file_trailer =
        Some(write_file_trailer as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    (*marker).pub_0.write_tables_only =
        Some(write_tables_only as unsafe extern "C" fn(_: j_compress_ptr) -> ());
    (*marker).pub_0.write_marker_header = Some(
        write_marker_header as unsafe extern "C" fn(_: j_compress_ptr, _: c_int, _: c_uint) -> (),
    );
    (*marker).pub_0.write_marker_byte =
        Some(write_marker_byte as unsafe extern "C" fn(_: j_compress_ptr, _: c_int) -> ());
    /* Initialize private state */
    (*marker).last_restart_interval = 0u32;
}
