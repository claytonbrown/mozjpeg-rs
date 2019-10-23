use libc::c_uchar;use libc::c_ulong;use libc::c_int;use libc::c_long;use libc::c_void;use libc;

pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
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
pub use crate::jpeglib_h::jpeg_marker_reader;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_resync_to_restart;
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
pub use crate::jpeglib_h::JPEG_EOI;
pub use crate::jpeglib_h::JPOOL_PERMANENT;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use super::jerror::C2RustUnnamed_3;
pub use super::jerror::JERR_ARITH_NOTIMPL;
pub use super::jerror::JERR_BAD_ALIGN_TYPE;
pub use super::jerror::JERR_BAD_ALLOC_CHUNK;
pub use super::jerror::JERR_BAD_BUFFER_MODE;
pub use super::jerror::JERR_BAD_COMPONENT_ID;
pub use super::jerror::JERR_BAD_CROP_SPEC;
pub use super::jerror::JERR_BAD_DCTSIZE;
pub use super::jerror::JERR_BAD_DCT_COEF;
pub use super::jerror::JERR_BAD_HUFF_TABLE;
pub use super::jerror::JERR_BAD_IN_COLORSPACE;
pub use super::jerror::JERR_BAD_J_COLORSPACE;
pub use super::jerror::JERR_BAD_LENGTH;
pub use super::jerror::JERR_BAD_LIB_VERSION;
pub use super::jerror::JERR_BAD_MCU_SIZE;
pub use super::jerror::JERR_BAD_PARAM;
pub use super::jerror::JERR_BAD_PARAM_VALUE;
pub use super::jerror::JERR_BAD_POOL_ID;
pub use super::jerror::JERR_BAD_PRECISION;
pub use super::jerror::JERR_BAD_PROGRESSION;
pub use super::jerror::JERR_BAD_PROG_SCRIPT;
pub use super::jerror::JERR_BAD_SAMPLING;
pub use super::jerror::JERR_BAD_SCAN_SCRIPT;
pub use super::jerror::JERR_BAD_STATE;
pub use super::jerror::JERR_BAD_STRUCT_SIZE;
pub use super::jerror::JERR_BAD_VIRTUAL_ACCESS;
pub use super::jerror::JERR_BUFFER_SIZE;
pub use super::jerror::JERR_CANT_SUSPEND;
pub use super::jerror::JERR_CCIR601_NOTIMPL;
pub use super::jerror::JERR_COMPONENT_COUNT;
pub use super::jerror::JERR_CONVERSION_NOTIMPL;
pub use super::jerror::JERR_DAC_INDEX;
pub use super::jerror::JERR_DAC_VALUE;
pub use super::jerror::JERR_DHT_INDEX;
pub use super::jerror::JERR_DQT_INDEX;
pub use super::jerror::JERR_EMPTY_IMAGE;
pub use super::jerror::JERR_EMS_READ;
pub use super::jerror::JERR_EMS_WRITE;
pub use super::jerror::JERR_EOI_EXPECTED;
pub use super::jerror::JERR_FILE_READ;
pub use super::jerror::JERR_FILE_WRITE;
pub use super::jerror::JERR_FRACT_SAMPLE_NOTIMPL;
pub use super::jerror::JERR_HUFF_CLEN_OVERFLOW;
pub use super::jerror::JERR_HUFF_MISSING_CODE;
pub use super::jerror::JERR_IMAGE_TOO_BIG;
pub use super::jerror::JERR_INPUT_EMPTY;
pub use super::jerror::JERR_INPUT_EOF;
pub use super::jerror::JERR_MISMATCHED_QUANT_TABLE;
pub use super::jerror::JERR_MISSING_DATA;
pub use super::jerror::JERR_MODE_CHANGE;
pub use super::jerror::JERR_NOTIMPL;
pub use super::jerror::JERR_NOT_COMPILED;
pub use super::jerror::JERR_NO_BACKING_STORE;
pub use super::jerror::JERR_NO_HUFF_TABLE;
pub use super::jerror::JERR_NO_IMAGE;
pub use super::jerror::JERR_NO_QUANT_TABLE;
pub use super::jerror::JERR_NO_SOI;
pub use super::jerror::JERR_OUT_OF_MEMORY;
pub use super::jerror::JERR_QUANT_COMPONENTS;
pub use super::jerror::JERR_QUANT_FEW_COLORS;
pub use super::jerror::JERR_QUANT_MANY_COLORS;
pub use super::jerror::JERR_SOF_DUPLICATE;
pub use super::jerror::JERR_SOF_NO_SOS;
pub use super::jerror::JERR_SOF_UNSUPPORTED;
pub use super::jerror::JERR_SOI_DUPLICATE;
pub use super::jerror::JERR_SOS_NO_SOF;
pub use super::jerror::JERR_TFILE_CREATE;
pub use super::jerror::JERR_TFILE_READ;
pub use super::jerror::JERR_TFILE_SEEK;
pub use super::jerror::JERR_TFILE_WRITE;
pub use super::jerror::JERR_TOO_LITTLE_DATA;
pub use super::jerror::JERR_UNKNOWN_MARKER;
pub use super::jerror::JERR_UNSUPPORTED_SUSPEND;
pub use super::jerror::JERR_VIRTUAL_BUG;
pub use super::jerror::JERR_WIDTH_OVERFLOW;
pub use super::jerror::JERR_XMS_READ;
pub use super::jerror::JERR_XMS_WRITE;
pub use super::jerror::JMSG_COPYRIGHT;
pub use super::jerror::JMSG_LASTMSGCODE;
pub use super::jerror::JMSG_NOMESSAGE;
pub use super::jerror::JMSG_VERSION;
pub use super::jerror::JTRC_16BIT_TABLES;
pub use super::jerror::JTRC_ADOBE;
pub use super::jerror::JTRC_APP0;
pub use super::jerror::JTRC_APP14;
pub use super::jerror::JTRC_DAC;
pub use super::jerror::JTRC_DHT;
pub use super::jerror::JTRC_DQT;
pub use super::jerror::JTRC_DRI;
pub use super::jerror::JTRC_EMS_CLOSE;
pub use super::jerror::JTRC_EMS_OPEN;
pub use super::jerror::JTRC_EOI;
pub use super::jerror::JTRC_HUFFBITS;
pub use super::jerror::JTRC_JFIF;
pub use super::jerror::JTRC_JFIF_BADTHUMBNAILSIZE;
pub use super::jerror::JTRC_JFIF_EXTENSION;
pub use super::jerror::JTRC_JFIF_THUMBNAIL;
pub use super::jerror::JTRC_MISC_MARKER;
pub use super::jerror::JTRC_PARMLESS_MARKER;
pub use super::jerror::JTRC_QUANTVALS;
pub use super::jerror::JTRC_QUANT_3_NCOLORS;
pub use super::jerror::JTRC_QUANT_NCOLORS;
pub use super::jerror::JTRC_QUANT_SELECTED;
pub use super::jerror::JTRC_RECOVERY_ACTION;
pub use super::jerror::JTRC_RST;
pub use super::jerror::JTRC_SMOOTH_NOTIMPL;
pub use super::jerror::JTRC_SOF;
pub use super::jerror::JTRC_SOF_COMPONENT;
pub use super::jerror::JTRC_SOI;
pub use super::jerror::JTRC_SOS;
pub use super::jerror::JTRC_SOS_COMPONENT;
pub use super::jerror::JTRC_SOS_PARAMS;
pub use super::jerror::JTRC_TFILE_CLOSE;
pub use super::jerror::JTRC_TFILE_OPEN;
pub use super::jerror::JTRC_THUMB_JPEG;
pub use super::jerror::JTRC_THUMB_PALETTE;
pub use super::jerror::JTRC_THUMB_RGB;
pub use super::jerror::JTRC_UNKNOWN_IDS;
pub use super::jerror::JTRC_XMS_CLOSE;
pub use super::jerror::JTRC_XMS_OPEN;
pub use super::jerror::JWRN_ADOBE_XFORM;
pub use super::jerror::JWRN_BOGUS_ICC;
pub use super::jerror::JWRN_BOGUS_PROGRESSION;
pub use super::jerror::JWRN_EXTRANEOUS_DATA;
pub use super::jerror::JWRN_HIT_MARKER;
pub use super::jerror::JWRN_HUFF_BAD_CODE;
pub use super::jerror::JWRN_JFIF_MAJOR;
pub use super::jerror::JWRN_JPEG_EOF;
pub use super::jerror::JWRN_MUST_RESYNC;
pub use super::jerror::JWRN_NOT_SEQUENTIAL;
pub use super::jerror::JWRN_TOO_MUCH_DATA;
use crate::stdlib::fread;

