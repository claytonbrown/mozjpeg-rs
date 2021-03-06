use ::libc;

#[c2rust::header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/jmorecfg.h:29"]
pub mod jmorecfg_h {

    /*
     * The remaining options affect code selection within the JPEG library,
     * but they don't need to be visible to most applications using the library.
     * To minimize application namespace pollution, the symbols won't be
     * defined unless JPEG_INTERNALS or JPEG_INTERNAL_OPTIONS has been defined.
     */
    /*
     * These defines indicate whether to include various optional functions.
     * Undefining some of these symbols will produce a smaller but less capable
     * library.  Note that you can leave certain source files out of the
     * compilation/linking process if you've #undef'd the corresponding symbols.
     * (You may HAVE to do that if your compiler doesn't like null source files.)
     */
    /* Capability options common to encoder and decoder: */
    /* slow but accurate integer algorithm */
    /* faster, less accurate integer method */
    /* floating-point: accurate, fast on fast HW */
    /* Encoder capability options: */
    /* Multiple-scan JPEG files? */
    /* Progressive JPEG? (Requires MULTISCAN)*/
    /* Optimization of entropy coding parms? */
    /* Note: if you selected 12-bit data precision, it is dangerous to turn off
     * ENTROPY_OPT_SUPPORTED.  The standard Huffman tables are only good for 8-bit
     * precision, so jchuff.c normally uses entropy optimization to compute
     * usable tables for higher precision.  If you don't want to do optimization,
     * you'll have to supply different default Huffman tables.
     * The exact same statements apply for progressive JPEG: the default tables
     * don't work for progressive mode.  (This may get fixed, however.)
     */
    /* Input image smoothing option? */
    /* Decoder capability options: */
    /* Multiple-scan JPEG files? */
    /* Progressive JPEG? (Requires MULTISCAN)*/
    /* jpeg_save_markers() needed? */
    /* Block smoothing? (Progressive only) */
    /* Output rescaling via IDCT? */
    /* Output rescaling at upsample stage? */
    /* Fast path for sloppy upsampling? */
    /* 1-pass color quantization? */
    /* 2-pass color quantization? */
    /* more capability options later, no doubt */
    /*
     * The RGB_RED, RGB_GREEN, RGB_BLUE, and RGB_PIXELSIZE macros are a vestigial
     * feature of libjpeg.  The idea was that, if an application developer needed
     * to compress from/decompress to a BGR/BGRX/RGBX/XBGR/XRGB buffer, they could
     * change these macros, rebuild libjpeg, and link their application statically
     * with it.  In reality, few people ever did this, because there were some
     * severe restrictions involved (cjpeg and djpeg no longer worked properly,
     * compressing/decompressing RGB JPEGs no longer worked properly, and the color
     * quantizer wouldn't work with pixel sizes other than 3.)  Furthermore, since
     * all of the O/S-supplied versions of libjpeg were built with the default
     * values of RGB_RED, RGB_GREEN, RGB_BLUE, and RGB_PIXELSIZE, many applications
     * have come to regard these values as immutable.
     *
     * The libjpeg-turbo colorspace extensions provide a much cleaner way of
     * compressing from/decompressing to buffers with arbitrary component orders
     * and pixel sizes.  Thus, we do not support changing the values of RGB_RED,
     * RGB_GREEN, RGB_BLUE, or RGB_PIXELSIZE.  In addition to the restrictions
     * listed above, changing these values will also break the SIMD extensions and
     * the regression tests.
     */
    /* Offset of Red in an RGB scanline element */
    /* Offset of Green */
    /* Offset of Blue */
    /* JSAMPLEs per RGB scanline element */

    pub static mut rgb_red: [libc::c_int; 17] = [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        crate::jmorecfg_h::RGB_RED,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        crate::jmorecfg_h::EXT_RGB_RED,
        crate::jmorecfg_h::EXT_RGBX_RED,
        crate::jmorecfg_h::EXT_BGR_RED,
        crate::jmorecfg_h::EXT_BGRX_RED,
        crate::jmorecfg_h::EXT_XBGR_RED,
        crate::jmorecfg_h::EXT_XRGB_RED,
        crate::jmorecfg_h::EXT_RGBX_RED,
        crate::jmorecfg_h::EXT_BGRX_RED,
        crate::jmorecfg_h::EXT_XBGR_RED,
        crate::jmorecfg_h::EXT_XRGB_RED,
        -(1 as libc::c_int),
    ];

    pub static mut rgb_green: [libc::c_int; 17] = [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        crate::jmorecfg_h::RGB_GREEN,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        crate::jmorecfg_h::EXT_RGB_GREEN,
        crate::jmorecfg_h::EXT_RGBX_GREEN,
        crate::jmorecfg_h::EXT_BGR_GREEN,
        crate::jmorecfg_h::EXT_BGRX_GREEN,
        crate::jmorecfg_h::EXT_XBGR_GREEN,
        crate::jmorecfg_h::EXT_XRGB_GREEN,
        crate::jmorecfg_h::EXT_RGBX_GREEN,
        crate::jmorecfg_h::EXT_BGRX_GREEN,
        crate::jmorecfg_h::EXT_XBGR_GREEN,
        crate::jmorecfg_h::EXT_XRGB_GREEN,
        -(1 as libc::c_int),
    ];

    pub static mut rgb_blue: [libc::c_int; 17] = [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        crate::jmorecfg_h::RGB_BLUE,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        crate::jmorecfg_h::EXT_RGB_BLUE,
        crate::jmorecfg_h::EXT_RGBX_BLUE,
        crate::jmorecfg_h::EXT_BGR_BLUE,
        crate::jmorecfg_h::EXT_BGRX_BLUE,
        crate::jmorecfg_h::EXT_XBGR_BLUE,
        crate::jmorecfg_h::EXT_XRGB_BLUE,
        crate::jmorecfg_h::EXT_RGBX_BLUE,
        crate::jmorecfg_h::EXT_BGRX_BLUE,
        crate::jmorecfg_h::EXT_XBGR_BLUE,
        crate::jmorecfg_h::EXT_XRGB_BLUE,
        -(1 as libc::c_int),
    ];

    pub static mut rgb_pixelsize: [libc::c_int; 17] = [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        crate::jmorecfg_h::RGB_PIXELSIZE,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        crate::jmorecfg_h::EXT_RGB_PIXELSIZE,
        crate::jmorecfg_h::EXT_RGBX_PIXELSIZE,
        crate::jmorecfg_h::EXT_BGR_PIXELSIZE,
        crate::jmorecfg_h::EXT_BGRX_PIXELSIZE,
        crate::jmorecfg_h::EXT_XBGR_PIXELSIZE,
        crate::jmorecfg_h::EXT_XRGB_PIXELSIZE,
        crate::jmorecfg_h::EXT_RGBX_PIXELSIZE,
        crate::jmorecfg_h::EXT_BGRX_PIXELSIZE,
        crate::jmorecfg_h::EXT_XBGR_PIXELSIZE,
        crate::jmorecfg_h::EXT_XRGB_PIXELSIZE,
        -(1 as libc::c_int),
    ];

    /* JPEG_INTERNAL_OPTIONS */
    /* FAST_FLOAT should be either float or double, whichever is done faster
     * by your compiler.  (Note that this type is only used in the floating point
     * DCT routines, so it only matters if you've defined DCT_FLOAT_SUPPORTED.)
     */
    /* prefer 16-bit with SIMD for parellelism */
    /* On some machines (notably 68000 series) "int" is 32 bits, but multiplying
     * two 16-bit shorts is faster than multiplying two ints.  Define MULTIPLIER
     * as short on such a machine.  MULTIPLIER must be at least 16 bits wide.
     */
    /* Definitions for speed-related optimizations. */
}

#[c2rust::header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/cmyk.h:29"]
pub mod cmyk_h {
    #[inline(always)]

