use libc;
use libc::c_int;
use libc::c_long;
use libc::c_ulong;
use libc::c_void;

pub use crate::jerror::C2RustUnnamed_4;
pub use crate::jerror::JERR_ARITH_NOTIMPL;
pub use crate::jerror::JERR_BAD_ALIGN_TYPE;
pub use crate::jerror::JERR_BAD_ALLOC_CHUNK;
pub use crate::jerror::JERR_BAD_BUFFER_MODE;
pub use crate::jerror::JERR_BAD_COMPONENT_ID;
pub use crate::jerror::JERR_BAD_CROP_SPEC;
pub use crate::jerror::JERR_BAD_DCTSIZE;
pub use crate::jerror::JERR_BAD_DCT_COEF;
pub use crate::jerror::JERR_BAD_HUFF_TABLE;
pub use crate::jerror::JERR_BAD_IN_COLORSPACE;
pub use crate::jerror::JERR_BAD_J_COLORSPACE;
pub use crate::jerror::JERR_BAD_LENGTH;
pub use crate::jerror::JERR_BAD_LIB_VERSION;
pub use crate::jerror::JERR_BAD_MCU_SIZE;
pub use crate::jerror::JERR_BAD_PARAM;
pub use crate::jerror::JERR_BAD_PARAM_VALUE;
pub use crate::jerror::JERR_BAD_POOL_ID;
pub use crate::jerror::JERR_BAD_PRECISION;
pub use crate::jerror::JERR_BAD_PROGRESSION;
pub use crate::jerror::JERR_BAD_PROG_SCRIPT;
pub use crate::jerror::JERR_BAD_SAMPLING;
pub use crate::jerror::JERR_BAD_SCAN_SCRIPT;
pub use crate::jerror::JERR_BAD_STATE;
pub use crate::jerror::JERR_BAD_STRUCT_SIZE;
pub use crate::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use crate::jerror::JERR_BUFFER_SIZE;
pub use crate::jerror::JERR_CANT_SUSPEND;
pub use crate::jerror::JERR_CCIR601_NOTIMPL;
pub use crate::jerror::JERR_COMPONENT_COUNT;
pub use crate::jerror::JERR_CONVERSION_NOTIMPL;
pub use crate::jerror::JERR_DAC_INDEX;
pub use crate::jerror::JERR_DAC_VALUE;
pub use crate::jerror::JERR_DHT_INDEX;
pub use crate::jerror::JERR_DQT_INDEX;
pub use crate::jerror::JERR_EMPTY_IMAGE;
pub use crate::jerror::JERR_EMS_READ;
pub use crate::jerror::JERR_EMS_WRITE;
pub use crate::jerror::JERR_EOI_EXPECTED;
pub use crate::jerror::JERR_FILE_READ;
pub use crate::jerror::JERR_FILE_WRITE;
pub use crate::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use crate::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use crate::jerror::JERR_HUFF_MISSING_CODE;
pub use crate::jerror::JERR_IMAGE_TOO_BIG;
pub use crate::jerror::JERR_INPUT_EMPTY;
pub use crate::jerror::JERR_INPUT_EOF;
pub use crate::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use crate::jerror::JERR_MISSING_DATA;
pub use crate::jerror::JERR_MODE_CHANGE;
pub use crate::jerror::JERR_NOTIMPL;
pub use crate::jerror::JERR_NOT_COMPILED;
pub use crate::jerror::JERR_NO_BACKING_STORE;
pub use crate::jerror::JERR_NO_HUFF_TABLE;
pub use crate::jerror::JERR_NO_IMAGE;
pub use crate::jerror::JERR_NO_QUANT_TABLE;
pub use crate::jerror::JERR_NO_SOI;
pub use crate::jerror::JERR_OUT_OF_MEMORY;
pub use crate::jerror::JERR_QUANT_COMPONENTS;
pub use crate::jerror::JERR_QUANT_FEW_COLORS;
pub use crate::jerror::JERR_QUANT_MANY_COLORS;
pub use crate::jerror::JERR_SOF_DUPLICATE;
pub use crate::jerror::JERR_SOF_NO_SOS;
pub use crate::jerror::JERR_SOF_UNSUPPORTED;
pub use crate::jerror::JERR_SOI_DUPLICATE;
pub use crate::jerror::JERR_SOS_NO_SOF;
pub use crate::jerror::JERR_TFILE_CREATE;
pub use crate::jerror::JERR_TFILE_READ;
pub use crate::jerror::JERR_TFILE_SEEK;
pub use crate::jerror::JERR_TFILE_WRITE;
pub use crate::jerror::JERR_TOO_LITTLE_DATA;
pub use crate::jerror::JERR_UNKNOWN_MARKER;
pub use crate::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use crate::jerror::JERR_VIRTUAL_BUG;
pub use crate::jerror::JERR_WIDTH_OVERFLOW;
pub use crate::jerror::JERR_XMS_READ;
pub use crate::jerror::JERR_XMS_WRITE;
pub use crate::jerror::JMSG_COPYRIGHT;
pub use crate::jerror::JMSG_LASTMSGCODE;
pub use crate::jerror::JMSG_NOMESSAGE;
pub use crate::jerror::JMSG_VERSION;
pub use crate::jerror::JTRC_16BIT_TABLES;
pub use crate::jerror::JTRC_ADOBE;
pub use crate::jerror::JTRC_APP0;
pub use crate::jerror::JTRC_APP14;
pub use crate::jerror::JTRC_DAC;
pub use crate::jerror::JTRC_DHT;
pub use crate::jerror::JTRC_DQT;
pub use crate::jerror::JTRC_DRI;
pub use crate::jerror::JTRC_EMS_CLOSE;
pub use crate::jerror::JTRC_EMS_OPEN;
pub use crate::jerror::JTRC_EOI;
pub use crate::jerror::JTRC_HUFFBITS;
pub use crate::jerror::JTRC_JFIF;
pub use crate::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use crate::jerror::JTRC_JFIF_EXTENSION;
pub use crate::jerror::JTRC_JFIF_THUMBNAIL;
pub use crate::jerror::JTRC_MISC_MARKER;
pub use crate::jerror::JTRC_PARMLESS_MARKER;
pub use crate::jerror::JTRC_QUANTVALS;
pub use crate::jerror::JTRC_QUANT_3_NCOLORS;
pub use crate::jerror::JTRC_QUANT_NCOLORS;
pub use crate::jerror::JTRC_QUANT_SELECTED;
pub use crate::jerror::JTRC_RECOVERY_ACTION;
pub use crate::jerror::JTRC_RST;
pub use crate::jerror::JTRC_SMOOTH_NOTIMPL;
pub use crate::jerror::JTRC_SOF;
pub use crate::jerror::JTRC_SOF_COMPONENT;
pub use crate::jerror::JTRC_SOI;
pub use crate::jerror::JTRC_SOS;
pub use crate::jerror::JTRC_SOS_COMPONENT;
pub use crate::jerror::JTRC_SOS_PARAMS;
pub use crate::jerror::JTRC_TFILE_CLOSE;
pub use crate::jerror::JTRC_TFILE_OPEN;
pub use crate::jerror::JTRC_THUMB_JPEG;
pub use crate::jerror::JTRC_THUMB_PALETTE;
pub use crate::jerror::JTRC_THUMB_RGB;
pub use crate::jerror::JTRC_UNKNOWN_IDS;
pub use crate::jerror::JTRC_XMS_CLOSE;
pub use crate::jerror::JTRC_XMS_OPEN;
pub use crate::jerror::JWRN_ADOBE_XFORM;
pub use crate::jerror::JWRN_BOGUS_ICC;
pub use crate::jerror::JWRN_BOGUS_PROGRESSION;
pub use crate::jerror::JWRN_EXTRANEOUS_DATA;
pub use crate::jerror::JWRN_HIT_MARKER;
pub use crate::jerror::JWRN_HUFF_BAD_CODE;
pub use crate::jerror::JWRN_JFIF_MAJOR;
pub use crate::jerror::JWRN_JPEG_EOF;
pub use crate::jerror::JWRN_MUST_RESYNC;
pub use crate::jerror::JWRN_NOT_SEQUENTIAL;
pub use crate::jerror::JWRN_TOO_MUCH_DATA;
pub use crate::jmemsys_h::backing_store_ptr;
pub use crate::jmemsys_h::backing_store_struct;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_3;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
use crate::stdlib::free;
use crate::stdlib::malloc;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;
/*
 * jmemnobs.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1992-1996, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2017-2018, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file provides a really simple implementation of the system-
 * dependent portion of the JPEG memory manager.  This implementation
 * assumes that no backing-store files are needed: all required space
 * can be obtained from malloc().
 * This is very portable in the sense that it'll compile on almost anything,
 * but you'd better have lots of main memory (or virtual memory) if you want
 * to process big images.
 */