pub type my_src_ptr = *mut my_source_mgr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_source_mgr {
    pub pub_0: jpeg_source_mgr,
    pub infile: *mut FILE,
    pub buffer: *mut JOCTET,
    pub start_of_file: boolean,
}

pub const INPUT_BUF_SIZE: c_int = 4096i32;
/* choose an efficiently fread'able size */
/*
 * Initialize source --- called by jpeg_read_header
 * before any data is actually read.
 */

unsafe extern "C" fn init_source(mut cinfo: j_decompress_ptr) {
    let mut src: my_src_ptr = (*cinfo).src as my_src_ptr;
    /* We reset the empty-input-file flag for each image,
     * but we don't clear the input buffer.
     * This is correct behavior for reading a series of images from one source.
     */
    (*src).start_of_file = TRUE;
}

unsafe extern "C" fn init_mem_source(mut cinfo: j_decompress_ptr) {
    /* no work necessary here */
}
/*
 * Fill the input buffer --- called whenever buffer is emptied.
 *
 * In typical applications, this should read fresh data into the buffer
 * (ignoring the current state of next_input_byte & bytes_in_buffer),
 * reset the pointer & count to the start of the buffer, and return TRUE
 * indicating that the buffer has been reloaded.  It is not necessary to
 * fill the buffer entirely, only to obtain at least one more byte.
 *
 * There is no such thing as an EOF return.  If the end of the file has been
 * reached, the routine has a choice of ERREXIT() or inserting fake data into
 * the buffer.  In most cases, generating a warning message and inserting a
 * fake EOI marker is the best course of action --- this will allow the
 * decompressor to output however much of the image is there.  However,
 * the resulting error message is misleading if the real problem is an empty
 * input file, so we handle that case specially.
 *
 * In applications that need to be able to suspend compression due to input
 * not being available yet, a FALSE return indicates that no more data can be
 * obtained right now, but more may be forthcoming later.  In this situation,
 * the decompressor will return to its caller (with an indication of the
 * number of scanlines it has read, if any).  The application should resume
 * decompression after it has loaded more data into the input buffer.  Note
 * that there are substantial restrictions on the use of suspension --- see
 * the documentation.
 *
 * When suspending, the decompressor will back up to a convenient restart point
 * (typically the start of the current MCU). next_input_byte & bytes_in_buffer
 * indicate where the restart point will be if the current call returns FALSE.
 * Data beyond this point must be rescanned after resumption, so move it to
 * the front of the buffer rather than discarding it.
 */