    pub unsafe extern "C" fn rgb_to_cmyk(
        mut r: crate::jmorecfg_h::JSAMPLE,
        mut g: crate::jmorecfg_h::JSAMPLE,
        mut b: crate::jmorecfg_h::JSAMPLE,
        mut c: *mut crate::jmorecfg_h::JSAMPLE,
        mut m: *mut crate::jmorecfg_h::JSAMPLE,
        mut y: *mut crate::jmorecfg_h::JSAMPLE,
        mut k: *mut crate::jmorecfg_h::JSAMPLE,
    ) {
        let mut ctmp: libc::c_double = 1.0f64 - r as libc::c_double / 255.0f64;
        let mut mtmp: libc::c_double = 1.0f64 - g as libc::c_double / 255.0f64;
        let mut ytmp: libc::c_double = 1.0f64 - b as libc::c_double / 255.0f64;
        let mut ktmp: libc::c_double = if (if ctmp < mtmp { ctmp } else { mtmp }) < ytmp {
            if ctmp < mtmp {
                ctmp
            } else {
                mtmp
            }
        } else {
            ytmp
        };
        if ktmp == 1.0f64 {
            ytmp = 0.0f64;
            mtmp = ytmp;
            ctmp = mtmp
        } else {
            ctmp = (ctmp - ktmp) / (1.0f64 - ktmp);
            mtmp = (mtmp - ktmp) / (1.0f64 - ktmp);
            ytmp = (ytmp - ktmp) / (1.0f64 - ktmp)
        }
        *c = (255.0f64 - ctmp * 255.0f64 + 0.5f64) as crate::jmorecfg_h::JSAMPLE;
        *m = (255.0f64 - mtmp * 255.0f64 + 0.5f64) as crate::jmorecfg_h::JSAMPLE;
        *y = (255.0f64 - ytmp * 255.0f64 + 0.5f64) as crate::jmorecfg_h::JSAMPLE;
        *k = (255.0f64 - ktmp * 255.0f64 + 0.5f64) as crate::jmorecfg_h::JSAMPLE;
    }
    use crate::jmorecfg_h::JSAMPLE;
    /* CMYK_H */
}

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

pub use crate::cderror_h::JERR_BAD_CMAP_FILE;
pub use crate::cderror_h::JERR_BMP_BADCMAP;
pub use crate::cderror_h::JERR_BMP_BADDEPTH;
pub use crate::cderror_h::JERR_BMP_BADHEADER;
pub use crate::cderror_h::JERR_BMP_BADPLANES;
pub use crate::cderror_h::JERR_BMP_COLORSPACE;
pub use crate::cderror_h::JERR_BMP_COMPRESSED;
pub use crate::cderror_h::JERR_BMP_EMPTY;
pub use crate::cderror_h::JERR_BMP_NOT;
pub use crate::cderror_h::JERR_BMP_OUTOFRANGE;
pub use crate::cderror_h::JERR_GIF_BUG;
pub use crate::cderror_h::JERR_GIF_CODESIZE;
pub use crate::cderror_h::JERR_GIF_COLORSPACE;
pub use crate::cderror_h::JERR_GIF_IMAGENOTFOUND;
pub use crate::cderror_h::JERR_GIF_NOT;
pub use crate::cderror_h::JERR_PPM_COLORSPACE;
pub use crate::cderror_h::JERR_PPM_NONNUMERIC;
pub use crate::cderror_h::JERR_PPM_NOT;
pub use crate::cderror_h::JERR_PPM_OUTOFRANGE;
pub use crate::cderror_h::JERR_TGA_BADCMAP;
pub use crate::cderror_h::JERR_TGA_BADPARMS;
pub use crate::cderror_h::JERR_TGA_COLORSPACE;
pub use crate::cderror_h::JERR_TOO_MANY_COLORS;
pub use crate::cderror_h::JERR_UNGETC_FAILED;
pub use crate::cderror_h::JERR_UNKNOWN_FORMAT;
pub use crate::cderror_h::JERR_UNSUPPORTED_FORMAT;
pub use crate::cderror_h::JMSG_FIRSTADDONCODE;
pub use crate::cderror_h::JMSG_LASTADDONCODE;
pub use crate::cderror_h::JTRC_BMP;
pub use crate::cderror_h::JTRC_BMP_MAPPED;
pub use crate::cderror_h::JTRC_BMP_OS2;
pub use crate::cderror_h::JTRC_BMP_OS2_MAPPED;
pub use crate::cderror_h::JTRC_GIF;
pub use crate::cderror_h::JTRC_GIF_BADVERSION;
pub use crate::cderror_h::JTRC_GIF_EXTENSION;
pub use crate::cderror_h::JTRC_GIF_NONSQUARE;
pub use crate::cderror_h::JTRC_PGM;
pub use crate::cderror_h::JTRC_PGM_TEXT;
pub use crate::cderror_h::JTRC_PPM;
pub use crate::cderror_h::JTRC_PPM_TEXT;
pub use crate::cderror_h::JTRC_TGA;
pub use crate::cderror_h::JTRC_TGA_GRAY;
pub use crate::cderror_h::JTRC_TGA_MAPPED;
pub use crate::cderror_h::JWRN_GIF_BADDATA;
pub use crate::cderror_h::JWRN_GIF_CHAR;
pub use crate::cderror_h::JWRN_GIF_ENDCODE;
pub use crate::cderror_h::JWRN_GIF_NOMOREDATA;
pub use crate::cdjpeg_h::cd_progress_ptr;
pub use crate::cdjpeg_h::cdjpeg_progress_mgr;
pub use crate::cdjpeg_h::cjpeg_source_ptr;
pub use crate::cdjpeg_h::cjpeg_source_struct;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::EXT_BGRX_BLUE;
pub use crate::jmorecfg_h::EXT_BGRX_GREEN;
pub use crate::jmorecfg_h::EXT_BGRX_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_BGRX_RED;
pub use crate::jmorecfg_h::EXT_BGR_BLUE;
pub use crate::jmorecfg_h::EXT_BGR_GREEN;
pub use crate::jmorecfg_h::EXT_BGR_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_BGR_RED;
pub use crate::jmorecfg_h::EXT_RGBX_BLUE;
pub use crate::jmorecfg_h::EXT_RGBX_GREEN;
pub use crate::jmorecfg_h::EXT_RGBX_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_RGBX_RED;
pub use crate::jmorecfg_h::EXT_RGB_BLUE;
pub use crate::jmorecfg_h::EXT_RGB_GREEN;
pub use crate::jmorecfg_h::EXT_RGB_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_RGB_RED;
pub use crate::jmorecfg_h::EXT_XBGR_BLUE;
pub use crate::jmorecfg_h::EXT_XBGR_GREEN;
pub use crate::jmorecfg_h::EXT_XBGR_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_XBGR_RED;
pub use crate::jmorecfg_h::EXT_XRGB_BLUE;
pub use crate::jmorecfg_h::EXT_XRGB_GREEN;
pub use crate::jmorecfg_h::EXT_XRGB_PIXELSIZE;
pub use crate::jmorecfg_h::EXT_XRGB_RED;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::RGB_BLUE;
pub use crate::jmorecfg_h::RGB_GREEN;
pub use crate::jmorecfg_h::RGB_PIXELSIZE;
pub use crate::jmorecfg_h::RGB_RED;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::jpeg_c_coef_controller;
pub use crate::jpegint_h::jpeg_c_main_controller;
pub use crate::jpegint_h::jpeg_c_prep_controller;
pub use crate::jpegint_h::jpeg_color_converter;
pub use crate::jpegint_h::jpeg_comp_master;
pub use crate::jpegint_h::jpeg_downsampler;
pub use crate::jpegint_h::jpeg_entropy_encoder;
pub use crate::jpegint_h::jpeg_forward_dct;
pub use crate::jpegint_h::jpeg_marker_writer;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_scan_info;
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
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
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
pub use crate::src::rdbmp::cmyk_h::rgb_to_cmyk;
pub use crate::src::rdbmp::jmorecfg_h::rgb_blue;
pub use crate::src::rdbmp::jmorecfg_h::rgb_green;
pub use crate::src::rdbmp::jmorecfg_h::rgb_pixelsize;
pub use crate::src::rdbmp::jmorecfg_h::rgb_red;
pub use crate::stdlib::feof;
pub use crate::stdlib::fread;
pub use crate::stdlib::getc;
use crate::stdlib::memcpy;
pub use crate::stdlib::C2RustUnnamed_0;
pub use crate::stdlib::EOF;
/* Private version of data source object */