/* <stdlib.h> should declare malloc(),free() */
/*
 * Memory allocation and freeing are controlled by the regular library
 * routines malloc() and free().
 */
/*
 * jmemsys.h
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1992-1997, Thomas G. Lane.
 * It was modified by The libjpeg-turbo Project to include only code and
 * information relevant to libjpeg-turbo.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This include file defines the interface between the system-independent
 * and system-dependent portions of the JPEG memory manager.  No other
 * modules need include it.  (The system-independent portion is jmemmgr.c;
 * there are several different versions of the system-dependent portion.)
 *
 * This file works as-is for the system-dependent memory managers supplied
 * in the IJG distribution.  You may need to modify it if you write a
 * custom memory manager.  If system-dependent changes are needed in
 * this file, the best method is to #ifdef them based on a configuration
 * symbol supplied in jconfig.h.
 */
/*
 * These two functions are used to allocate and release small chunks of
 * memory.  (Typically the total amount requested through jpeg_get_small is
 * no more than 20K or so; this will be requested in chunks of a few K each.)
 * Behavior should be the same as for the standard library functions malloc
 * and free; in particular, jpeg_get_small must return NULL on failure.
 * On most systems, these ARE malloc and free.  jpeg_free_small is passed the
 * size of the object being freed, just in case it's needed.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_get_small(
    mut cinfo: j_common_ptr,
    mut sizeofobject: size_t,
) -> *mut c_void {
    return malloc(sizeofobject);
}
#[no_mangle]
pub unsafe extern "C" fn jpeg_free_small(
    mut cinfo: j_common_ptr,
    mut object: *mut c_void,
    mut sizeofobject: size_t,
) {
    free(object);
}
/*
 * "Large" objects are treated the same as "small" ones.
 */