unsafe extern "C" fn fill_input_buffer(
    mut cinfo: j_decompress_ptr,
) -> boolean {
     let mut src: my_src_ptr = (*cinfo).src as my_src_ptr;
    
     let mut nbytes:   size_t =
     fread(
        (*src).buffer as *mut c_void,
        1u64,
        4096u64,
        (*src).infile,
    );
    if nbytes <= 0u64 {
        if (*src).start_of_file != 0 {
            /* Treat empty input file as fatal error */
            (*(*cinfo).err).msg_code = super::jerror::JERR_INPUT_EMPTY as c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as j_common_ptr
            );
        }
        (*(*cinfo).err).msg_code = super::jerror::JWRN_JPEG_EOF as c_int;
        Some(
            (*(*cinfo).err)
                .emit_message
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr, -1i32);
        /* Insert a fake EOI marker */
        *(*src).buffer.offset(0) = 0xffu8;
        *(*src).buffer.offset(1) = JPEG_EOI as JOCTET;
        nbytes = 2u64
    }
    (*src).pub_0.next_input_byte = (*src).buffer;
    (*src).pub_0.bytes_in_buffer = nbytes;
    (*src).start_of_file = FALSE;
    return TRUE;
}

unsafe extern "C" fn fill_mem_input_buffer(
    mut cinfo: j_decompress_ptr,
) -> boolean {
    static mut mybuffer: [JOCTET; 4] = [
        0xffu8,
        JPEG_EOI as JOCTET,
        0u8,
        0u8,
    ];
    /* The whole JPEG data is expected to reside in the supplied memory
     * buffer, so any request for more data beyond the given buffer size
     * is treated as an error.
     */
    (*(*cinfo).err).msg_code = super::jerror::JWRN_JPEG_EOF as c_int;
    Some(
        (*(*cinfo).err)
            .emit_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo as j_common_ptr, -1i32);
    /* Insert a fake EOI marker */
    (*(*cinfo).src).next_input_byte = mybuffer.as_ptr();
    (*(*cinfo).src).bytes_in_buffer = 2u64;
    return TRUE;
}
/*
 * Skip data --- used to skip over a potentially large amount of
 * uninteresting data (such as an APPn marker).
 *
 * Writers of suspendable-input applications must note that skip_input_data
 * is not granted the right to give a suspension return.  If the skip extends
 * beyond the data currently in the buffer, the buffer can be marked empty so
 * that the next read will cause a fill_input_buffer call that can suspend.
 * Arranging for additional bytes to be discarded before reloading the input
 * buffer is the application writer's problem.
 */