pub type bmp_source_ptr = *mut _bmp_source_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _bmp_source_struct {
    pub pub_0: crate::cdjpeg_h::cjpeg_source_struct,
    pub cinfo: crate::jpeglib_h::j_compress_ptr,
    pub colormap: crate::jpeglib_h::JSAMPARRAY,
    pub whole_image: crate::jpeglib_h::jvirt_sarray_ptr,
    pub source_row: crate::jmorecfg_h::JDIMENSION,
    pub row_width: crate::jmorecfg_h::JDIMENSION,
    pub bits_per_pixel: libc::c_int,
    pub cmap_length: libc::c_int,
    pub use_inversion_array: crate::jmorecfg_h::boolean,
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

pub type U_CHAR = libc::c_uchar;

pub type bmp_source_struct = _bmp_source_struct;
/* public fields */
/* back link saves passing separate parm */
/* BMP colormap (converted to my format) */
/* Needed to reverse row order */
/* Current source row number */
/* Physical width of scanlines in file */
/* remembers 8- or 24-bit format */
/* colormap length */
/* TRUE = preload the whole image, which is
stored in bottom-up order, and feed it to
the calling program in top-down order

FALSE = the calling program will maintain
its own image buffer and read the rows in
bottom-up order */
/* I/O buffer (used to buffer a single row from
disk if use_inversion_array == FALSE) */
/* !HAVE_UNSIGNED_CHAR */
/* HAVE_UNSIGNED_CHAR */

static mut alpha_index: [libc::c_int; 17] = [
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    -(1 as libc::c_int),
    3 as libc::c_int,
    3 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    -(1 as libc::c_int),
];

unsafe extern "C" fn read_byte(mut sinfo: bmp_source_ptr) -> libc::c_int
/* Read next byte from BMP file */ {
    let mut infile: *mut crate::stdlib::FILE = (*sinfo).pub_0.input_file;
    let mut c: libc::c_int = 0;
    c = crate::stdlib::getc(infile);
    if c == crate::stdlib::EOF {
        (*(*(*sinfo).cinfo).err).msg_code = crate::src::jerror::JERR_INPUT_EOF as libc::c_int;
        Some(
            (*(*(*sinfo).cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            (*sinfo).cinfo as crate::jpeglib_h::j_common_ptr
        );
    }
    return c;
}

unsafe extern "C" fn read_colormap(
    mut sinfo: bmp_source_ptr,
    mut cmaplen: libc::c_int,
    mut mapentrysize: libc::c_int,
)
/* Read the colormap from a BMP file */
{
    let mut i: libc::c_int = 0;
    let mut gray: libc::c_int = 1 as libc::c_int;
    match mapentrysize {
        3 => {
            /* BGR format (occurs in OS/2 files) */
            i = 0 as libc::c_int;
            while i < cmaplen {
                *(*(*sinfo).colormap.offset(2 as libc::c_int as isize)).offset(i as isize) =
                    read_byte(sinfo) as crate::jmorecfg_h::JSAMPLE;
                *(*(*sinfo).colormap.offset(1 as libc::c_int as isize)).offset(i as isize) =
                    read_byte(sinfo) as crate::jmorecfg_h::JSAMPLE;
                *(*(*sinfo).colormap.offset(0 as libc::c_int as isize)).offset(i as isize) =
                    read_byte(sinfo) as crate::jmorecfg_h::JSAMPLE;
                if *(*(*sinfo).colormap.offset(2 as libc::c_int as isize)).offset(i as isize)
                    as libc::c_int
                    != *(*(*sinfo).colormap.offset(1 as libc::c_int as isize)).offset(i as isize)
                        as libc::c_int
                    || *(*(*sinfo).colormap.offset(1 as libc::c_int as isize)).offset(i as isize)
                        as libc::c_int
                        != *(*(*sinfo).colormap.offset(0 as libc::c_int as isize))
                            .offset(i as isize) as libc::c_int
                {
                    gray = 0 as libc::c_int
                }
                i += 1
            }
        }
        4 => {
            /* BGR0 format (occurs in MS Windows files) */
            i = 0 as libc::c_int;
            while i < cmaplen {
                *(*(*sinfo).colormap.offset(2 as libc::c_int as isize)).offset(i as isize) =
                    read_byte(sinfo) as crate::jmorecfg_h::JSAMPLE;
                *(*(*sinfo).colormap.offset(1 as libc::c_int as isize)).offset(i as isize) =
                    read_byte(sinfo) as crate::jmorecfg_h::JSAMPLE;
                *(*(*sinfo).colormap.offset(0 as libc::c_int as isize)).offset(i as isize) =
                    read_byte(sinfo) as crate::jmorecfg_h::JSAMPLE;
                read_byte(sinfo);
                if *(*(*sinfo).colormap.offset(2 as libc::c_int as isize)).offset(i as isize)
                    as libc::c_int
                    != *(*(*sinfo).colormap.offset(1 as libc::c_int as isize)).offset(i as isize)
                        as libc::c_int
                    || *(*(*sinfo).colormap.offset(1 as libc::c_int as isize)).offset(i as isize)
                        as libc::c_int
                        != *(*(*sinfo).colormap.offset(0 as libc::c_int as isize))
                            .offset(i as isize) as libc::c_int
                {
                    gray = 0 as libc::c_int
                }
                i += 1
            }
        }
        _ => {
            (*(*(*sinfo).cinfo).err).msg_code = crate::cderror_h::JERR_BMP_BADCMAP as libc::c_int;
            Some(
                (*(*(*sinfo).cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                (*sinfo).cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
    }
    if (*(*sinfo).cinfo).in_color_space as libc::c_uint
        == crate::jpeglib_h::JCS_UNKNOWN as libc::c_int as libc::c_uint
        && gray != 0
    {
        (*(*sinfo).cinfo).in_color_space = crate::jpeglib_h::JCS_GRAYSCALE
    }
    if (*(*sinfo).cinfo).in_color_space as libc::c_uint
        == crate::jpeglib_h::JCS_GRAYSCALE as libc::c_int as libc::c_uint
        && gray == 0
    {
        (*(*(*sinfo).cinfo).err).msg_code =
            crate::src::jerror::JERR_BAD_IN_COLORSPACE as libc::c_int;
        Some(
            (*(*(*sinfo).cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            (*sinfo).cinfo as crate::jpeglib_h::j_common_ptr
        );
    };
}
/*
 * Read one row of pixels.
 * The image has been read into the whole_image array, but is otherwise
 * unprocessed.  We must read it out in top-to-bottom row order, and if
 * it is an 8-bit image, we must expand colormapped pixels to 24bit format.
 */

unsafe extern "C" fn get_8bit_row(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut sinfo: crate::cdjpeg_h::cjpeg_source_ptr,
) -> crate::jmorecfg_h::JDIMENSION
/* This version is for reading 8-bit colormap indexes */ {
    let mut source: bmp_source_ptr = sinfo as bmp_source_ptr;
    let mut colormap: crate::jpeglib_h::JSAMPARRAY = (*source).colormap;
    let mut cmaplen: libc::c_int = (*source).cmap_length;
    let mut image_ptr: crate::jpeglib_h::JSAMPARRAY = 0 as *mut crate::jpeglib_h::JSAMPROW;
    let mut t: libc::c_int = 0;
    let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    if (*source).use_inversion_array != 0 {
        /* Fetch next row from virtual array */
        (*source).source_row = (*source).source_row.wrapping_sub(1);
        image_ptr = Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            (*source).whole_image,
            (*source).source_row,
            1 as libc::c_int as crate::jmorecfg_h::JDIMENSION,
            crate::jmorecfg_h::FALSE,
        );
        inptr = *image_ptr.offset(0 as libc::c_int as isize)
    } else {
        if !(crate::stdlib::fread(
            (*source).iobuffer as *mut libc::c_void,
            1 as libc::c_int as crate::stddef_h::size_t,
            (*source).row_width as crate::stddef_h::size_t,
            (*source).pub_0.input_file,
        ) == (*source).row_width as crate::stddef_h::size_t)
        {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_INPUT_EOF as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        inptr = (*source).iobuffer
    }
    /* Expand the colormap indexes to real data */
    outptr = *(*source).pub_0.buffer.offset(0 as libc::c_int as isize);
    if (*cinfo).in_color_space as libc::c_uint
        == crate::jpeglib_h::JCS_GRAYSCALE as libc::c_int as libc::c_uint
    {
        col = (*cinfo).image_width;
        while col > 0 as libc::c_int as libc::c_uint {
            let fresh0 = inptr;
            inptr = inptr.offset(1);
            t = *fresh0 as libc::c_int;
            if t >= cmaplen {
                (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BMP_OUTOFRANGE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            let fresh1 = outptr;
            outptr = outptr.offset(1);
            *fresh1 = *(*colormap.offset(0 as libc::c_int as isize)).offset(t as isize);
            col = col.wrapping_sub(1)
        }
    } else if (*cinfo).in_color_space as libc::c_uint
        == crate::jpeglib_h::JCS_CMYK as libc::c_int as libc::c_uint
    {
        col = (*cinfo).image_width;
        while col > 0 as libc::c_int as libc::c_uint {
            let fresh2 = inptr;
            inptr = inptr.offset(1);
            t = *fresh2 as libc::c_int;
            if t >= cmaplen {
                (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BMP_OUTOFRANGE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            rgb_to_cmyk(
                *(*colormap.offset(0 as libc::c_int as isize)).offset(t as isize),
                *(*colormap.offset(1 as libc::c_int as isize)).offset(t as isize),
                *(*colormap.offset(2 as libc::c_int as isize)).offset(t as isize),
                outptr,
                outptr.offset(1 as libc::c_int as isize),
                outptr.offset(2 as libc::c_int as isize),
                outptr.offset(3 as libc::c_int as isize),
            );
            outptr = outptr.offset(4 as libc::c_int as isize);
            col = col.wrapping_sub(1)
        }
    } else {
        let mut rindex: libc::c_int = rgb_red[(*cinfo).in_color_space as usize];
        let mut gindex: libc::c_int = rgb_green[(*cinfo).in_color_space as usize];
        let mut bindex: libc::c_int = rgb_blue[(*cinfo).in_color_space as usize];
        let mut aindex: libc::c_int = alpha_index[(*cinfo).in_color_space as usize];
        let mut ps: libc::c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
        if aindex >= 0 as libc::c_int {
            col = (*cinfo).image_width;
            while col > 0 as libc::c_int as libc::c_uint {
                let fresh3 = inptr;
                inptr = inptr.offset(1);
                t = *fresh3 as libc::c_int;
                if t >= cmaplen {
                    (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BMP_OUTOFRANGE as libc::c_int;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr
                    );
                }
                *outptr.offset(rindex as isize) =
                    *(*colormap.offset(0 as libc::c_int as isize)).offset(t as isize);
                *outptr.offset(gindex as isize) =
                    *(*colormap.offset(1 as libc::c_int as isize)).offset(t as isize);
                *outptr.offset(bindex as isize) =
                    *(*colormap.offset(2 as libc::c_int as isize)).offset(t as isize);
                *outptr.offset(aindex as isize) = 0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
                outptr = outptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0 as libc::c_int as libc::c_uint {
                let fresh4 = inptr;
                inptr = inptr.offset(1);
                t = *fresh4 as libc::c_int;
                if t >= cmaplen {
                    (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BMP_OUTOFRANGE as libc::c_int;
                    Some(
                        (*(*cinfo).err)
                            .error_exit
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr
                    );
                }
                *outptr.offset(rindex as isize) =
                    *(*colormap.offset(0 as libc::c_int as isize)).offset(t as isize);
                *outptr.offset(gindex as isize) =
                    *(*colormap.offset(1 as libc::c_int as isize)).offset(t as isize);
                *outptr.offset(bindex as isize) =
                    *(*colormap.offset(2 as libc::c_int as isize)).offset(t as isize);
                outptr = outptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        }
    }
    return 1 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
}

unsafe extern "C" fn get_24bit_row(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut sinfo: crate::cdjpeg_h::cjpeg_source_ptr,
) -> crate::jmorecfg_h::JDIMENSION
/* This version is for reading 24-bit pixels */ {
    let mut source: bmp_source_ptr = sinfo as bmp_source_ptr;
    let mut image_ptr: crate::jpeglib_h::JSAMPARRAY = 0 as *mut crate::jpeglib_h::JSAMPROW;
    let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    if (*source).use_inversion_array != 0 {
        /* Fetch next row from virtual array */
        (*source).source_row = (*source).source_row.wrapping_sub(1);
        image_ptr = Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            (*source).whole_image,
            (*source).source_row,
            1 as libc::c_int as crate::jmorecfg_h::JDIMENSION,
            crate::jmorecfg_h::FALSE,
        );
        inptr = *image_ptr.offset(0 as libc::c_int as isize)
    } else {
        if !(crate::stdlib::fread(
            (*source).iobuffer as *mut libc::c_void,
            1 as libc::c_int as crate::stddef_h::size_t,
            (*source).row_width as crate::stddef_h::size_t,
            (*source).pub_0.input_file,
        ) == (*source).row_width as crate::stddef_h::size_t)
        {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_INPUT_EOF as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        inptr = (*source).iobuffer
    }
    /* Transfer data.  Note source values are in BGR order
     * (even though Microsoft's own documents say the opposite).
     */
    outptr = *(*source).pub_0.buffer.offset(0 as libc::c_int as isize);
    if (*cinfo).in_color_space as libc::c_uint
        == crate::jpeglib_h::JCS_EXT_BGR as libc::c_int as libc::c_uint
    {
        crate::stdlib::memcpy(
            outptr as *mut libc::c_void,
            inptr as *const libc::c_void,
            (*source).row_width as crate::stddef_h::size_t,
        );
    } else if (*cinfo).in_color_space as libc::c_uint
        == crate::jpeglib_h::JCS_CMYK as libc::c_int as libc::c_uint
    {
        col = (*cinfo).image_width;
        while col > 0 as libc::c_int as libc::c_uint {
            /* can omit GETJSAMPLE() safely */
            let fresh5 = inptr; /* can omit GETJSAMPLE() safely */
            inptr = inptr.offset(1); /* can omit GETJSAMPLE() safely */
            let mut b: crate::jmorecfg_h::JSAMPLE = *fresh5;
            let fresh6 = inptr;
            inptr = inptr.offset(1);
            let mut g: crate::jmorecfg_h::JSAMPLE = *fresh6;
            let fresh7 = inptr;
            inptr = inptr.offset(1);
            let mut r: crate::jmorecfg_h::JSAMPLE = *fresh7;
            rgb_to_cmyk(
                r,
                g,
                b,
                outptr,
                outptr.offset(1 as libc::c_int as isize),
                outptr.offset(2 as libc::c_int as isize),
                outptr.offset(3 as libc::c_int as isize),
            );
            outptr = outptr.offset(4 as libc::c_int as isize);
            col = col.wrapping_sub(1)
        }
    } else {
        let mut rindex: libc::c_int = rgb_red[(*cinfo).in_color_space as usize];
        let mut gindex: libc::c_int = rgb_green[(*cinfo).in_color_space as usize];
        let mut bindex: libc::c_int = rgb_blue[(*cinfo).in_color_space as usize];
        let mut aindex: libc::c_int = alpha_index[(*cinfo).in_color_space as usize];
        let mut ps: libc::c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
        if aindex >= 0 as libc::c_int {
            col = (*cinfo).image_width;
            while col > 0 as libc::c_int as libc::c_uint {
                let fresh8 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(bindex as isize) = *fresh8;
                let fresh9 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(gindex as isize) = *fresh9;
                let fresh10 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(rindex as isize) = *fresh10;
                *outptr.offset(aindex as isize) = 0xff as libc::c_int as crate::jmorecfg_h::JSAMPLE;
                outptr = outptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        } else {
            col = (*cinfo).image_width;
            while col > 0 as libc::c_int as libc::c_uint {
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
    return 1 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
}

unsafe extern "C" fn get_32bit_row(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut sinfo: crate::cdjpeg_h::cjpeg_source_ptr,
) -> crate::jmorecfg_h::JDIMENSION
/* This version is for reading 32-bit pixels */ {
    let mut source: bmp_source_ptr = sinfo as bmp_source_ptr;
    let mut image_ptr: crate::jpeglib_h::JSAMPARRAY = 0 as *mut crate::jpeglib_h::JSAMPROW;
    let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    if (*source).use_inversion_array != 0 {
        /* Fetch next row from virtual array */
        (*source).source_row = (*source).source_row.wrapping_sub(1);
        image_ptr = Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            (*source).whole_image,
            (*source).source_row,
            1 as libc::c_int as crate::jmorecfg_h::JDIMENSION,
            crate::jmorecfg_h::FALSE,
        );
        inptr = *image_ptr.offset(0 as libc::c_int as isize)
    } else {
        if !(crate::stdlib::fread(
            (*source).iobuffer as *mut libc::c_void,
            1 as libc::c_int as crate::stddef_h::size_t,
            (*source).row_width as crate::stddef_h::size_t,
            (*source).pub_0.input_file,
        ) == (*source).row_width as crate::stddef_h::size_t)
        {
            (*(*cinfo).err).msg_code = crate::src::jerror::JERR_INPUT_EOF as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        inptr = (*source).iobuffer
    }
    /* Transfer data.  Note source values are in BGR order
     * (even though Microsoft's own documents say the opposite).
     */
    outptr = *(*source).pub_0.buffer.offset(0 as libc::c_int as isize);
    if (*cinfo).in_color_space as libc::c_uint
        == crate::jpeglib_h::JCS_EXT_BGRX as libc::c_int as libc::c_uint
        || (*cinfo).in_color_space as libc::c_uint
            == crate::jpeglib_h::JCS_EXT_BGRA as libc::c_int as libc::c_uint
    {
        crate::stdlib::memcpy(
            outptr as *mut libc::c_void,
            inptr as *const libc::c_void,
            (*source).row_width as crate::stddef_h::size_t,
        );
    } else if (*cinfo).in_color_space as libc::c_uint
        == crate::jpeglib_h::JCS_CMYK as libc::c_int as libc::c_uint
    {
        col = (*cinfo).image_width;
        while col > 0 as libc::c_int as libc::c_uint {
            /* can omit GETJSAMPLE() safely */
            let fresh14 = inptr; /* skip the 4th byte (Alpha channel) */
            inptr = inptr.offset(1); /* can omit GETJSAMPLE() safely */
            let mut b: crate::jmorecfg_h::JSAMPLE = *fresh14; /* can omit GETJSAMPLE() safely */
            let fresh15 = inptr; /* skip the 4th byte (Alpha channel) */
            inptr = inptr.offset(1);
            let mut g: crate::jmorecfg_h::JSAMPLE = *fresh15;
            let fresh16 = inptr;
            inptr = inptr.offset(1);
            let mut r: crate::jmorecfg_h::JSAMPLE = *fresh16;
            rgb_to_cmyk(
                r,
                g,
                b,
                outptr,
                outptr.offset(1 as libc::c_int as isize),
                outptr.offset(2 as libc::c_int as isize),
                outptr.offset(3 as libc::c_int as isize),
            );
            inptr = inptr.offset(1);
            outptr = outptr.offset(4 as libc::c_int as isize);
            col = col.wrapping_sub(1)
        }
    } else {
        let mut rindex: libc::c_int = rgb_red[(*cinfo).in_color_space as usize];
        let mut gindex: libc::c_int = rgb_green[(*cinfo).in_color_space as usize];
        let mut bindex: libc::c_int = rgb_blue[(*cinfo).in_color_space as usize];
        let mut aindex: libc::c_int = alpha_index[(*cinfo).in_color_space as usize];
        let mut ps: libc::c_int = rgb_pixelsize[(*cinfo).in_color_space as usize];
        if aindex >= 0 as libc::c_int {
            col = (*cinfo).image_width;
            while col > 0 as libc::c_int as libc::c_uint {
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
            while col > 0 as libc::c_int as libc::c_uint {
                let fresh21 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(bindex as isize) = *fresh21;
                let fresh22 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(gindex as isize) = *fresh22;
                let fresh23 = inptr;
                inptr = inptr.offset(1);
                *outptr.offset(rindex as isize) = *fresh23;
                inptr = inptr.offset(1);
                outptr = outptr.offset(ps as isize);
                col = col.wrapping_sub(1)
            }
        }
    }
    return 1 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
}
/*
 * This method loads the image into whole_image during the first call on
 * get_pixel_rows.  The get_pixel_rows pointer is then adjusted to call
 * get_8bit_row, get_24bit_row, or get_32bit_row on subsequent calls.
 */

unsafe extern "C" fn preload_image(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut sinfo: crate::cdjpeg_h::cjpeg_source_ptr,
) -> crate::jmorecfg_h::JDIMENSION {
    let mut source: bmp_source_ptr = sinfo as bmp_source_ptr;
    let mut infile: *mut crate::stdlib::FILE = (*source).pub_0.input_file;
    let mut out_ptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut image_ptr: crate::jpeglib_h::JSAMPARRAY = 0 as *mut crate::jpeglib_h::JSAMPROW;
    let mut row: crate::jmorecfg_h::JDIMENSION = 0;
    let mut progress: crate::cdjpeg_h::cd_progress_ptr =
        (*cinfo).progress as crate::cdjpeg_h::cd_progress_ptr;
    /* Read the data into a virtual array in input-file row order. */
    row = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
    while row < (*cinfo).image_height {
        if !progress.is_null() {
            (*progress).pub_0.pass_counter = row as libc::c_long;
            (*progress).pub_0.pass_limit = (*cinfo).image_height as libc::c_long;
            Some(
                (*progress)
                    .pub_0
                    .progress_monitor
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        image_ptr = Some(
            (*(*cinfo).mem)
                .access_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            (*source).whole_image,
            row,
            1 as libc::c_int as crate::jmorecfg_h::JDIMENSION,
            crate::jmorecfg_h::TRUE,
        );
        out_ptr = *image_ptr.offset(0 as libc::c_int as isize);
        if crate::stdlib::fread(
            out_ptr as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            (*source).row_width as libc::c_ulong,
            infile,
        ) != (*source).row_width as libc::c_ulong
        {
            if crate::stdlib::feof(infile) != 0 {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_INPUT_EOF as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            } else {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_FILE_READ as libc::c_int;
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
        row = row.wrapping_add(1)
    }
    if !progress.is_null() {
        (*progress).completed_extra_passes += 1
    }
    /* Set up to read from the virtual array in top-to-bottom order */
    match (*source).bits_per_pixel {
        8 => {
            (*source).pub_0.get_pixel_rows = Some(
                get_8bit_row
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_compress_ptr,
                        _: crate::cdjpeg_h::cjpeg_source_ptr,
                    ) -> crate::jmorecfg_h::JDIMENSION,
            )
        }
        24 => {
            (*source).pub_0.get_pixel_rows = Some(
                get_24bit_row
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_compress_ptr,
                        _: crate::cdjpeg_h::cjpeg_source_ptr,
                    ) -> crate::jmorecfg_h::JDIMENSION,
            )
        }
        32 => {
            (*source).pub_0.get_pixel_rows = Some(
                get_32bit_row
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_compress_ptr,
                        _: crate::cdjpeg_h::cjpeg_source_ptr,
                    ) -> crate::jmorecfg_h::JDIMENSION,
            )
        }
        _ => {
            (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BMP_BADDEPTH as libc::c_int;
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
    (*source).source_row = (*cinfo).image_height;
    /* And read the first row */
    return Some(
        (*source)
            .pub_0
            .get_pixel_rows
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, sinfo);
}
/*
 * Read the file header; return image size and component count.
 */

unsafe extern "C" fn start_input_bmp(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut sinfo: crate::cdjpeg_h::cjpeg_source_ptr,
) {
    let mut source: bmp_source_ptr = sinfo as bmp_source_ptr; /* 0 indicates no colormap */
    let mut bmpfileheader: [U_CHAR; 14] = [0; 14];
    let mut bmpinfoheader: [U_CHAR; 64] = [0; 64];
    let mut bfOffBits: libc::c_uint = 0;
    let mut headerSize: libc::c_uint = 0;
    let mut biWidth: libc::c_int = 0;
    let mut biHeight: libc::c_int = 0;
    let mut biPlanes: libc::c_ushort = 0;
    let mut biCompression: libc::c_uint = 0;
    let mut biXPelsPerMeter: libc::c_int = 0;
    let mut biYPelsPerMeter: libc::c_int = 0;
    let mut biClrUsed: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut mapentrysize: libc::c_int = 0 as libc::c_int;
    let mut bPad: libc::c_int = 0;
    let mut row_width: crate::jmorecfg_h::JDIMENSION =
        0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
    /* Read and verify the bitmap file header */
    if !(crate::stdlib::fread(
        bmpfileheader.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int as crate::stddef_h::size_t,
        14 as libc::c_int as crate::stddef_h::size_t,
        (*source).pub_0.input_file,
    ) == 14 as libc::c_int as crate::stddef_h::size_t)
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_INPUT_EOF as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if bmpfileheader[0 as libc::c_int as usize] as libc::c_int as libc::c_ushort as libc::c_int
        + ((bmpfileheader[(0 as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int
            as libc::c_ushort as libc::c_int)
            << 8 as libc::c_int)
        != 0x4d42 as libc::c_int
    {
        /* 'BM' */
        (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BMP_NOT as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    bfOffBits = (bmpfileheader[10 as libc::c_int as usize] as libc::c_int as libc::c_uint)
        .wrapping_add(
            (bmpfileheader[(10 as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int
                as libc::c_uint)
                << 8 as libc::c_int,
        )
        .wrapping_add(
            (bmpfileheader[(10 as libc::c_int + 2 as libc::c_int) as usize] as libc::c_int
                as libc::c_uint)
                << 16 as libc::c_int,
        )
        .wrapping_add(
            (bmpfileheader[(10 as libc::c_int + 3 as libc::c_int) as usize] as libc::c_int
                as libc::c_uint)
                << 24 as libc::c_int,
        );
    /* We ignore the remaining fileheader fields */
    /* The infoheader might be 12 bytes (OS/2 1.x), 40 bytes (Windows),
     * or 64 bytes (OS/2 2.x).  Check the first 4 bytes to find out which.
     */
    if !(crate::stdlib::fread(
        bmpinfoheader.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int as crate::stddef_h::size_t,
        4 as libc::c_int as crate::stddef_h::size_t,
        (*source).pub_0.input_file,
    ) == 4 as libc::c_int as crate::stddef_h::size_t)
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_INPUT_EOF as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    headerSize = (bmpinfoheader[0 as libc::c_int as usize] as libc::c_int as libc::c_uint)
        .wrapping_add(
            (bmpinfoheader[(0 as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int
                as libc::c_uint)
                << 8 as libc::c_int,
        )
        .wrapping_add(
            (bmpinfoheader[(0 as libc::c_int + 2 as libc::c_int) as usize] as libc::c_int
                as libc::c_uint)
                << 16 as libc::c_int,
        )
        .wrapping_add(
            (bmpinfoheader[(0 as libc::c_int + 3 as libc::c_int) as usize] as libc::c_int
                as libc::c_uint)
                << 24 as libc::c_int,
        );
    if headerSize < 12 as libc::c_int as libc::c_uint
        || headerSize > 64 as libc::c_int as libc::c_uint
    {
        (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BMP_BADHEADER as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if !(crate::stdlib::fread(
        bmpinfoheader.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut libc::c_void,
        1 as libc::c_int as crate::stddef_h::size_t,
        headerSize.wrapping_sub(4 as libc::c_int as libc::c_uint) as crate::stddef_h::size_t,
        (*source).pub_0.input_file,
    ) == headerSize.wrapping_sub(4 as libc::c_int as libc::c_uint) as crate::stddef_h::size_t)
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_INPUT_EOF as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    match headerSize {
        12 => {
            /* Decode OS/2 1.x header (Microsoft calls this a BITMAPCOREHEADER) */
            biWidth = bmpinfoheader[4 as libc::c_int as usize] as libc::c_int as libc::c_ushort
                as libc::c_int
                + ((bmpinfoheader[(4 as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int
                    as libc::c_ushort as libc::c_int)
                    << 8 as libc::c_int); /* OS/2 uses RGBTRIPLE colormap */
            biHeight = bmpinfoheader[6 as libc::c_int as usize] as libc::c_int as libc::c_ushort
                as libc::c_int
                + ((bmpinfoheader[(6 as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int
                    as libc::c_ushort as libc::c_int)
                    << 8 as libc::c_int);
            biPlanes = (bmpinfoheader[8 as libc::c_int as usize] as libc::c_int as libc::c_ushort
                as libc::c_int
                + ((bmpinfoheader[(8 as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int
                    as libc::c_ushort as libc::c_int)
                    << 8 as libc::c_int)) as libc::c_ushort;
            (*source).bits_per_pixel = bmpinfoheader[10 as libc::c_int as usize] as libc::c_int
                as libc::c_ushort as libc::c_int
                + ((bmpinfoheader[(10 as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int
                    as libc::c_ushort as libc::c_int)
                    << 8 as libc::c_int);
            match (*source).bits_per_pixel {
                8 => {
                    /* colormapped image */
                    mapentrysize = 3 as libc::c_int;
                    (*(*cinfo).err).msg_code = crate::cderror_h::JTRC_BMP_OS2_MAPPED as libc::c_int;
                    (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = biWidth;
                    (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = biHeight;
                    Some(
                        (*(*cinfo).err)
                            .emit_message
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr,
                        1 as libc::c_int,
                    );
                }
                24 => {
                    /* RGB image */
                    (*(*cinfo).err).msg_code = crate::cderror_h::JTRC_BMP_OS2 as libc::c_int;
                    (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = biWidth;
                    (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = biHeight;
                    Some(
                        (*(*cinfo).err)
                            .emit_message
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr,
                        1 as libc::c_int,
                    );
                }
                _ => {
                    (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BMP_BADDEPTH as libc::c_int;
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
        }
        40 | 64 => {
            /* Decode Windows 3.x header (Microsoft calls this a BITMAPINFOHEADER) */
            /* or OS/2 2.x header, which has additional fields that we ignore */
            biWidth = (bmpinfoheader[4 as libc::c_int as usize] as libc::c_int as libc::c_uint)
                .wrapping_add(
                    (bmpinfoheader[(4 as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int
                        as libc::c_uint)
                        << 8 as libc::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(4 as libc::c_int + 2 as libc::c_int) as usize] as libc::c_int
                        as libc::c_uint)
                        << 16 as libc::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(4 as libc::c_int + 3 as libc::c_int) as usize] as libc::c_int
                        as libc::c_uint)
                        << 24 as libc::c_int,
                ) as libc::c_int;
            biHeight = (bmpinfoheader[8 as libc::c_int as usize] as libc::c_int as libc::c_uint)
                .wrapping_add(
                    (bmpinfoheader[(8 as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int
                        as libc::c_uint)
                        << 8 as libc::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(8 as libc::c_int + 2 as libc::c_int) as usize] as libc::c_int
                        as libc::c_uint)
                        << 16 as libc::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(8 as libc::c_int + 3 as libc::c_int) as usize] as libc::c_int
                        as libc::c_uint)
                        << 24 as libc::c_int,
                ) as libc::c_int;
            biPlanes = (bmpinfoheader[12 as libc::c_int as usize] as libc::c_int as libc::c_ushort
                as libc::c_int
                + ((bmpinfoheader[(12 as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int
                    as libc::c_ushort as libc::c_int)
                    << 8 as libc::c_int)) as libc::c_ushort;
            (*source).bits_per_pixel = bmpinfoheader[14 as libc::c_int as usize] as libc::c_int
                as libc::c_ushort as libc::c_int
                + ((bmpinfoheader[(14 as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int
                    as libc::c_ushort as libc::c_int)
                    << 8 as libc::c_int);
            biCompression = (bmpinfoheader[16 as libc::c_int as usize] as libc::c_int
                as libc::c_uint)
                .wrapping_add(
                    (bmpinfoheader[(16 as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int
                        as libc::c_uint)
                        << 8 as libc::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(16 as libc::c_int + 2 as libc::c_int) as usize] as libc::c_int
                        as libc::c_uint)
                        << 16 as libc::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(16 as libc::c_int + 3 as libc::c_int) as usize] as libc::c_int
                        as libc::c_uint)
                        << 24 as libc::c_int,
                );
            biXPelsPerMeter = (bmpinfoheader[24 as libc::c_int as usize] as libc::c_int
                as libc::c_uint)
                .wrapping_add(
                    (bmpinfoheader[(24 as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int
                        as libc::c_uint)
                        << 8 as libc::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(24 as libc::c_int + 2 as libc::c_int) as usize] as libc::c_int
                        as libc::c_uint)
                        << 16 as libc::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(24 as libc::c_int + 3 as libc::c_int) as usize] as libc::c_int
                        as libc::c_uint)
                        << 24 as libc::c_int,
                ) as libc::c_int;
            biYPelsPerMeter = (bmpinfoheader[28 as libc::c_int as usize] as libc::c_int
                as libc::c_uint)
                .wrapping_add(
                    (bmpinfoheader[(28 as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int
                        as libc::c_uint)
                        << 8 as libc::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(28 as libc::c_int + 2 as libc::c_int) as usize] as libc::c_int
                        as libc::c_uint)
                        << 16 as libc::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(28 as libc::c_int + 3 as libc::c_int) as usize] as libc::c_int
                        as libc::c_uint)
                        << 24 as libc::c_int,
                ) as libc::c_int;
            biClrUsed = (bmpinfoheader[32 as libc::c_int as usize] as libc::c_int as libc::c_uint)
                .wrapping_add(
                    (bmpinfoheader[(32 as libc::c_int + 1 as libc::c_int) as usize] as libc::c_int
                        as libc::c_uint)
                        << 8 as libc::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(32 as libc::c_int + 2 as libc::c_int) as usize] as libc::c_int
                        as libc::c_uint)
                        << 16 as libc::c_int,
                )
                .wrapping_add(
                    (bmpinfoheader[(32 as libc::c_int + 3 as libc::c_int) as usize] as libc::c_int
                        as libc::c_uint)
                        << 24 as libc::c_int,
                );
            /* biSizeImage, biClrImportant fields are ignored */
            match (*source).bits_per_pixel {
                8 => {
                    mapentrysize = 4 as libc::c_int; /* Windows uses RGBQUAD colormap */
                    (*(*cinfo).err).msg_code = crate::cderror_h::JTRC_BMP_MAPPED as libc::c_int;
                    (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = biWidth;
                    (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = biHeight;
                    Some(
                        (*(*cinfo).err)
                            .emit_message
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr,
                        1 as libc::c_int,
                    );
                }
                24 => {
                    /* colormapped image */
                    /* RGB image */
                    (*(*cinfo).err).msg_code = crate::cderror_h::JTRC_BMP as libc::c_int;
                    (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = biWidth;
                    (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = biHeight;
                    Some(
                        (*(*cinfo).err)
                            .emit_message
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr,
                        1 as libc::c_int,
                    );
                }
                32 => {
                    /* RGB image + Alpha channel */
                    (*(*cinfo).err).msg_code = crate::cderror_h::JTRC_BMP as libc::c_int;
                    (*(*cinfo).err).msg_parm.i[0 as libc::c_int as usize] = biWidth;
                    (*(*cinfo).err).msg_parm.i[1 as libc::c_int as usize] = biHeight;
                    Some(
                        (*(*cinfo).err)
                            .emit_message
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        cinfo as crate::jpeglib_h::j_common_ptr,
                        1 as libc::c_int,
                    );
                }
                _ => {
                    (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BMP_BADDEPTH as libc::c_int;
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
            if biCompression != 0 as libc::c_int as libc::c_uint {
                (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BMP_COMPRESSED as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            if biXPelsPerMeter > 0 as libc::c_int && biYPelsPerMeter > 0 as libc::c_int {
                /* Set JFIF density parameters from the BMP data */
                (*cinfo).X_density =
                    (biXPelsPerMeter / 100 as libc::c_int) as crate::jmorecfg_h::UINT16;
                (*cinfo).Y_density =
                    (biYPelsPerMeter / 100 as libc::c_int) as crate::jmorecfg_h::UINT16;
                (*cinfo).density_unit = 2 as libc::c_int as crate::jmorecfg_h::UINT8
                /* 100 cm per meter */
                /* dots/cm */
            }
        }
        _ => {
            (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BMP_BADHEADER as libc::c_int;
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
            return;
        }
    }
    if biWidth <= 0 as libc::c_int
        || biHeight <= 0 as libc::c_int
        || biWidth as libc::c_long > 0x7fffffff as libc::c_long
        || biHeight as libc::c_long > 0x7fffffff as libc::c_long
    {
        (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BMP_EMPTY as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    if biPlanes as libc::c_int != 1 as libc::c_int {
        (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BMP_BADPLANES as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Compute distance to bitmap data --- will adjust for colormap below */
    bPad = bfOffBits.wrapping_sub(headerSize.wrapping_add(14 as libc::c_int as libc::c_uint))
        as libc::c_int;
    /* Read the colormap, if any */
    if mapentrysize > 0 as libc::c_int {
        if biClrUsed <= 0 as libc::c_int as libc::c_uint {
            biClrUsed = 256 as libc::c_int as libc::c_uint
        } else if biClrUsed > 256 as libc::c_int as libc::c_uint {
            (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BMP_BADCMAP as libc::c_int; /* assume it's 256 */
            Some(
                (*(*cinfo).err)
                    .error_exit
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                cinfo as crate::jpeglib_h::j_common_ptr
            );
        }
        /* Allocate space to store the colormap */
        (*source).colormap = Some(
            (*(*cinfo).mem)
                .alloc_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            biClrUsed,
            3 as libc::c_int as crate::jmorecfg_h::JDIMENSION,
        );
        (*source).cmap_length = biClrUsed as libc::c_int;
        /* and read it from the file */
        read_colormap(source, biClrUsed as libc::c_int, mapentrysize);
        /* account for size of colormap */
        bPad = (bPad as libc::c_uint)
            .wrapping_sub(biClrUsed.wrapping_mul(mapentrysize as libc::c_uint))
            as libc::c_int as libc::c_int
    }
    /* Skip any remaining pad bytes */
    if bPad < 0 as libc::c_int {
        /* incorrect bfOffBits value? */
        (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BMP_BADHEADER as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    loop {
        bPad -= 1;
        if !(bPad >= 0 as libc::c_int) {
            break;
        }
        read_byte(source);
    }
    /* Compute row width in file, including padding to 4-byte boundary */
    match (*source).bits_per_pixel {
        8 => {
            if (*cinfo).in_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_UNKNOWN as libc::c_int as libc::c_uint
            {
                (*cinfo).in_color_space = crate::jpeglib_h::JCS_EXT_RGB
            }
            if (*cinfo).in_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_RGB as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    >= crate::jpeglib_h::JCS_EXT_RGB as libc::c_int as libc::c_uint
                    && (*cinfo).in_color_space as libc::c_uint
                        <= crate::jpeglib_h::JCS_EXT_ARGB as libc::c_int as libc::c_uint
            {
                (*cinfo).input_components = rgb_pixelsize[(*cinfo).in_color_space as usize]
            } else if (*cinfo).in_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_GRAYSCALE as libc::c_int as libc::c_uint
            {
                (*cinfo).input_components = 1 as libc::c_int
            } else if (*cinfo).in_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_CMYK as libc::c_int as libc::c_uint
            {
                (*cinfo).input_components = 4 as libc::c_int
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_BAD_IN_COLORSPACE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            row_width = biWidth as crate::jmorecfg_h::JDIMENSION
        }
        24 => {
            if (*cinfo).in_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_UNKNOWN as libc::c_int as libc::c_uint
            {
                (*cinfo).in_color_space = crate::jpeglib_h::JCS_EXT_BGR
            }
            if (*cinfo).in_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_RGB as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    >= crate::jpeglib_h::JCS_EXT_RGB as libc::c_int as libc::c_uint
                    && (*cinfo).in_color_space as libc::c_uint
                        <= crate::jpeglib_h::JCS_EXT_ARGB as libc::c_int as libc::c_uint
            {
                (*cinfo).input_components = rgb_pixelsize[(*cinfo).in_color_space as usize]
            } else if (*cinfo).in_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_CMYK as libc::c_int as libc::c_uint
            {
                (*cinfo).input_components = 4 as libc::c_int
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_BAD_IN_COLORSPACE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            row_width = (biWidth * 3 as libc::c_int) as crate::jmorecfg_h::JDIMENSION
        }
        32 => {
            if (*cinfo).in_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_UNKNOWN as libc::c_int as libc::c_uint
            {
                (*cinfo).in_color_space = crate::jpeglib_h::JCS_EXT_BGRA
            }
            if (*cinfo).in_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_RGB as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    >= crate::jpeglib_h::JCS_EXT_RGB as libc::c_int as libc::c_uint
                    && (*cinfo).in_color_space as libc::c_uint
                        <= crate::jpeglib_h::JCS_EXT_ARGB as libc::c_int as libc::c_uint
            {
                (*cinfo).input_components = rgb_pixelsize[(*cinfo).in_color_space as usize]
            } else if (*cinfo).in_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_CMYK as libc::c_int as libc::c_uint
            {
                (*cinfo).input_components = 4 as libc::c_int
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_BAD_IN_COLORSPACE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            row_width = (biWidth * 4 as libc::c_int) as crate::jmorecfg_h::JDIMENSION
        }
        _ => {
            (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BMP_BADDEPTH as libc::c_int;
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
    while row_width & 3 as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint {
        row_width = row_width.wrapping_add(1)
    }
    (*source).row_width = row_width;
    if (*source).use_inversion_array != 0 {
        /* Allocate space for inversion array, prepare for preload pass */
        (*source).whole_image = Some(
            (*(*cinfo).mem)
                .request_virt_sarray
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            crate::jmorecfg_h::FALSE,
            row_width,
            biHeight as crate::jmorecfg_h::JDIMENSION,
            1 as libc::c_int as crate::jmorecfg_h::JDIMENSION,
        );
        (*source).pub_0.get_pixel_rows = Some(
            preload_image
                as unsafe extern "C" fn(
                    _: crate::jpeglib_h::j_compress_ptr,
                    _: crate::cdjpeg_h::cjpeg_source_ptr,
                ) -> crate::jmorecfg_h::JDIMENSION,
        );
        if !(*cinfo).progress.is_null() {
            let mut progress: crate::cdjpeg_h::cd_progress_ptr =
                (*cinfo).progress as crate::cdjpeg_h::cd_progress_ptr;
            (*progress).total_extra_passes += 1
            /* count file input as separate pass */
        }
    } else {
        (*source).iobuffer = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as crate::jpeglib_h::j_common_ptr,
            crate::jpeglib_h::JPOOL_IMAGE,
            row_width as crate::stddef_h::size_t,
        ) as *mut U_CHAR;
        match (*source).bits_per_pixel {
            8 => {
                (*source).pub_0.get_pixel_rows = Some(
                    get_8bit_row
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: crate::cdjpeg_h::cjpeg_source_ptr,
                        )
                            -> crate::jmorecfg_h::JDIMENSION,
                )
            }
            24 => {
                (*source).pub_0.get_pixel_rows = Some(
                    get_24bit_row
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: crate::cdjpeg_h::cjpeg_source_ptr,
                        )
                            -> crate::jmorecfg_h::JDIMENSION,
                )
            }
            32 => {
                (*source).pub_0.get_pixel_rows = Some(
                    get_32bit_row
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: crate::cdjpeg_h::cjpeg_source_ptr,
                        )
                            -> crate::jmorecfg_h::JDIMENSION,
                )
            }
            _ => {
                (*(*cinfo).err).msg_code = crate::cderror_h::JERR_BMP_BADDEPTH as libc::c_int;
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
    }
    /* Ensure that biWidth * cinfo->input_components doesn't exceed the maximum
    value of the JDIMENSION type.  This is only a danger with BMP files, since
    their width and height fields are 32-bit integers. */
    if (biWidth as libc::c_ulonglong).wrapping_mul((*cinfo).input_components as libc::c_ulonglong)
        > 0xffffffff as libc::c_ulonglong
    {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_WIDTH_OVERFLOW as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo as crate::jpeglib_h::j_common_ptr);
    }
    /* Allocate one-row buffer for returned data */
    (*source).pub_0.buffer = Some(
        (*(*cinfo).mem)
            .alloc_sarray
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        (biWidth * (*cinfo).input_components) as crate::jmorecfg_h::JDIMENSION,
        1 as libc::c_int as crate::jmorecfg_h::JDIMENSION,
    );
    (*source).pub_0.buffer_height = 1 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
    (*cinfo).data_precision = 8 as libc::c_int;
    (*cinfo).image_width = biWidth as crate::jmorecfg_h::JDIMENSION;
    (*cinfo).image_height = biHeight as crate::jmorecfg_h::JDIMENSION;
}
/*
 * Finish up at the end of the file.
 */

unsafe extern "C" fn finish_input_bmp(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut sinfo: crate::cdjpeg_h::cjpeg_source_ptr,
) {
    /* no work */
}
/*
 * The module selection routine for BMP format input.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_read_bmp(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut use_inversion_array: crate::jmorecfg_h::boolean,
) -> crate::cdjpeg_h::cjpeg_source_ptr {
    let mut source: bmp_source_ptr = 0 as *mut _bmp_source_struct;
    /* Create module interface object */
    source = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<bmp_source_struct>() as libc::c_ulong,
    ) as bmp_source_ptr; /* make back link for subroutines */
    (*source).cinfo = cinfo;
    /* Fill in method ptrs, except get_pixel_rows which start_input sets */
    (*source).pub_0.start_input = Some(
        start_input_bmp
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_compress_ptr,
                _: crate::cdjpeg_h::cjpeg_source_ptr,
            ) -> (),
    );
    (*source).pub_0.finish_input = Some(
        finish_input_bmp
            as unsafe extern "C" fn(
                _: crate::jpeglib_h::j_compress_ptr,
                _: crate::cdjpeg_h::cjpeg_source_ptr,
            ) -> (),
    );
    (*source).use_inversion_array = use_inversion_array;
    return source as crate::cdjpeg_h::cjpeg_source_ptr;
}
/* BMP_SUPPORTED */