/*
 * These two functions are used to allocate and release large chunks of
 * memory (up to the total free space designated by jpeg_mem_available).
 * These are identical to the jpeg_get/free_small routines; but we keep them
 * separate anyway, in case a different allocation strategy is desirable for
 * large chunks.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_get_large(
    mut cinfo: j_common_ptr,
    mut sizeofobject: size_t,
) -> *mut c_void {
    return malloc(sizeofobject);
}
#[no_mangle]
pub unsafe extern "C" fn jpeg_free_large(
    mut cinfo: j_common_ptr,
    mut object: *mut c_void,
    mut sizeofobject: size_t,
) {
    free(object);
}
/*
 * This routine computes the total memory space available for allocation.
 */
/*
 * The macro MAX_ALLOC_CHUNK designates the maximum number of bytes that may
 * be requested in a single call to jpeg_get_large (and jpeg_get_small for that
 * matter, but that case should never come into play).  This macro was needed
 * to model the 64Kb-segment-size limit of far addressing on 80x86 machines.
 * On machines with flat address spaces, any large constant may be used.
 *
 * NB: jmemmgr.c expects that MAX_ALLOC_CHUNK will be representable as type
 * size_t and will be a multiple of sizeof(align_type).
 */
/* may be overridden in jconfig.h */
/*
 * This routine computes the total space still available for allocation by
 * jpeg_get_large.  If more space than this is needed, backing store will be
 * used.  NOTE: any memory already allocated must not be counted.
 *
 * There is a minimum space requirement, corresponding to the minimum
 * feasible buffer sizes; jmemmgr.c will request that much space even if
 * jpeg_mem_available returns zero.  The maximum space needed, enough to hold
 * all working storage in memory, is also passed in case it is useful.
 * Finally, the total space already allocated is passed.  If no better
 * method is available, cinfo->mem->max_memory_to_use - already_allocated
 * is often a suitable calculation.
 *
 * It is OK for jpeg_mem_available to underestimate the space available
 * (that'll just lead to more backing-store access than is really necessary).
 * However, an overestimate will lead to failure.  Hence it's wise to subtract
 * a slop factor from the true available space.  5% should be enough.
 *
 * On machines with lots of virtual memory, any large constant may be returned.
 * Conversely, zero may be returned to always use the minimum amount of memory.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_mem_available(
    mut cinfo: j_common_ptr,
    mut min_bytes_needed: size_t,
    mut max_bytes_needed: size_t,
    mut already_allocated: size_t,
) -> size_t {
    if 0 != (*(*cinfo).mem).max_memory_to_use {
        if (*(*cinfo).mem).max_memory_to_use as size_t > already_allocated {
            return ((*(*cinfo).mem).max_memory_to_use as c_ulong).wrapping_sub(already_allocated);
        } else {
            return 0i32 as size_t;
        }
    } else {
        return max_bytes_needed;
    };
}
/*
 * Backing store (temporary file) management.
 * Since jpeg_mem_available always promised the moon,
 * this should never be called and we can just error out.
 */
/*
 * Initial opening of a backing-store object.  This must fill in the
 * read/write/close pointers in the object.  The read/write routines
 * may take an error exit if the specified maximum file size is exceeded.
 * (If jpeg_mem_available always returns a large value, this routine can
 * just take an error exit.)
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_open_backing_store(
    mut cinfo: j_common_ptr,
    mut info: backing_store_ptr,
    mut total_bytes_needed: c_long,
) {
    (*(*cinfo).err).msg_code = JERR_NO_BACKING_STORE as c_int;
    (*(*cinfo).err)
        .error_exit
        .expect("non-null function pointer")(cinfo);
}
/*
 * These routines take care of any system-dependent initialization and
 * cleanup required.  Here, there isn't any.
 */
/*
 * These routines take care of any system-dependent initialization and
 * cleanup required.  jpeg_mem_init will be called before anything is
 * allocated (and, therefore, nothing in cinfo is of use except the error
 * manager pointer).  It should return a suitable default value for
 * max_memory_to_use; this may subsequently be overridden by the surrounding
 * application.  (Note that max_memory_to_use is only important if
 * jpeg_mem_available chooses to consult it ... no one else will.)
 * jpeg_mem_term may assume that all requested memory has been freed and that
 * all opened backing-store objects have been closed.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_mem_init(mut cinfo: j_common_ptr) -> c_long {
    return 0i32 as c_long;
}
#[no_mangle]
pub unsafe extern "C" fn jpeg_mem_term(mut cinfo: j_common_ptr) {}
