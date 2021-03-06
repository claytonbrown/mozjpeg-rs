use ::libc;

#[c2rust::header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/jmorecfg.h:18"]
pub mod jmorecfg_h {

    pub static mut rgb_pixelsize: [libc::c_int; 17] = [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        crate::jmorecfg_h::RGB_PIXELSIZE_5,
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

#[c2rust::header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/jccolext.c:90"]
pub mod jccolext_c {
    /*
     * jccolext.c
     *
     * This file was part of the Independent JPEG Group's software:
     * Copyright (C) 1991-1996, Thomas G. Lane.
     * libjpeg-turbo Modifications:
     * Copyright (C) 2009-2012, 2015, D. R. Commander.
     * For conditions of distribution and use, see the accompanying README.ijg
     * file.
     *
     * This file contains input colorspace conversion routines.
     */
    /*
     * jccolext.c
     *
     * This file was part of the Independent JPEG Group's software:
     * Copyright (C) 1991-1996, Thomas G. Lane.
     * libjpeg-turbo Modifications:
     * Copyright (C) 2009-2012, 2015, D. R. Commander.
     * For conditions of distribution and use, see the accompanying README.ijg
     * file.
     *
     * This file contains input colorspace conversion routines.
     */
    /*
     * jccolext.c
     *
     * This file was part of the Independent JPEG Group's software:
     * Copyright (C) 1991-1996, Thomas G. Lane.
     * libjpeg-turbo Modifications:
     * Copyright (C) 2009-2012, 2015, D. R. Commander.
     * For conditions of distribution and use, see the accompanying README.ijg
     * file.
     *
     * This file contains input colorspace conversion routines.
     */
    /*
     * jccolext.c
     *
     * This file was part of the Independent JPEG Group's software:
     * Copyright (C) 1991-1996, Thomas G. Lane.
     * libjpeg-turbo Modifications:
     * Copyright (C) 2009-2012, 2015, D. R. Commander.
     * For conditions of distribution and use, see the accompanying README.ijg
     * file.
     *
     * This file contains input colorspace conversion routines.
     */
    /*
     * jccolext.c
     *
     * This file was part of the Independent JPEG Group's software:
     * Copyright (C) 1991-1996, Thomas G. Lane.
     * libjpeg-turbo Modifications:
     * Copyright (C) 2009-2012, 2015, D. R. Commander.
     * For conditions of distribution and use, see the accompanying README.ijg
     * file.
     *
     * This file contains input colorspace conversion routines.
     */
    /*
     * jccolext.c
     *
     * This file was part of the Independent JPEG Group's software:
     * Copyright (C) 1991-1996, Thomas G. Lane.
     * libjpeg-turbo Modifications:
     * Copyright (C) 2009-2012, 2015, D. R. Commander.
     * For conditions of distribution and use, see the accompanying README.ijg
     * file.
     *
     * This file contains input colorspace conversion routines.
     */
    /*
     * jccolext.c
     *
     * This file was part of the Independent JPEG Group's software:
     * Copyright (C) 1991-1996, Thomas G. Lane.
     * libjpeg-turbo Modifications:
     * Copyright (C) 2009-2012, 2015, D. R. Commander.
     * For conditions of distribution and use, see the accompanying README.ijg
     * file.
     *
     * This file contains input colorspace conversion routines.
     */
    /* This file is included by jccolor.c */
    /* This file is included by jccolor.c */
    /* This file is included by jccolor.c */
    /* This file is included by jccolor.c */
    /* This file is included by jccolor.c */
    /* This file is included by jccolor.c */
    /* This file is included by jccolor.c */
    /*
     * Convert some rows of samples to the JPEG colorspace.
     *
     * Note that we change from the application's interleaved-pixel format
     * to our internal noninterleaved, one-plane-per-component format.
     * The input buffer is therefore three times as wide as the output buffer.
     *
     * A starting row offset is provided only for the output buffer.  The caller
     * can easily adjust the passed input_buf value to accommodate any row
     * offset required on that side.
     */
    /*
     * Convert some rows of samples to the JPEG colorspace.
     *
     * Note that we change from the application's interleaved-pixel format
     * to our internal noninterleaved, one-plane-per-component format.
     * The input buffer is therefore three times as wide as the output buffer.
     *
     * A starting row offset is provided only for the output buffer.  The caller
     * can easily adjust the passed input_buf value to accommodate any row
     * offset required on that side.
     */
    /*
     * Convert some rows of samples to the JPEG colorspace.
     *
     * Note that we change from the application's interleaved-pixel format
     * to our internal noninterleaved, one-plane-per-component format.
     * The input buffer is therefore three times as wide as the output buffer.
     *
     * A starting row offset is provided only for the output buffer.  The caller
     * can easily adjust the passed input_buf value to accommodate any row
     * offset required on that side.
     */
    /*
     * Convert some rows of samples to the JPEG colorspace.
     *
     * Note that we change from the application's interleaved-pixel format
     * to our internal noninterleaved, one-plane-per-component format.
     * The input buffer is therefore three times as wide as the output buffer.
     *
     * A starting row offset is provided only for the output buffer.  The caller
     * can easily adjust the passed input_buf value to accommodate any row
     * offset required on that side.
     */
    /*
     * Convert some rows of samples to the JPEG colorspace.
     *
     * Note that we change from the application's interleaved-pixel format
     * to our internal noninterleaved, one-plane-per-component format.
     * The input buffer is therefore three times as wide as the output buffer.
     *
     * A starting row offset is provided only for the output buffer.  The caller
     * can easily adjust the passed input_buf value to accommodate any row
     * offset required on that side.
     */
    /*
     * Convert some rows of samples to the JPEG colorspace.
     *
     * Note that we change from the application's interleaved-pixel format
     * to our internal noninterleaved, one-plane-per-component format.
     * The input buffer is therefore three times as wide as the output buffer.
     *
     * A starting row offset is provided only for the output buffer.  The caller
     * can easily adjust the passed input_buf value to accommodate any row
     * offset required on that side.
     */
    /*
     * Convert some rows of samples to the JPEG colorspace.
     *
     * Note that we change from the application's interleaved-pixel format
     * to our internal noninterleaved, one-plane-per-component format.
     * The input buffer is therefore three times as wide as the output buffer.
     *
     * A starting row offset is provided only for the output buffer.  The caller
     * can easily adjust the passed input_buf value to accommodate any row
     * offset required on that side.
     */
    #[inline(always)]

    pub unsafe extern "C" fn extbgr_ycc_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
        let mut r: libc::c_int = 0;
        let mut g: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh0 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh0;
            outptr0 = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            outptr1 = *(*output_buf.offset(1 as libc::c_int as isize)).offset(output_row as isize);
            outptr2 = *(*output_buf.offset(2 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                r = *inptr.offset(2 as libc::c_int as isize) as libc::c_int;
                g = *inptr.offset(1 as libc::c_int as isize) as libc::c_int;
                b = *inptr.offset(0 as libc::c_int as isize) as libc::c_int;
                inptr = inptr.offset(RGB_PIXELSIZE_3 as isize);
                /* If the inputs are 0..MAXJSAMPLE, the outputs of these equations
                 * must be too; we do not need an explicit range-limiting operation.
                 * Hence the value being shifted is never negative, and we don't
                 * need the general RIGHT_SHIFT macro.
                 */
                /* Y */
                /* If the inputs are 0..MAXJSAMPLE, the outputs of these equations
                 * must be too; we do not need an explicit range-limiting operation.
                 * Hence the value being shifted is never negative, and we don't
                 * need the general RIGHT_SHIFT macro.
                 */
                /* Y */
                /* If the inputs are 0..MAXJSAMPLE, the outputs of these equations
                 * must be too; we do not need an explicit range-limiting operation.
                 * Hence the value being shifted is never negative, and we don't
                 * need the general RIGHT_SHIFT macro.
                 */
                /* Y */
                /* If the inputs are 0..MAXJSAMPLE, the outputs of these equations
                 * must be too; we do not need an explicit range-limiting operation.
                 * Hence the value being shifted is never negative, and we don't
                 * need the general RIGHT_SHIFT macro.
                 */
                /* Y */
                /* If the inputs are 0..MAXJSAMPLE, the outputs of these equations
                 * must be too; we do not need an explicit range-limiting operation.
                 * Hence the value being shifted is never negative, and we don't
                 * need the general RIGHT_SHIFT macro.
                 */
                /* Y */
                /* If the inputs are 0..MAXJSAMPLE, the outputs of these equations
                 * must be too; we do not need an explicit range-limiting operation.
                 * Hence the value being shifted is never negative, and we don't
                 * need the general RIGHT_SHIFT macro.
                 */
                /* Y */
                /* If the inputs are 0..MAXJSAMPLE, the outputs of these equations
                 * must be too; we do not need an explicit range-limiting operation.
                 * Hence the value being shifted is never negative, and we don't
                 * need the general RIGHT_SHIFT macro.
                 */
                /* Y */
                *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                    + *ctab.offset((g + G_Y_OFF) as isize)
                    + *ctab.offset((b + B_Y_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                /* Cb */
                /* Cb */
                /* Cb */
                /* Cb */
                /* Cb */
                /* Cb */
                /* Cb */
                *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                    + *ctab.offset((g + G_CB_OFF) as isize)
                    + *ctab.offset((b + B_CB_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                /* Cr */
                /* Cr */
                /* Cr */
                /* Cr */
                /* Cr */
                /* Cr */
                /* Cr */
                *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                    + *ctab.offset((g + G_CR_OFF) as isize)
                    + *ctab.offset((b + B_CR_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                col = col.wrapping_add(1)
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn rgb_ycc_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
        let mut r: libc::c_int = 0;
        let mut g: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh1 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh1;
            outptr0 = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            outptr1 = *(*output_buf.offset(1 as libc::c_int as isize)).offset(output_row as isize);
            outptr2 = *(*output_buf.offset(2 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                r = *inptr.offset(0 as libc::c_int as isize) as libc::c_int;
                g = *inptr.offset(1 as libc::c_int as isize) as libc::c_int;
                b = *inptr.offset(2 as libc::c_int as isize) as libc::c_int;
                inptr = inptr.offset(crate::jmorecfg_h::RGB_PIXELSIZE_5 as isize);
                *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                    + *ctab.offset((g + G_Y_OFF) as isize)
                    + *ctab.offset((b + B_Y_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                    + *ctab.offset((g + G_CB_OFF) as isize)
                    + *ctab.offset((b + B_CB_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                    + *ctab.offset((g + G_CR_OFF) as isize)
                    + *ctab.offset((b + B_CR_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                col = col.wrapping_add(1)
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn extrgb_ycc_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
        let mut r: libc::c_int = 0;
        let mut g: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh2 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh2;
            outptr0 = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            outptr1 = *(*output_buf.offset(1 as libc::c_int as isize)).offset(output_row as isize);
            outptr2 = *(*output_buf.offset(2 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                r = *inptr.offset(0 as libc::c_int as isize) as libc::c_int;
                g = *inptr.offset(1 as libc::c_int as isize) as libc::c_int;
                b = *inptr.offset(2 as libc::c_int as isize) as libc::c_int;
                inptr = inptr.offset(RGB_PIXELSIZE_4 as isize);
                *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                    + *ctab.offset((g + G_Y_OFF) as isize)
                    + *ctab.offset((b + B_Y_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                    + *ctab.offset((g + G_CB_OFF) as isize)
                    + *ctab.offset((b + B_CB_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                    + *ctab.offset((g + G_CR_OFF) as isize)
                    + *ctab.offset((b + B_CR_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                col = col.wrapping_add(1)
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn extxbgr_ycc_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
        let mut r: libc::c_int = 0;
        let mut g: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh3 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh3;
            outptr0 = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            outptr1 = *(*output_buf.offset(1 as libc::c_int as isize)).offset(output_row as isize);
            outptr2 = *(*output_buf.offset(2 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                r = *inptr.offset(3 as libc::c_int as isize) as libc::c_int;
                g = *inptr.offset(2 as libc::c_int as isize) as libc::c_int;
                b = *inptr.offset(1 as libc::c_int as isize) as libc::c_int;
                inptr = inptr.offset(RGB_PIXELSIZE_0 as isize);
                *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                    + *ctab.offset((g + G_Y_OFF) as isize)
                    + *ctab.offset((b + B_Y_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                    + *ctab.offset((g + G_CB_OFF) as isize)
                    + *ctab.offset((b + B_CB_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                    + *ctab.offset((g + G_CR_OFF) as isize)
                    + *ctab.offset((b + B_CR_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                col = col.wrapping_add(1)
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn extbgrx_ycc_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
        let mut r: libc::c_int = 0;
        let mut g: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh4 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh4;
            outptr0 = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            outptr1 = *(*output_buf.offset(1 as libc::c_int as isize)).offset(output_row as isize);
            outptr2 = *(*output_buf.offset(2 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                r = *inptr.offset(2 as libc::c_int as isize) as libc::c_int;
                g = *inptr.offset(1 as libc::c_int as isize) as libc::c_int;
                b = *inptr.offset(0 as libc::c_int as isize) as libc::c_int;
                inptr = inptr.offset(RGB_PIXELSIZE_1 as isize);
                *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                    + *ctab.offset((g + G_Y_OFF) as isize)
                    + *ctab.offset((b + B_Y_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                    + *ctab.offset((g + G_CB_OFF) as isize)
                    + *ctab.offset((b + B_CB_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                    + *ctab.offset((g + G_CR_OFF) as isize)
                    + *ctab.offset((b + B_CR_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                col = col.wrapping_add(1)
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn extrgbx_ycc_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
        let mut r: libc::c_int = 0;
        let mut g: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh5 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh5;
            outptr0 = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            outptr1 = *(*output_buf.offset(1 as libc::c_int as isize)).offset(output_row as isize);
            outptr2 = *(*output_buf.offset(2 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                r = *inptr.offset(0 as libc::c_int as isize) as libc::c_int;
                g = *inptr.offset(1 as libc::c_int as isize) as libc::c_int;
                b = *inptr.offset(2 as libc::c_int as isize) as libc::c_int;
                inptr = inptr.offset(RGB_PIXELSIZE_2 as isize);
                *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                    + *ctab.offset((g + G_Y_OFF) as isize)
                    + *ctab.offset((b + B_Y_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                    + *ctab.offset((g + G_CB_OFF) as isize)
                    + *ctab.offset((b + B_CB_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                    + *ctab.offset((g + G_CR_OFF) as isize)
                    + *ctab.offset((b + B_CR_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                col = col.wrapping_add(1)
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn extxrgb_ycc_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
        let mut r: libc::c_int = 0;
        let mut g: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh6 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh6;
            outptr0 = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            outptr1 = *(*output_buf.offset(1 as libc::c_int as isize)).offset(output_row as isize);
            outptr2 = *(*output_buf.offset(2 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                r = *inptr.offset(1 as libc::c_int as isize) as libc::c_int;
                g = *inptr.offset(2 as libc::c_int as isize) as libc::c_int;
                b = *inptr.offset(3 as libc::c_int as isize) as libc::c_int;
                inptr = inptr.offset(RGB_PIXELSIZE as isize);
                *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                    + *ctab.offset((g + G_Y_OFF) as isize)
                    + *ctab.offset((b + B_Y_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                    + *ctab.offset((g + G_CB_OFF) as isize)
                    + *ctab.offset((b + B_CB_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                    + *ctab.offset((g + G_CR_OFF) as isize)
                    + *ctab.offset((b + B_CR_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                col = col.wrapping_add(1)
            }
        }
    }
    /* *************** Cases other than RGB -> YCbCr **************/
    /* *************** Cases other than RGB -> YCbCr **************/
    /* *************** Cases other than RGB -> YCbCr **************/
    /* *************** Cases other than RGB -> YCbCr **************/
    /* *************** Cases other than RGB -> YCbCr **************/
    /* *************** Cases other than RGB -> YCbCr **************/
    /* *************** Cases other than RGB -> YCbCr **************/
    /*
     * Convert some rows of samples to the JPEG colorspace.
     * This version handles RGB->grayscale conversion, which is the same
     * as the RGB->Y portion of RGB->YCbCr.
     * We assume rgb_ycc_start has been called (we only use the Y tables).
     */
    /*
     * Convert some rows of samples to the JPEG colorspace.
     * This version handles RGB->grayscale conversion, which is the same
     * as the RGB->Y portion of RGB->YCbCr.
     * We assume rgb_ycc_start has been called (we only use the Y tables).
     */
    /*
     * Convert some rows of samples to the JPEG colorspace.
     * This version handles RGB->grayscale conversion, which is the same
     * as the RGB->Y portion of RGB->YCbCr.
     * We assume rgb_ycc_start has been called (we only use the Y tables).
     */
    /*
     * Convert some rows of samples to the JPEG colorspace.
     * This version handles RGB->grayscale conversion, which is the same
     * as the RGB->Y portion of RGB->YCbCr.
     * We assume rgb_ycc_start has been called (we only use the Y tables).
     */
    /*
     * Convert some rows of samples to the JPEG colorspace.
     * This version handles RGB->grayscale conversion, which is the same
     * as the RGB->Y portion of RGB->YCbCr.
     * We assume rgb_ycc_start has been called (we only use the Y tables).
     */
    /*
     * Convert some rows of samples to the JPEG colorspace.
     * This version handles RGB->grayscale conversion, which is the same
     * as the RGB->Y portion of RGB->YCbCr.
     * We assume rgb_ycc_start has been called (we only use the Y tables).
     */
    /*
     * Convert some rows of samples to the JPEG colorspace.
     * This version handles RGB->grayscale conversion, which is the same
     * as the RGB->Y portion of RGB->YCbCr.
     * We assume rgb_ycc_start has been called (we only use the Y tables).
     */
    #[inline(always)]

    pub unsafe extern "C" fn rgb_gray_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
        let mut r: libc::c_int = 0;
        let mut g: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh7 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh7;
            outptr = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                r = *inptr.offset(0 as libc::c_int as isize) as libc::c_int;
                g = *inptr.offset(1 as libc::c_int as isize) as libc::c_int;
                b = *inptr.offset(2 as libc::c_int as isize) as libc::c_int;
                inptr = inptr.offset(crate::jmorecfg_h::RGB_PIXELSIZE_5 as isize);
                /* Y */
                /* Y */
                /* Y */
                /* Y */
                /* Y */
                /* Y */
                /* Y */
                *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                    + *ctab.offset((g + G_Y_OFF) as isize)
                    + *ctab.offset((b + B_Y_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                col = col.wrapping_add(1)
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn extxrgb_gray_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
        let mut r: libc::c_int = 0;
        let mut g: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh8 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh8;
            outptr = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                r = *inptr.offset(1 as libc::c_int as isize) as libc::c_int;
                g = *inptr.offset(2 as libc::c_int as isize) as libc::c_int;
                b = *inptr.offset(3 as libc::c_int as isize) as libc::c_int;
                inptr = inptr.offset(RGB_PIXELSIZE as isize);
                *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                    + *ctab.offset((g + G_Y_OFF) as isize)
                    + *ctab.offset((b + B_Y_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                col = col.wrapping_add(1)
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn extxbgr_gray_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
        let mut r: libc::c_int = 0;
        let mut g: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh9 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh9;
            outptr = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                r = *inptr.offset(3 as libc::c_int as isize) as libc::c_int;
                g = *inptr.offset(2 as libc::c_int as isize) as libc::c_int;
                b = *inptr.offset(1 as libc::c_int as isize) as libc::c_int;
                inptr = inptr.offset(RGB_PIXELSIZE_0 as isize);
                *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                    + *ctab.offset((g + G_Y_OFF) as isize)
                    + *ctab.offset((b + B_Y_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                col = col.wrapping_add(1)
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn extbgrx_gray_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
        let mut r: libc::c_int = 0;
        let mut g: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh10 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh10;
            outptr = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                r = *inptr.offset(2 as libc::c_int as isize) as libc::c_int;
                g = *inptr.offset(1 as libc::c_int as isize) as libc::c_int;
                b = *inptr.offset(0 as libc::c_int as isize) as libc::c_int;
                inptr = inptr.offset(RGB_PIXELSIZE_1 as isize);
                *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                    + *ctab.offset((g + G_Y_OFF) as isize)
                    + *ctab.offset((b + B_Y_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                col = col.wrapping_add(1)
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn extbgr_gray_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
        let mut r: libc::c_int = 0;
        let mut g: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh11 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh11;
            outptr = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                r = *inptr.offset(2 as libc::c_int as isize) as libc::c_int;
                g = *inptr.offset(1 as libc::c_int as isize) as libc::c_int;
                b = *inptr.offset(0 as libc::c_int as isize) as libc::c_int;
                inptr = inptr.offset(RGB_PIXELSIZE_3 as isize);
                *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                    + *ctab.offset((g + G_Y_OFF) as isize)
                    + *ctab.offset((b + B_Y_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                col = col.wrapping_add(1)
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn extrgbx_gray_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
        let mut r: libc::c_int = 0;
        let mut g: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh12 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh12;
            outptr = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                r = *inptr.offset(0 as libc::c_int as isize) as libc::c_int;
                g = *inptr.offset(1 as libc::c_int as isize) as libc::c_int;
                b = *inptr.offset(2 as libc::c_int as isize) as libc::c_int;
                inptr = inptr.offset(RGB_PIXELSIZE_2 as isize);
                *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                    + *ctab.offset((g + G_Y_OFF) as isize)
                    + *ctab.offset((b + B_Y_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                col = col.wrapping_add(1)
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn extrgb_gray_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
        let mut r: libc::c_int = 0;
        let mut g: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh13 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh13;
            outptr = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                r = *inptr.offset(0 as libc::c_int as isize) as libc::c_int;
                g = *inptr.offset(1 as libc::c_int as isize) as libc::c_int;
                b = *inptr.offset(2 as libc::c_int as isize) as libc::c_int;
                inptr = inptr.offset(RGB_PIXELSIZE_4 as isize);
                *outptr.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                    + *ctab.offset((g + G_Y_OFF) as isize)
                    + *ctab.offset((b + B_Y_OFF) as isize)
                    >> SCALEBITS)
                    as crate::jmorecfg_h::JSAMPLE;
                col = col.wrapping_add(1)
            }
        }
    }
    /*
     * Convert some rows of samples to the JPEG colorspace.
     * This version handles extended RGB->plain RGB conversion
     */
    /*
     * Convert some rows of samples to the JPEG colorspace.
     * This version handles extended RGB->plain RGB conversion
     */
    /*
     * Convert some rows of samples to the JPEG colorspace.
     * This version handles extended RGB->plain RGB conversion
     */
    /*
     * Convert some rows of samples to the JPEG colorspace.
     * This version handles extended RGB->plain RGB conversion
     */
    /*
     * Convert some rows of samples to the JPEG colorspace.
     * This version handles extended RGB->plain RGB conversion
     */
    /*
     * Convert some rows of samples to the JPEG colorspace.
     * This version handles extended RGB->plain RGB conversion
     */
    /*
     * Convert some rows of samples to the JPEG colorspace.
     * This version handles extended RGB->plain RGB conversion
     */
    #[inline(always)]

    pub unsafe extern "C" fn extxrgb_rgb_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh14 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh14;
            outptr0 = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            outptr1 = *(*output_buf.offset(1 as libc::c_int as isize)).offset(output_row as isize);
            outptr2 = *(*output_buf.offset(2 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                *outptr0.offset(col as isize) = *inptr.offset(1 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr1.offset(col as isize) = *inptr.offset(2 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr2.offset(col as isize) = *inptr.offset(3 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                inptr = inptr.offset(RGB_PIXELSIZE as isize);
                col = col.wrapping_add(1)
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn extxbgr_rgb_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh15 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh15;
            outptr0 = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            outptr1 = *(*output_buf.offset(1 as libc::c_int as isize)).offset(output_row as isize);
            outptr2 = *(*output_buf.offset(2 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                *outptr0.offset(col as isize) = *inptr.offset(3 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr1.offset(col as isize) = *inptr.offset(2 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr2.offset(col as isize) = *inptr.offset(1 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                inptr = inptr.offset(RGB_PIXELSIZE_0 as isize);
                col = col.wrapping_add(1)
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn extbgrx_rgb_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh16 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh16;
            outptr0 = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            outptr1 = *(*output_buf.offset(1 as libc::c_int as isize)).offset(output_row as isize);
            outptr2 = *(*output_buf.offset(2 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                *outptr0.offset(col as isize) = *inptr.offset(2 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr1.offset(col as isize) = *inptr.offset(1 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr2.offset(col as isize) = *inptr.offset(0 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                inptr = inptr.offset(RGB_PIXELSIZE_1 as isize);
                col = col.wrapping_add(1)
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn extrgbx_rgb_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh17 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh17;
            outptr0 = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            outptr1 = *(*output_buf.offset(1 as libc::c_int as isize)).offset(output_row as isize);
            outptr2 = *(*output_buf.offset(2 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                *outptr0.offset(col as isize) = *inptr.offset(0 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr1.offset(col as isize) = *inptr.offset(1 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr2.offset(col as isize) = *inptr.offset(2 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                inptr = inptr.offset(RGB_PIXELSIZE_2 as isize);
                col = col.wrapping_add(1)
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn extrgb_rgb_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh18 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh18;
            outptr0 = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            outptr1 = *(*output_buf.offset(1 as libc::c_int as isize)).offset(output_row as isize);
            outptr2 = *(*output_buf.offset(2 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                *outptr0.offset(col as isize) = *inptr.offset(0 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr1.offset(col as isize) = *inptr.offset(1 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr2.offset(col as isize) = *inptr.offset(2 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                inptr = inptr.offset(RGB_PIXELSIZE_4 as isize);
                col = col.wrapping_add(1)
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn extbgr_rgb_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh19 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh19;
            outptr0 = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            outptr1 = *(*output_buf.offset(1 as libc::c_int as isize)).offset(output_row as isize);
            outptr2 = *(*output_buf.offset(2 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                *outptr0.offset(col as isize) = *inptr.offset(2 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr1.offset(col as isize) = *inptr.offset(1 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr2.offset(col as isize) = *inptr.offset(0 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                inptr = inptr.offset(RGB_PIXELSIZE_3 as isize);
                col = col.wrapping_add(1)
            }
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn rgb_rgb_convert_internal(
        mut cinfo: crate::jpeglib_h::j_compress_ptr,
        mut input_buf: crate::jpeglib_h::JSAMPARRAY,
        mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
        mut output_row: crate::jmorecfg_h::JDIMENSION,
        mut num_rows: libc::c_int,
    ) {
        let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut outptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
        let mut col: crate::jmorecfg_h::JDIMENSION = 0;
        let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh20 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh20;
            outptr0 = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            outptr1 = *(*output_buf.offset(1 as libc::c_int as isize)).offset(output_row as isize);
            outptr2 = *(*output_buf.offset(2 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                *outptr0.offset(col as isize) = *inptr.offset(0 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr1.offset(col as isize) = *inptr.offset(1 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                *outptr2.offset(col as isize) = *inptr.offset(2 as libc::c_int as isize)
                    as libc::c_int
                    as crate::jmorecfg_h::JSAMPLE;
                inptr = inptr.offset(crate::jmorecfg_h::RGB_PIXELSIZE_5 as isize);
                col = col.wrapping_add(1)
            }
        }
    }

    use crate::jmorecfg_h::JDIMENSION;
    use crate::jmorecfg_h::JSAMPLE;
    use crate::jmorecfg_h::RGB_PIXELSIZE_5;
    use crate::jpegint_h::JLONG;
    use crate::jpeglib_h::JSAMPROW;
    use crate::src::jccolor::my_cconvert_ptr;
    use crate::src::jccolor::B_CB_OFF;
    use crate::src::jccolor::B_CR_OFF;
    use crate::src::jccolor::B_Y_OFF;
    use crate::src::jccolor::G_CB_OFF;
    use crate::src::jccolor::G_CR_OFF;
    use crate::src::jccolor::G_Y_OFF;
    use crate::src::jccolor::RGB_PIXELSIZE;
    use crate::src::jccolor::RGB_PIXELSIZE_0;
    use crate::src::jccolor::RGB_PIXELSIZE_1;
    use crate::src::jccolor::RGB_PIXELSIZE_2;
    use crate::src::jccolor::RGB_PIXELSIZE_3;
    use crate::src::jccolor::RGB_PIXELSIZE_4;
    use crate::src::jccolor::R_CB_OFF;
    use crate::src::jccolor::R_CR_OFF;
    use crate::src::jccolor::R_Y_OFF;
    use crate::src::jccolor::SCALEBITS;
}
pub use crate::stddef_h::size_t;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::CENTERJSAMPLE;
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
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::MAXJSAMPLE;
pub use crate::jmorecfg_h::RGB_BLUE;
pub use crate::jmorecfg_h::RGB_GREEN;
pub use crate::jmorecfg_h::RGB_PIXELSIZE_5;
pub use crate::jmorecfg_h::RGB_RED;
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
pub use crate::jpegint_h::JLONG;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
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
pub use crate::src::jccolor::jccolext_c::extbgr_gray_convert_internal;
pub use crate::src::jccolor::jccolext_c::extbgr_rgb_convert_internal;
pub use crate::src::jccolor::jccolext_c::extbgr_ycc_convert_internal;
pub use crate::src::jccolor::jccolext_c::extbgrx_gray_convert_internal;
pub use crate::src::jccolor::jccolext_c::extbgrx_rgb_convert_internal;
pub use crate::src::jccolor::jccolext_c::extbgrx_ycc_convert_internal;
pub use crate::src::jccolor::jccolext_c::extrgb_gray_convert_internal;
pub use crate::src::jccolor::jccolext_c::extrgb_rgb_convert_internal;
pub use crate::src::jccolor::jccolext_c::extrgb_ycc_convert_internal;
pub use crate::src::jccolor::jccolext_c::extrgbx_gray_convert_internal;
pub use crate::src::jccolor::jccolext_c::extrgbx_rgb_convert_internal;
pub use crate::src::jccolor::jccolext_c::extrgbx_ycc_convert_internal;
pub use crate::src::jccolor::jccolext_c::extxbgr_gray_convert_internal;
pub use crate::src::jccolor::jccolext_c::extxbgr_rgb_convert_internal;
pub use crate::src::jccolor::jccolext_c::extxbgr_ycc_convert_internal;
pub use crate::src::jccolor::jccolext_c::extxrgb_gray_convert_internal;
pub use crate::src::jccolor::jccolext_c::extxrgb_rgb_convert_internal;
pub use crate::src::jccolor::jccolext_c::extxrgb_ycc_convert_internal;
pub use crate::src::jccolor::jccolext_c::rgb_gray_convert_internal;
pub use crate::src::jccolor::jccolext_c::rgb_rgb_convert_internal;
pub use crate::src::jccolor::jccolext_c::rgb_ycc_convert_internal;
pub use crate::src::jccolor::jmorecfg_h::rgb_blue;
pub use crate::src::jccolor::jmorecfg_h::rgb_green;
pub use crate::src::jccolor::jmorecfg_h::rgb_pixelsize;
pub use crate::src::jccolor::jmorecfg_h::rgb_red;
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
use crate::src::simd::x86_64::jsimd::jsimd_can_rgb_gray;
use crate::src::simd::x86_64::jsimd::jsimd_can_rgb_ycc;
use crate::src::simd::x86_64::jsimd::jsimd_rgb_gray_convert;
use crate::src::simd::x86_64::jsimd::jsimd_rgb_ycc_convert;
pub use crate::stdlib::C2RustUnnamed_0;

pub type my_cconvert_ptr = *mut my_color_converter;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_color_converter {
    pub pub_0: crate::jpegint_h::jpeg_color_converter,
    pub rgb_ycc_tab: *mut crate::jpegint_h::JLONG,
}
/* *************** RGB -> YCbCr conversion: most common case **************/
/*
 * YCbCr is defined per CCIR 601-1, except that Cb and Cr are
 * normalized to the range 0..MAXJSAMPLE rather than -0.5 .. 0.5.
 * The conversion equations to be implemented are therefore
 *      Y  =  0.29900 * R + 0.58700 * G + 0.11400 * B
 *      Cb = -0.16874 * R - 0.33126 * G + 0.50000 * B  + CENTERJSAMPLE
 *      Cr =  0.50000 * R - 0.41869 * G - 0.08131 * B  + CENTERJSAMPLE
 * (These numbers are derived from TIFF 6.0 section 21, dated 3-June-92.)
 * Note: older versions of the IJG code used a zero offset of MAXJSAMPLE/2,
 * rather than CENTERJSAMPLE, for Cb and Cr.  This gave equal positive and
 * negative swings for Cb/Cr, but meant that grayscale values (Cb=Cr=0)
 * were not represented exactly.  Now we sacrifice exact representation of
 * maximum red and maximum blue in order to get exact grayscales.
 *
 * To avoid floating-point arithmetic, we represent the fractional constants
 * as integers scaled up by 2^16 (about 4 digits precision); we have to divide
 * the products by 2^16, with appropriate rounding, to get the correct answer.
 *
 * For even more speed, we avoid doing any multiplications in the inner loop
 * by precalculating the constants times R,G,B for all possible values.
 * For 8-bit JSAMPLEs this is very reasonable (only 256 entries per table);
 * for 12-bit samples it is still acceptable.  It's not very reasonable for
 * 16-bit samples, but if you want lossless storage you shouldn't be changing
 * colorspace anyway.
 * The CENTERJSAMPLE offsets and the rounding fudge-factor of 0.5 are included
 * in the tables to save adding them separately in the inner loop.
 */

pub const SCALEBITS: libc::c_int = 16 as libc::c_int;
/* speediest right-shift on some machines */

pub const CBCR_OFFSET: crate::jpegint_h::JLONG =
    (crate::jmorecfg_h::CENTERJSAMPLE as crate::jpegint_h::JLONG) << SCALEBITS;

pub const ONE_HALF: crate::jpegint_h::JLONG =
    (1 as libc::c_int as crate::jpegint_h::JLONG) << SCALEBITS - 1 as libc::c_int;
/* We allocate one big table and divide it up into eight parts, instead of
 * doing eight alloc_small requests.  This lets us use a single table base
 * address, which can be held in a register in the inner loops on many
 * machines (more than can hold all eight addresses, anyway).
 */

pub const R_Y_OFF: libc::c_int = 0 as libc::c_int;
/* offset to R => Y section */

pub const G_Y_OFF: libc::c_int =
    1 as libc::c_int * (crate::jmorecfg_h::MAXJSAMPLE + 1 as libc::c_int);
/* offset to G => Y section */

pub const B_Y_OFF: libc::c_int =
    2 as libc::c_int * (crate::jmorecfg_h::MAXJSAMPLE + 1 as libc::c_int);
/* etc. */

pub const R_CB_OFF: libc::c_int =
    3 as libc::c_int * (crate::jmorecfg_h::MAXJSAMPLE + 1 as libc::c_int);

pub const G_CB_OFF: libc::c_int =
    4 as libc::c_int * (crate::jmorecfg_h::MAXJSAMPLE + 1 as libc::c_int);

pub const B_CB_OFF: libc::c_int =
    5 as libc::c_int * (crate::jmorecfg_h::MAXJSAMPLE + 1 as libc::c_int);

pub const R_CR_OFF: libc::c_int = B_CB_OFF;
/* B=>Cb, R=>Cr are the same */

pub const G_CR_OFF: libc::c_int =
    6 as libc::c_int * (crate::jmorecfg_h::MAXJSAMPLE + 1 as libc::c_int);

pub const B_CR_OFF: libc::c_int =
    7 as libc::c_int * (crate::jmorecfg_h::MAXJSAMPLE + 1 as libc::c_int);

pub const TABLE_SIZE: libc::c_int =
    8 as libc::c_int * (crate::jmorecfg_h::MAXJSAMPLE + 1 as libc::c_int);
/* Include inline routines for colorspace extensions */

pub const RGB_PIXELSIZE_4: libc::c_int = crate::jmorecfg_h::EXT_RGB_PIXELSIZE;

pub const RGB_PIXELSIZE_2: libc::c_int = crate::jmorecfg_h::EXT_RGBX_PIXELSIZE;

pub const RGB_PIXELSIZE_3: libc::c_int = crate::jmorecfg_h::EXT_BGR_PIXELSIZE;

pub const RGB_PIXELSIZE_1: libc::c_int = crate::jmorecfg_h::EXT_BGRX_PIXELSIZE;

pub const RGB_PIXELSIZE_0: libc::c_int = crate::jmorecfg_h::EXT_XBGR_PIXELSIZE;

pub const RGB_PIXELSIZE: libc::c_int = crate::jmorecfg_h::EXT_XRGB_PIXELSIZE;
/*
 * Initialize for RGB->YCC colorspace conversion.
 */

unsafe extern "C" fn rgb_ycc_start(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut rgb_ycc_tab: *mut crate::jpegint_h::JLONG = 0 as *mut crate::jpegint_h::JLONG;
    let mut i: crate::jpegint_h::JLONG = 0;
    /* Allocate and fill in the conversion tables. */
    rgb_ycc_tab = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        (TABLE_SIZE as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<crate::jpegint_h::JLONG>() as libc::c_ulong),
    ) as *mut crate::jpegint_h::JLONG;
    (*cconvert).rgb_ycc_tab = rgb_ycc_tab;
    i = 0 as libc::c_int as crate::jpegint_h::JLONG;
    while i <= crate::jmorecfg_h::MAXJSAMPLE as libc::c_long {
        *rgb_ycc_tab.offset((i + R_Y_OFF as libc::c_long) as isize) =
            (0.29900f64 * ((1 as libc::c_long) << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG
                * i;
        *rgb_ycc_tab.offset((i + G_Y_OFF as libc::c_long) as isize) =
            (0.58700f64 * ((1 as libc::c_long) << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG
                * i;
        *rgb_ycc_tab.offset((i + B_Y_OFF as libc::c_long) as isize) =
            (0.11400f64 * ((1 as libc::c_long) << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG
                * i
                + ONE_HALF;
        *rgb_ycc_tab.offset((i + R_CB_OFF as libc::c_long) as isize) =
            -((0.16874f64 * ((1 as libc::c_long) << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG)
                * i;
        *rgb_ycc_tab.offset((i + G_CB_OFF as libc::c_long) as isize) =
            -((0.33126f64 * ((1 as libc::c_long) << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG)
                * i;
        /* We use a rounding fudge-factor of 0.5-epsilon for Cb and Cr.
         * This ensures that the maximum output will round to MAXJSAMPLE
         * not MAXJSAMPLE+1, and thus that we don't have to range-limit.
         */
        *rgb_ycc_tab.offset((i + B_CB_OFF as libc::c_long) as isize) =
            (0.50000f64 * ((1 as libc::c_long) << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG
                * i
                + CBCR_OFFSET
                + ONE_HALF
                - 1 as libc::c_int as libc::c_long;
        /*  B=>Cb and R=>Cr tables are the same
            rgb_ycc_tab[i + R_CR_OFF] = FIX(0.50000) * i  + CBCR_OFFSET + ONE_HALF - 1;
        */
        *rgb_ycc_tab.offset((i + G_CR_OFF as libc::c_long) as isize) =
            -((0.41869f64 * ((1 as libc::c_long) << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG)
                * i;
        *rgb_ycc_tab.offset((i + B_CR_OFF as libc::c_long) as isize) =
            -((0.08131f64 * ((1 as libc::c_long) << SCALEBITS) as libc::c_double + 0.5f64)
                as crate::jpegint_h::JLONG)
                * i;
        i += 1
    }
}
/*
 * Convert some rows of samples to the JPEG colorspace.
 */

unsafe extern "C" fn rgb_ycc_convert(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    match (*cinfo).in_color_space as libc::c_uint {
        6 => {
            extrgb_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        7 | 12 => {
            extrgbx_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        8 => {
            extbgr_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        9 | 13 => {
            extbgrx_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        10 | 14 => {
            extxbgr_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        11 | 15 => {
            extxrgb_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        _ => {
            rgb_ycc_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
    };
}
/* *************** Cases other than RGB -> YCbCr **************/
/*
 * Convert some rows of samples to the JPEG colorspace.
 */

unsafe extern "C" fn rgb_gray_convert(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    match (*cinfo).in_color_space as libc::c_uint {
        6 => {
            extrgb_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        7 | 12 => {
            extrgbx_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        8 => {
            extbgr_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        9 | 13 => {
            extbgrx_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        10 | 14 => {
            extxbgr_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        11 | 15 => {
            extxrgb_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        _ => {
            rgb_gray_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
    };
}
/*
 * Extended RGB to plain RGB conversion
 */

unsafe extern "C" fn rgb_rgb_convert(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    match (*cinfo).in_color_space as libc::c_uint {
        6 => {
            extrgb_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        7 | 12 => {
            extrgbx_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        8 => {
            extbgr_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        9 | 13 => {
            extbgrx_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        10 | 14 => {
            extxbgr_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        11 | 15 => {
            extxrgb_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
        _ => {
            rgb_rgb_convert_internal(cinfo, input_buf, output_buf, output_row, num_rows);
        }
    };
}
/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles Adobe-style CMYK->YCCK conversion,
 * where we convert R=1-C, G=1-M, and B=1-Y to YCbCr using the same
 * conversion as above, while passing K (black) unchanged.
 * We assume rgb_ycc_start has been called.
 */

unsafe extern "C" fn cmyk_ycck_convert(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut cconvert: my_cconvert_ptr = (*cinfo).cconvert as my_cconvert_ptr;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut ctab: *mut crate::jpegint_h::JLONG = (*cconvert).rgb_ycc_tab;
    let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outptr3: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as libc::c_int) {
            break;
        }
        let fresh21 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh21;
        outptr0 = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
        outptr1 = *(*output_buf.offset(1 as libc::c_int as isize)).offset(output_row as isize);
        outptr2 = *(*output_buf.offset(2 as libc::c_int as isize)).offset(output_row as isize);
        outptr3 = *(*output_buf.offset(3 as libc::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            r = crate::jmorecfg_h::MAXJSAMPLE
                - *inptr.offset(0 as libc::c_int as isize) as libc::c_int;
            g = crate::jmorecfg_h::MAXJSAMPLE
                - *inptr.offset(1 as libc::c_int as isize) as libc::c_int;
            b = crate::jmorecfg_h::MAXJSAMPLE
                - *inptr.offset(2 as libc::c_int as isize) as libc::c_int;
            /* K passes through as-is */
            *outptr3.offset(col as isize) = *inptr.offset(3 as libc::c_int as isize); /* don't need GETJSAMPLE here */
            inptr = inptr.offset(4 as libc::c_int as isize);
            /* If the inputs are 0..MAXJSAMPLE, the outputs of these equations
             * must be too; we do not need an explicit range-limiting operation.
             * Hence the value being shifted is never negative, and we don't
             * need the general RIGHT_SHIFT macro.
             */
            /* Y */
            *outptr0.offset(col as isize) = (*ctab.offset((r + R_Y_OFF) as isize)
                + *ctab.offset((g + G_Y_OFF) as isize)
                + *ctab.offset((b + B_Y_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            /* Cb */
            *outptr1.offset(col as isize) = (*ctab.offset((r + R_CB_OFF) as isize)
                + *ctab.offset((g + G_CB_OFF) as isize)
                + *ctab.offset((b + B_CB_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            /* Cr */
            *outptr2.offset(col as isize) = (*ctab.offset((r + R_CR_OFF) as isize)
                + *ctab.offset((g + G_CR_OFF) as isize)
                + *ctab.offset((b + B_CR_OFF) as isize)
                >> SCALEBITS)
                as crate::jmorecfg_h::JSAMPLE;
            col = col.wrapping_add(1)
        }
    }
}
/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles grayscale output with no conversion.
 * The source can be either plain grayscale or YCbCr (since Y == gray).
 */

unsafe extern "C" fn grayscale_convert(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE; /* don't need GETJSAMPLE() here */
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    let mut instride: libc::c_int = (*cinfo).input_components;
    loop {
        num_rows -= 1;
        if !(num_rows >= 0 as libc::c_int) {
            break;
        }
        let fresh22 = input_buf;
        input_buf = input_buf.offset(1);
        inptr = *fresh22;
        outptr = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
        output_row = output_row.wrapping_add(1);
        col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
        while col < num_cols {
            *outptr.offset(col as isize) = *inptr.offset(0 as libc::c_int as isize);
            inptr = inptr.offset(instride as isize);
            col = col.wrapping_add(1)
        }
    }
}
/*
 * Convert some rows of samples to the JPEG colorspace.
 * This version handles multi-component colorspaces without conversion.
 * We assume input_components == num_components.
 */

unsafe extern "C" fn null_convert(
    mut cinfo: crate::jpeglib_h::j_compress_ptr,
    mut input_buf: crate::jpeglib_h::JSAMPARRAY,
    mut output_buf: crate::jpeglib_h::JSAMPIMAGE,
    mut output_row: crate::jmorecfg_h::JDIMENSION,
    mut num_rows: libc::c_int,
) {
    let mut inptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outptr: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outptr0: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outptr1: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outptr2: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outptr3: crate::jpeglib_h::JSAMPROW = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut col: crate::jmorecfg_h::JDIMENSION = 0;
    let mut ci: libc::c_int = 0;
    let mut nc: libc::c_int = (*cinfo).num_components;
    let mut num_cols: crate::jmorecfg_h::JDIMENSION = (*cinfo).image_width;
    if nc == 3 as libc::c_int {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh23 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh23;
            outptr0 = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            outptr1 = *(*output_buf.offset(1 as libc::c_int as isize)).offset(output_row as isize);
            outptr2 = *(*output_buf.offset(2 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                let fresh24 = inptr;
                inptr = inptr.offset(1);
                *outptr0.offset(col as isize) = *fresh24;
                let fresh25 = inptr;
                inptr = inptr.offset(1);
                *outptr1.offset(col as isize) = *fresh25;
                let fresh26 = inptr;
                inptr = inptr.offset(1);
                *outptr2.offset(col as isize) = *fresh26;
                col = col.wrapping_add(1)
            }
        }
    } else if nc == 4 as libc::c_int {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            let fresh27 = input_buf;
            input_buf = input_buf.offset(1);
            inptr = *fresh27;
            outptr0 = *(*output_buf.offset(0 as libc::c_int as isize)).offset(output_row as isize);
            outptr1 = *(*output_buf.offset(1 as libc::c_int as isize)).offset(output_row as isize);
            outptr2 = *(*output_buf.offset(2 as libc::c_int as isize)).offset(output_row as isize);
            outptr3 = *(*output_buf.offset(3 as libc::c_int as isize)).offset(output_row as isize);
            output_row = output_row.wrapping_add(1);
            col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
            while col < num_cols {
                let fresh28 = inptr;
                inptr = inptr.offset(1);
                *outptr0.offset(col as isize) = *fresh28;
                let fresh29 = inptr;
                inptr = inptr.offset(1);
                *outptr1.offset(col as isize) = *fresh29;
                let fresh30 = inptr;
                inptr = inptr.offset(1);
                *outptr2.offset(col as isize) = *fresh30;
                let fresh31 = inptr;
                inptr = inptr.offset(1);
                *outptr3.offset(col as isize) = *fresh31;
                col = col.wrapping_add(1)
            }
        }
    } else {
        loop {
            num_rows -= 1;
            if !(num_rows >= 0 as libc::c_int) {
                break;
            }
            /* It seems fastest to make a separate pass for each component. */
            ci = 0 as libc::c_int; /* don't need GETJSAMPLE() here */
            while ci < nc {
                inptr = *input_buf;
                outptr = *(*output_buf.offset(ci as isize)).offset(output_row as isize);
                col = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
                while col < num_cols {
                    *outptr.offset(col as isize) = *inptr.offset(ci as isize);
                    inptr = inptr.offset(nc as isize);
                    col = col.wrapping_add(1)
                }
                ci += 1
            }
            input_buf = input_buf.offset(1);
            output_row = output_row.wrapping_add(1)
        }
    };
}
/*
 * Empty method for start_pass.
 */

unsafe extern "C" fn null_method(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    /* no work needed */
}
/*
 * Module initialization routine for input colorspace conversion.
 */
#[no_mangle]

pub unsafe extern "C" fn jinit_color_converter(mut cinfo: crate::jpeglib_h::j_compress_ptr) {
    let mut cconvert: my_cconvert_ptr = 0 as *mut my_color_converter;
    cconvert = Some(
        (*(*cinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        cinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ::std::mem::size_of::<my_color_converter>() as libc::c_ulong,
    ) as my_cconvert_ptr;
    (*cinfo).cconvert = cconvert as *mut crate::jpegint_h::jpeg_color_converter;
    /* set start_pass to null method until we find out differently */
    (*cconvert).pub_0.start_pass =
        Some(null_method as unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> ());
    /* Make sure input_components agrees with in_color_space */
    match (*cinfo).in_color_space as libc::c_uint {
        1 => {
            if (*cinfo).input_components != 1 as libc::c_int {
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
        }
        2 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => {
            if (*cinfo).input_components != rgb_pixelsize[(*cinfo).in_color_space as usize] {
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
        }
        3 => {
            if (*cinfo).input_components != 3 as libc::c_int {
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
        }
        4 | 5 => {
            if (*cinfo).input_components != 4 as libc::c_int {
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
        }
        _ => {
            /* JCS_UNKNOWN can be anything */
            if (*cinfo).input_components < 1 as libc::c_int {
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
        }
    }
    /* Check num_components, set conversion method based on requested space */
    match (*cinfo).jpeg_color_space as libc::c_uint {
        1 => {
            if (*cinfo).num_components != 1 as libc::c_int {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_J_COLORSPACE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            if (*cinfo).in_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_GRAYSCALE as libc::c_int as libc::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    grayscale_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: libc::c_int,
                        ) -> (),
                )
            } else if (*cinfo).in_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_RGB as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_RGB as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_RGBX as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_BGR as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_BGRX as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_XBGR as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_XRGB as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_RGBA as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_BGRA as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_ABGR as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_ARGB as libc::c_int as libc::c_uint
            {
                if crate::src::simd::x86_64::jsimd::jsimd_can_rgb_gray() != 0 {
                    (*cconvert).pub_0.color_convert = Some(
                        crate::src::simd::x86_64::jsimd::jsimd_rgb_gray_convert
                            as unsafe extern "C" fn(
                                _: crate::jpeglib_h::j_compress_ptr,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: crate::jpeglib_h::JSAMPIMAGE,
                                _: crate::jmorecfg_h::JDIMENSION,
                                _: libc::c_int,
                            ) -> (),
                    )
                } else {
                    (*cconvert).pub_0.start_pass = Some(
                        rgb_ycc_start
                            as unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> (),
                    );
                    (*cconvert).pub_0.color_convert = Some(
                        rgb_gray_convert
                            as unsafe extern "C" fn(
                                _: crate::jpeglib_h::j_compress_ptr,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: crate::jpeglib_h::JSAMPIMAGE,
                                _: crate::jmorecfg_h::JDIMENSION,
                                _: libc::c_int,
                            ) -> (),
                    )
                }
            } else if (*cinfo).in_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_YCbCr as libc::c_int as libc::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    grayscale_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: libc::c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_CONVERSION_NOTIMPL as libc::c_int;
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
        2 => {
            if (*cinfo).num_components != 3 as libc::c_int {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_J_COLORSPACE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            if rgb_red[(*cinfo).in_color_space as usize] == 0 as libc::c_int
                && rgb_green[(*cinfo).in_color_space as usize] == 1 as libc::c_int
                && rgb_blue[(*cinfo).in_color_space as usize] == 2 as libc::c_int
                && rgb_pixelsize[(*cinfo).in_color_space as usize] == 3 as libc::c_int
            {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: libc::c_int,
                        ) -> (),
                )
            } else if (*cinfo).in_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_RGB as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_RGB as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_RGBX as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_BGR as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_BGRX as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_XBGR as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_XRGB as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_RGBA as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_BGRA as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_ABGR as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_ARGB as libc::c_int as libc::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    rgb_rgb_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: libc::c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_CONVERSION_NOTIMPL as libc::c_int;
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
        3 => {
            if (*cinfo).num_components != 3 as libc::c_int {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_J_COLORSPACE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            if (*cinfo).in_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_RGB as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_RGB as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_RGBX as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_BGR as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_BGRX as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_XBGR as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_XRGB as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_RGBA as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_BGRA as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_ABGR as libc::c_int as libc::c_uint
                || (*cinfo).in_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_EXT_ARGB as libc::c_int as libc::c_uint
            {
                if crate::src::simd::x86_64::jsimd::jsimd_can_rgb_ycc() != 0 {
                    (*cconvert).pub_0.color_convert = Some(
                        crate::src::simd::x86_64::jsimd::jsimd_rgb_ycc_convert
                            as unsafe extern "C" fn(
                                _: crate::jpeglib_h::j_compress_ptr,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: crate::jpeglib_h::JSAMPIMAGE,
                                _: crate::jmorecfg_h::JDIMENSION,
                                _: libc::c_int,
                            ) -> (),
                    )
                } else {
                    (*cconvert).pub_0.start_pass = Some(
                        rgb_ycc_start
                            as unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> (),
                    );
                    (*cconvert).pub_0.color_convert = Some(
                        rgb_ycc_convert
                            as unsafe extern "C" fn(
                                _: crate::jpeglib_h::j_compress_ptr,
                                _: crate::jpeglib_h::JSAMPARRAY,
                                _: crate::jpeglib_h::JSAMPIMAGE,
                                _: crate::jmorecfg_h::JDIMENSION,
                                _: libc::c_int,
                            ) -> (),
                    )
                }
            } else if (*cinfo).in_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_YCbCr as libc::c_int as libc::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: libc::c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_CONVERSION_NOTIMPL as libc::c_int;
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
        4 => {
            if (*cinfo).num_components != 4 as libc::c_int {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_J_COLORSPACE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            if (*cinfo).in_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_CMYK as libc::c_int as libc::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: libc::c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_CONVERSION_NOTIMPL as libc::c_int;
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
        5 => {
            if (*cinfo).num_components != 4 as libc::c_int {
                (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_J_COLORSPACE as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            if (*cinfo).in_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_CMYK as libc::c_int as libc::c_uint
            {
                (*cconvert).pub_0.start_pass = Some(
                    rgb_ycc_start
                        as unsafe extern "C" fn(_: crate::jpeglib_h::j_compress_ptr) -> (),
                );
                (*cconvert).pub_0.color_convert = Some(
                    cmyk_ycck_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: libc::c_int,
                        ) -> (),
                )
            } else if (*cinfo).in_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_YCCK as libc::c_int as libc::c_uint
            {
                (*cconvert).pub_0.color_convert = Some(
                    null_convert
                        as unsafe extern "C" fn(
                            _: crate::jpeglib_h::j_compress_ptr,
                            _: crate::jpeglib_h::JSAMPARRAY,
                            _: crate::jpeglib_h::JSAMPIMAGE,
                            _: crate::jmorecfg_h::JDIMENSION,
                            _: libc::c_int,
                        ) -> (),
                )
            } else {
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_CONVERSION_NOTIMPL as libc::c_int;
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
        _ => {
            /* allow null conversion of JCS_UNKNOWN */
            if (*cinfo).jpeg_color_space as libc::c_uint != (*cinfo).in_color_space as libc::c_uint
                || (*cinfo).num_components != (*cinfo).input_components
            {
                (*(*cinfo).err).msg_code =
                    crate::src::jerror::JERR_CONVERSION_NOTIMPL as libc::c_int;
                Some(
                    (*(*cinfo).err)
                        .error_exit
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
            }
            (*cconvert).pub_0.color_convert = Some(
                null_convert
                    as unsafe extern "C" fn(
                        _: crate::jpeglib_h::j_compress_ptr,
                        _: crate::jpeglib_h::JSAMPARRAY,
                        _: crate::jpeglib_h::JSAMPIMAGE,
                        _: crate::jmorecfg_h::JDIMENSION,
                        _: libc::c_int,
                    ) -> (),
            )
        }
    };
}