unsafe extern "C" fn skip_input_data(
    mut cinfo: j_decompress_ptr,
    mut num_bytes: c_long,
) {
    let mut src: *mut jpeg_source_mgr = (*cinfo).src;
    /* Just a dumb implementation for now.  Could use fseek() except
     * it doesn't work on pipes.  Not clear that being smart is worth
     * any trouble anyway --- large skips are infrequent.
     */
    if num_bytes > 0i64 {
        while num_bytes > (*src).bytes_in_buffer as c_long {
            num_bytes -= (*src).bytes_in_buffer as c_long;
            Some((*src).fill_input_buffer.expect("non-null function pointer"))
                .expect("non-null function pointer")(cinfo);
            /* note we assume that fill_input_buffer will never return FALSE,
             * so suspension need not be handled.
             */
        }
        (*src).next_input_byte = (*src)
            .next_input_byte
            .offset(num_bytes as isize);
        (*src).bytes_in_buffer = (*src).bytes_in_buffer - num_bytes as size_t
    };
}
/*
 * An additional method that can be provided by data source modules is the
 * resync_to_restart method for error recovery in the presence of RST markers.
 * For the moment, this source module just uses the default resync method
 * provided by the JPEG library.  That method assumes that no backtracking
 * is possible.
 */
/*
 * Terminate source --- called by jpeg_finish_decompress
 * after all data has been read.  Often a no-op.
 *
 * NB: *not* called by jpeg_abort or jpeg_destroy; surrounding
 * application must deal with any cleanup that should happen even
 * for error exit.
 */

