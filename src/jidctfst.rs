use libc;
use libc::c_int;
use libc::c_long;

pub use crate::jdct_h::DCTELEM;
pub use crate::jdct_h::IFAST_MULT_TYPE;
pub use crate::jdct_h::RANGE_MASK;
pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::CENTERJSAMPLE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::MAXJSAMPLE;
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
pub use crate::jpeglib_h::C2RustUnnamed_3;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::DCTSIZE;
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
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::stddef_h::size_t;
/*
 * jidctfst.c
 *
 * This file was part of the Independent JPEG Group's software:
 * Copyright (C) 1994-1998, Thomas G. Lane.
 * libjpeg-turbo Modifications:
 * Copyright (C) 2015, D. R. Commander.
 * For conditions of distribution and use, see the accompanying README.ijg
 * file.
 *
 * This file contains a fast, not so accurate integer implementation of the
 * inverse DCT (Discrete Cosine Transform).  In the IJG code, this routine
 * must also perform dequantization of the input coefficients.
 *
 * A 2-D IDCT can be done by 1-D IDCT on each column followed by 1-D IDCT
 * on each row (or vice versa, but it's more convenient to emit a row at
 * a time).  Direct algorithms are also available, but they are much more
 * complex and seem not to be any faster when reduced to code.
 *
 * This implementation is based on Arai, Agui, and Nakajima's algorithm for
 * scaled DCT.  Their original paper (Trans. IEICE E-71(11):1095) is in
 * Japanese, but the algorithm is described in the Pennebaker & Mitchell
 * JPEG textbook (see REFERENCES section in file README.ijg).  The following
 * code is based directly on figure 4-8 in P&M.
 * While an 8-point DCT cannot be done in less than 11 multiplies, it is
 * possible to arrange the computation so that many of the multiplies are
 * simple scalings of the final outputs.  These multiplies can then be
 * folded into the multiplications or divisions by the JPEG quantization
 * table entries.  The AA&N method leaves only 5 multiplies and 29 adds
 * to be done in the DCT itself.
 * The primary disadvantage of this method is that with fixed-point math,
 * accuracy is lost due to imprecise representation of the scaled
 * quantization values.  The smaller the quantization table entry, the less
 * precise the scaled value, so this implementation does worse with high-
 * quality-setting files than with low-quality ones.
 */
/*
 * This module is specialized to the case DCTSIZE = 8.
 */
/* Scaling decisions are generally the same as in the LL&M algorithm;
 * see jidctint.c for more details.  However, we choose to descale
 * (right shift) multiplication products as soon as they are formed,
 * rather than carrying additional fractional bits into subsequent additions.
 * This compromises accuracy slightly, but it lets us save a few shifts.
 * More importantly, 16-bit arithmetic is then adequate (for 8-bit samples)
 * everywhere except in the multiplications proper; this saves a good deal
 * of work on 16-bit-int machines.
 *
 * The dequantized coefficients are not integers because the AA&N scaling
 * factors have been incorporated.  We represent them scaled up by PASS1_BITS,
 * so that the first and second IDCT rounds have the same input scaling.
 * For 8-bit JSAMPLEs, we choose IFAST_SCALE_BITS = PASS1_BITS so as to
 * avoid a descaling shift; this compromises accuracy rather drastically
 * for small quantization table entries, but it saves a lot of shifts.
 * For 12-bit JSAMPLEs, there's no hope of using 16x16 multiplies anyway,
 * so we use a much larger scaling factor to preserve accuracy.
 *
 * A final compromise is to represent the multiplicative constants to only
 * 8 fractional bits, rather than 13.  This saves some shifting work on some
 * machines, and may also reduce the cost of multiplication (since there
 * are fewer one-bits in the constants).
 */
/* Some C compilers fail to reduce "FIX(constant)" at compile time, thus
 * causing a lot of useless floating-point operations at run time.
 * To get around this we use the following pre-calculated constants.
 * If you change CONST_BITS you may want to add appropriate values.
 * (With a reasonable C compiler, you can just rely on the FIX() macro...)
 */
/* FIX(1.082392200) */
/* FIX(1.414213562) */
/* FIX(1.847759065) */
/* FIX(2.613125930) */
/* We can gain a little more speed, with a further compromise in accuracy,
 * by omitting the addition in a descaling shift.  This yields an incorrectly
 * rounded result half the time...
 */
/* Multiply a DCTELEM variable by an JLONG constant, and immediately
 * descale to yield a DCTELEM result.
 */
/* Dequantize a coefficient by multiplying it by the multiplier-table
 * entry; produce a DCTELEM result.  For 8-bit data a 16x16->16
 * multiplication will do.  For 12-bit data, the multiplier table is
 * declared JLONG, so a 32-bit multiply will be used.
 */
/* Like DESCALE, but applies to a DCTELEM and produces an int.
 * We assume that int right shift is unsigned if JLONG right shift is.
 */
/*
 * Perform dequantization and inverse DCT on one block of coefficients.
 */
#[no_mangle]
pub unsafe extern "C" fn jpeg_idct_ifast(
    mut cinfo: j_decompress_ptr,
    mut compptr: *mut jpeg_component_info,
    mut coef_block: JCOEFPTR,
    mut output_buf: JSAMPARRAY,
    mut output_col: JDIMENSION,
) {
    let mut tmp0: DCTELEM = 0;
    let mut tmp1: DCTELEM = 0;
    let mut tmp2: DCTELEM = 0;
    let mut tmp3: DCTELEM = 0;
    let mut tmp4: DCTELEM = 0;
    let mut tmp5: DCTELEM = 0;
    let mut tmp6: DCTELEM = 0;
    let mut tmp7: DCTELEM = 0;
    let mut tmp10: DCTELEM = 0;
    let mut tmp11: DCTELEM = 0;
    let mut tmp12: DCTELEM = 0;
    let mut tmp13: DCTELEM = 0;
    let mut z5: DCTELEM = 0;
    let mut z10: DCTELEM = 0;
    let mut z11: DCTELEM = 0;
    let mut z12: DCTELEM = 0;
    let mut z13: DCTELEM = 0;
    let mut inptr: JCOEFPTR = 0 as *mut JCOEF;
    let mut quantptr: *mut IFAST_MULT_TYPE = 0 as *mut IFAST_MULT_TYPE;
    let mut wsptr: *mut c_int = 0 as *mut c_int;
    let mut outptr: JSAMPROW = 0 as *mut JSAMPLE;
    let mut range_limit: *mut JSAMPLE = (*cinfo).sample_range_limit.offset(CENTERJSAMPLE as isize);
    let mut ctr: c_int = 0;
    /* buffers data between passes */
    let mut workspace: [c_int; 64] = [0; 64];
    inptr = coef_block;
    quantptr = (*compptr).dct_table as *mut IFAST_MULT_TYPE;
    wsptr = workspace.as_mut_ptr();
    ctr = DCTSIZE;
    while ctr > 0i32 {
        /* Due to quantization, we will usually find that many of the input
         * coefficients are zero, especially the AC terms.  We can exploit this
         * by short-circuiting the IDCT calculation for any column in which all
         * the AC terms are zero.  In that case each output is equal to the
         * DC coefficient (with scale factor as needed).
         * With typical images and quantization tables, half or more of the
         * column DCT calculations can be simplified this way.
         */
        if *inptr.offset((DCTSIZE * 1i32) as isize) as c_int == 0i32
            && *inptr.offset((DCTSIZE * 2i32) as isize) as c_int == 0i32
            && *inptr.offset((DCTSIZE * 3i32) as isize) as c_int == 0i32
            && *inptr.offset((DCTSIZE * 4i32) as isize) as c_int == 0i32
            && *inptr.offset((DCTSIZE * 5i32) as isize) as c_int == 0i32
            && *inptr.offset((DCTSIZE * 6i32) as isize) as c_int == 0i32
            && *inptr.offset((DCTSIZE * 7i32) as isize) as c_int == 0i32
        {
            /* AC terms all zero */
            let mut dcval: c_int = *inptr.offset((8i32 * 0i32) as isize) as c_int
                * *quantptr.offset((8i32 * 0i32) as isize) as c_int;
            *wsptr.offset((DCTSIZE * 0i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 1i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 2i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 3i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 4i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 5i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 6i32) as isize) = dcval;
            *wsptr.offset((DCTSIZE * 7i32) as isize) = dcval;
            inptr = inptr.offset(1isize);
            quantptr = quantptr.offset(1isize);
            wsptr = wsptr.offset(1isize)
        } else {
            tmp0 = (*inptr.offset((8i32 * 0i32) as isize) as c_int
                * *quantptr.offset((8i32 * 0i32) as isize) as c_int) as DCTELEM;
            tmp1 = (*inptr.offset((8i32 * 2i32) as isize) as c_int
                * *quantptr.offset((8i32 * 2i32) as isize) as c_int) as DCTELEM;
            tmp2 = (*inptr.offset((8i32 * 4i32) as isize) as c_int
                * *quantptr.offset((8i32 * 4i32) as isize) as c_int) as DCTELEM;
            tmp3 = (*inptr.offset((8i32 * 6i32) as isize) as c_int
                * *quantptr.offset((8i32 * 6i32) as isize) as c_int) as DCTELEM;
            tmp10 = (tmp0 as c_int + tmp2 as c_int) as DCTELEM;
            tmp11 = (tmp0 as c_int - tmp2 as c_int) as DCTELEM;
            tmp13 = (tmp1 as c_int + tmp3 as c_int) as DCTELEM;
            tmp12 = (((tmp1 as c_int - tmp3 as c_int) as c_long * 362i32 as JLONG >> 8i32)
                as DCTELEM as c_int
                - tmp13 as c_int) as DCTELEM;
            tmp0 = (tmp10 as c_int + tmp13 as c_int) as DCTELEM;
            tmp3 = (tmp10 as c_int - tmp13 as c_int) as DCTELEM;
            tmp1 = (tmp11 as c_int + tmp12 as c_int) as DCTELEM;
            tmp2 = (tmp11 as c_int - tmp12 as c_int) as DCTELEM;
            tmp4 = (*inptr.offset((8i32 * 1i32) as isize) as c_int
                * *quantptr.offset((8i32 * 1i32) as isize) as c_int) as DCTELEM;
            tmp5 = (*inptr.offset((8i32 * 3i32) as isize) as c_int
                * *quantptr.offset((8i32 * 3i32) as isize) as c_int) as DCTELEM;
            tmp6 = (*inptr.offset((8i32 * 5i32) as isize) as c_int
                * *quantptr.offset((8i32 * 5i32) as isize) as c_int) as DCTELEM;
            tmp7 = (*inptr.offset((8i32 * 7i32) as isize) as c_int
                * *quantptr.offset((8i32 * 7i32) as isize) as c_int) as DCTELEM;
            z13 = (tmp6 as c_int + tmp5 as c_int) as DCTELEM;
            z10 = (tmp6 as c_int - tmp5 as c_int) as DCTELEM;
            z11 = (tmp4 as c_int + tmp7 as c_int) as DCTELEM;
            z12 = (tmp4 as c_int - tmp7 as c_int) as DCTELEM;
            tmp7 = (z11 as c_int + z13 as c_int) as DCTELEM;
            tmp11 = ((z11 as c_int - z13 as c_int) as c_long * 362i32 as JLONG >> 8i32) as DCTELEM;
            z5 = ((z10 as c_int + z12 as c_int) as c_long * 473i32 as JLONG >> 8i32) as DCTELEM;
            tmp10 = ((z12 as c_long * 277i32 as JLONG >> 8i32) as DCTELEM as c_int - z5 as c_int)
                as DCTELEM;
            tmp12 = ((z10 as c_long * -(669i32 as JLONG) >> 8i32) as DCTELEM as c_int + z5 as c_int)
                as DCTELEM;
            tmp6 = (tmp12 as c_int - tmp7 as c_int) as DCTELEM;
            tmp5 = (tmp11 as c_int - tmp6 as c_int) as DCTELEM;
            tmp4 = (tmp10 as c_int + tmp5 as c_int) as DCTELEM;
            *wsptr.offset((DCTSIZE * 0i32) as isize) = tmp0 as c_int + tmp7 as c_int;
            *wsptr.offset((DCTSIZE * 7i32) as isize) = tmp0 as c_int - tmp7 as c_int;
            *wsptr.offset((DCTSIZE * 1i32) as isize) = tmp1 as c_int + tmp6 as c_int;
            *wsptr.offset((DCTSIZE * 6i32) as isize) = tmp1 as c_int - tmp6 as c_int;
            *wsptr.offset((DCTSIZE * 2i32) as isize) = tmp2 as c_int + tmp5 as c_int;
            *wsptr.offset((DCTSIZE * 5i32) as isize) = tmp2 as c_int - tmp5 as c_int;
            *wsptr.offset((DCTSIZE * 4i32) as isize) = tmp3 as c_int + tmp4 as c_int;
            *wsptr.offset((DCTSIZE * 3i32) as isize) = tmp3 as c_int - tmp4 as c_int;
            inptr = inptr.offset(1isize);
            quantptr = quantptr.offset(1isize);
            wsptr = wsptr.offset(1isize)
        }
        ctr -= 1
    }
    wsptr = workspace.as_mut_ptr();
    ctr = 0i32;
    while ctr < DCTSIZE {
        outptr = (*output_buf.offset(ctr as isize)).offset(output_col as isize);
        /* Rows of zeroes can be exploited in the same way as we did with columns.
         * However, the column calculation has created many nonzero AC terms, so
         * the simplification applies less often (typically 5% to 10% of the time).
         * On machines with very fast multiplication, it's possible that the
         * test takes more time than it's worth.  In that case this section
         * may be commented out.
         */
        if *wsptr.offset(1isize) == 0i32
            && *wsptr.offset(2isize) == 0i32
            && *wsptr.offset(3isize) == 0i32
            && *wsptr.offset(4isize) == 0i32
            && *wsptr.offset(5isize) == 0i32
            && *wsptr.offset(6isize) == 0i32
            && *wsptr.offset(7isize) == 0i32
        {
            /* AC terms all zero */
            let mut dcval_0: JSAMPLE =
                *range_limit.offset((*wsptr.offset(0isize) >> 2i32 + 3i32 & RANGE_MASK) as isize);
            *outptr.offset(0isize) = dcval_0;
            *outptr.offset(1isize) = dcval_0;
            *outptr.offset(2isize) = dcval_0;
            *outptr.offset(3isize) = dcval_0;
            *outptr.offset(4isize) = dcval_0;
            *outptr.offset(5isize) = dcval_0;
            *outptr.offset(6isize) = dcval_0;
            *outptr.offset(7isize) = dcval_0;
            wsptr = wsptr.offset(DCTSIZE as isize)
        } else {
            tmp10 = (*wsptr.offset(0isize) as DCTELEM as c_int
                + *wsptr.offset(4isize) as DCTELEM as c_int) as DCTELEM;
            tmp11 = (*wsptr.offset(0isize) as DCTELEM as c_int
                - *wsptr.offset(4isize) as DCTELEM as c_int) as DCTELEM;
            tmp13 = (*wsptr.offset(2isize) as DCTELEM as c_int
                + *wsptr.offset(6isize) as DCTELEM as c_int) as DCTELEM;
            tmp12 = (((*wsptr.offset(2isize) as DCTELEM as c_int
                - *wsptr.offset(6isize) as DCTELEM as c_int) as c_long
                * 362i32 as JLONG
                >> 8i32) as DCTELEM as c_int
                - tmp13 as c_int) as DCTELEM;
            tmp0 = (tmp10 as c_int + tmp13 as c_int) as DCTELEM;
            tmp3 = (tmp10 as c_int - tmp13 as c_int) as DCTELEM;
            tmp1 = (tmp11 as c_int + tmp12 as c_int) as DCTELEM;
            tmp2 = (tmp11 as c_int - tmp12 as c_int) as DCTELEM;
            z13 = (*wsptr.offset(5isize) as DCTELEM as c_int
                + *wsptr.offset(3isize) as DCTELEM as c_int) as DCTELEM;
            z10 = (*wsptr.offset(5isize) as DCTELEM as c_int
                - *wsptr.offset(3isize) as DCTELEM as c_int) as DCTELEM;
            z11 = (*wsptr.offset(1isize) as DCTELEM as c_int
                + *wsptr.offset(7isize) as DCTELEM as c_int) as DCTELEM;
            z12 = (*wsptr.offset(1isize) as DCTELEM as c_int
                - *wsptr.offset(7isize) as DCTELEM as c_int) as DCTELEM;
            tmp7 = (z11 as c_int + z13 as c_int) as DCTELEM;
            tmp11 = ((z11 as c_int - z13 as c_int) as c_long * 362i32 as JLONG >> 8i32) as DCTELEM;
            z5 = ((z10 as c_int + z12 as c_int) as c_long * 473i32 as JLONG >> 8i32) as DCTELEM;
            tmp10 = ((z12 as c_long * 277i32 as JLONG >> 8i32) as DCTELEM as c_int - z5 as c_int)
                as DCTELEM;
            tmp12 = ((z10 as c_long * -(669i32 as JLONG) >> 8i32) as DCTELEM as c_int + z5 as c_int)
                as DCTELEM;
            tmp6 = (tmp12 as c_int - tmp7 as c_int) as DCTELEM;
            tmp5 = (tmp11 as c_int - tmp6 as c_int) as DCTELEM;
            tmp4 = (tmp10 as c_int + tmp5 as c_int) as DCTELEM;
            *outptr.offset(0isize) = *range_limit
                .offset((tmp0 as c_int + tmp7 as c_int >> 2i32 + 3i32 & RANGE_MASK) as isize);
            *outptr.offset(7isize) = *range_limit
                .offset((tmp0 as c_int - tmp7 as c_int >> 2i32 + 3i32 & RANGE_MASK) as isize);
            *outptr.offset(1isize) = *range_limit
                .offset((tmp1 as c_int + tmp6 as c_int >> 2i32 + 3i32 & RANGE_MASK) as isize);
            *outptr.offset(6isize) = *range_limit
                .offset((tmp1 as c_int - tmp6 as c_int >> 2i32 + 3i32 & RANGE_MASK) as isize);
            *outptr.offset(2isize) = *range_limit
                .offset((tmp2 as c_int + tmp5 as c_int >> 2i32 + 3i32 & RANGE_MASK) as isize);
            *outptr.offset(5isize) = *range_limit
                .offset((tmp2 as c_int - tmp5 as c_int >> 2i32 + 3i32 & RANGE_MASK) as isize);
            *outptr.offset(4isize) = *range_limit
                .offset((tmp3 as c_int + tmp4 as c_int >> 2i32 + 3i32 & RANGE_MASK) as isize);
            *outptr.offset(3isize) = *range_limit
                .offset((tmp3 as c_int - tmp4 as c_int >> 2i32 + 3i32 & RANGE_MASK) as isize);
            wsptr = wsptr.offset(DCTSIZE as isize)
        }
        ctr += 1
    }
}