unsafe extern "C" fn term_source(mut cinfo: j_decompress_ptr) {
    /* no work necessary here */
}
/*
 * Prepare for input from a stdio stream.
 * The caller must have already opened the stream, and is responsible
 * for closing it after finishing decompression.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_stdio_src(
    mut cinfo: j_decompress_ptr,
    mut infile: *mut FILE,
) {
     let mut src:  my_src_ptr =  ::std::ptr::null_mut::< my_source_mgr>();
    /* The source object and input buffer are made permanent so that a series
     * of JPEG images can be read from the same file by calling jpeg_stdio_src
     * only before the first one.  (If we discarded the buffer at the end of
     * one image, we'd likely lose the start of the next one.)
     */
    if (*cinfo).src.is_null() {
        /* first time for this JPEG object? */
        (*cinfo).src = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            ::std::mem::size_of::<my_source_mgr>() as c_ulong,
        ) as *mut jpeg_source_mgr;
        src = (*cinfo).src as my_src_ptr;
        (*src).buffer = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            INPUT_BUF_SIZE as c_ulong *
    ::std::mem::size_of::<JOCTET>() as c_ulong,
        ) as *mut JOCTET
    } else if (*(*cinfo).src).init_source
        != Some(init_source as unsafe extern "C" fn(_: j_decompress_ptr) -> ())
    {
        /* It is unsafe to reuse the existing source manager unless it was created
         * by this function.  Otherwise, there is no guarantee that the opaque
         * structure is the right size.  Note that we could just create a new
         * structure, but the old structure would not be freed until
         * jpeg_destroy_decompress() was called.
         */
        (*(*cinfo).err).msg_code = super::jerror::JERR_BUFFER_SIZE as c_int; /* use default method */
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
        /* forces fill_input_buffer on first read */
    }
    src = (*cinfo).src as my_src_ptr;
    (*src).pub_0.init_source =
        Some(init_source as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*src).pub_0.fill_input_buffer = Some(
        fill_input_buffer
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
            ) -> boolean,
    );
    (*src).pub_0.skip_input_data = Some(
        skip_input_data
            as unsafe extern "C" fn(_: j_decompress_ptr, _: c_long) -> (),
    );
    (*src).pub_0.resync_to_restart = Some(
        jpeg_resync_to_restart
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
                _: c_int,
            ) -> boolean,
    );
    (*src).pub_0.term_source =
        Some(term_source as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*src).infile = infile;
    (*src).pub_0.bytes_in_buffer = 0u64;
    (*src).pub_0.next_input_byte = NULL as *const JOCTET;
    /* until buffer loaded */
}
/*
 * Prepare for input from a supplied memory buffer.
 * The buffer must contain the whole JPEG data.
 */
#[no_mangle]

pub unsafe extern "C" fn jpeg_mem_src(
    mut cinfo: j_decompress_ptr,
    mut inbuffer: *const c_uchar,
    mut insize: c_ulong,
) {
     
    if inbuffer.is_null() || insize == 0u64 {
        /* Treat empty input as fatal error */
        (*(*cinfo).err).msg_code = super::jerror::JERR_INPUT_EMPTY as c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
    /* The source object is made permanent so that a series of JPEG images
     * can be read from the same buffer by calling jpeg_mem_src only before
     * the first one.
     */
    if (*cinfo).src.is_null() {
        /* first time for this JPEG object? */
        (*cinfo).src = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_PERMANENT,
            ::std::mem::size_of::<jpeg_source_mgr>() as c_ulong,
        ) as *mut jpeg_source_mgr
    } else if (*(*cinfo).src).init_source
        != Some(
            init_mem_source as unsafe extern "C" fn(_: j_decompress_ptr) -> (),
        )
    {
        /* It is unsafe to reuse the existing source manager unless it was created
         * by this function.
         */
        (*(*cinfo).err).msg_code = super::jerror::JERR_BUFFER_SIZE as c_int; /* use default method */
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as j_common_ptr);
    }
     let mut src:   *mut jpeg_source_mgr =  (*cinfo).src;
    (*src).init_source =
        Some(init_mem_source as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*src).fill_input_buffer = Some(
        fill_mem_input_buffer
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
            ) -> boolean,
    );
    (*src).skip_input_data = Some(
        skip_input_data
            as unsafe extern "C" fn(_: j_decompress_ptr, _: c_long) -> (),
    );
    (*src).resync_to_restart = Some(
        jpeg_resync_to_restart
            as unsafe extern "C" fn(
                _: j_decompress_ptr,
                _: c_int,
            ) -> boolean,
    );
    (*src).term_source =
        Some(term_source as unsafe extern "C" fn(_: j_decompress_ptr) -> ());
    (*src).bytes_in_buffer = insize;
    (*src).next_input_byte =  inbuffer;
}
