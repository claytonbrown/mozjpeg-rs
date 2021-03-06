// =============== BEGIN turbojpeg_h ================

/* *
 * Chrominance subsampling options.
 * When pixels are converted from RGB to YCbCr (see #TJCS_YCbCr) or from CMYK
 * to YCCK (see #TJCS_YCCK) as part of the JPEG compression process, some of
 * the Cb and Cr (chrominance) components can be discarded or averaged together
 * to produce a smaller image with little perceptible loss of image clarity
 * (the human eye is more sensitive to small changes in brightness than to
 * small changes in color.)  This is called "chrominance subsampling".
 */
pub type TJSAMP = libc::c_uint;
/* *
 * Pixel formats
 */

pub type TJPF = libc::c_int;
/* *
 * The number of JPEG colorspaces
 */
/* *
 * JPEG colorspaces
 */

pub type TJCS = libc::c_uint;
/* *
 * The number of error codes
 */
/* *
 * Error codes
 */

pub type TJERR = libc::c_uint;
/* *
 * The number of transform operations
 */
/* *
 * Transform operations for #tjTransform()
 */

pub type TJXOP = libc::c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tjscalingfactor {
    pub num: libc::c_int,
    pub denom: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tjregion {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub w: libc::c_int,
    pub h: libc::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tjtransform {
    pub r: crate::src::turbojpeg::tjregion,
    pub op: libc::c_int,
    pub options: libc::c_int,
    pub data: *mut libc::c_void,
    pub customFilter: Option<
        unsafe extern "C" fn(
            _: *mut libc::c_short,
            _: crate::src::turbojpeg::tjregion,
            _: crate::src::turbojpeg::tjregion,
            _: libc::c_int,
            _: libc::c_int,
            _: *mut crate::src::turbojpeg::tjtransform,
        ) -> libc::c_int,
    >,
}
/* *
 * TurboJPEG instance handle
 */

pub type tjhandle = *mut libc::c_void;
/* *
 * 4:1:1 chrominance subsampling.  The JPEG or YUV image will contain one
 * chrominance component for every 4x1 block of pixels in the source image.
 * JPEG images compressed with 4:1:1 subsampling will be almost exactly the
 * same size as those compressed with 4:2:0 subsampling, and in the
 * aggregate, both subsampling methods produce approximately the same
 * perceptual quality.  However, 4:1:1 is better able to reproduce sharp
 * horizontal features.
 *
 * @note 4:1:1 subsampling is not fully accelerated in libjpeg-turbo.
 */

pub const TJSAMP_411: crate::src::turbojpeg::TJSAMP = 5;
/* *
 * 4:4:0 chrominance subsampling.  The JPEG or YUV image will contain one
 * chrominance component for every 1x2 block of pixels in the source image.
 *
 * @note 4:4:0 subsampling is not fully accelerated in libjpeg-turbo.
 */

pub const TJSAMP_440: crate::src::turbojpeg::TJSAMP = 4;
/* *
 * Grayscale.  The JPEG or YUV image will contain no chrominance components.
 */

pub const TJSAMP_GRAY: crate::src::turbojpeg::TJSAMP = 3;
/* *
 * 4:2:0 chrominance subsampling.  The JPEG or YUV image will contain one
 * chrominance component for every 2x2 block of pixels in the source image.
 */

pub const TJSAMP_420: crate::src::turbojpeg::TJSAMP = 2;
/* *
 * 4:2:2 chrominance subsampling.  The JPEG or YUV image will contain one
 * chrominance component for every 2x1 block of pixels in the source image.
 */

pub const TJSAMP_422: crate::src::turbojpeg::TJSAMP = 1;
/* *
 * 4:4:4 chrominance subsampling (no chrominance subsampling).  The JPEG or
 * YUV image will contain one chrominance component for every pixel in the
 * source image.
 */

pub const TJSAMP_444: crate::src::turbojpeg::TJSAMP = 0;
/* *
 * Unknown pixel format.  Currently this is only used by #tjLoadImage().
 */

pub const TJPF_UNKNOWN: crate::src::turbojpeg::TJPF = -1;
/* *
 * CMYK pixel format.  Unlike RGB, which is an additive color model used
 * primarily for display, CMYK (Cyan/Magenta/Yellow/Key) is a subtractive
 * color model used primarily for printing.  In the CMYK color model, the
 * value of each color component typically corresponds to an amount of cyan,
 * magenta, yellow, or black ink that is applied to a white background.  In
 * order to convert between CMYK and RGB, it is necessary to use a color
 * management system (CMS.)  A CMS will attempt to map colors within the
 * printer's gamut to perceptually similar colors in the display's gamut and
 * vice versa, but the mapping is typically not 1:1 or reversible, nor can it
 * be defined with a simple formula.  Thus, such a conversion is out of scope
 * for a codec library.  However, the TurboJPEG API allows for compressing
 * CMYK pixels into a YCCK JPEG image (see #TJCS_YCCK) and decompressing YCCK
 * JPEG images into CMYK pixels.
 */

pub const TJPF_CMYK: crate::src::turbojpeg::TJPF = 11;
/* *
 * ARGB pixel format.  This is the same as @ref TJPF_XRGB, except that when
 * decompressing, the X component is guaranteed to be 0xFF, which can be
 * interpreted as an opaque alpha channel.
 */

pub const TJPF_ARGB: crate::src::turbojpeg::TJPF = 10;
/* *
 * ABGR pixel format.  This is the same as @ref TJPF_XBGR, except that when
 * decompressing, the X component is guaranteed to be 0xFF, which can be
 * interpreted as an opaque alpha channel.
 */

pub const TJPF_ABGR: crate::src::turbojpeg::TJPF = 9;
/* *
 * BGRA pixel format.  This is the same as @ref TJPF_BGRX, except that when
 * decompressing, the X component is guaranteed to be 0xFF, which can be
 * interpreted as an opaque alpha channel.
 */

pub const TJPF_BGRA: crate::src::turbojpeg::TJPF = 8;
/* *
 * RGBA pixel format.  This is the same as @ref TJPF_RGBX, except that when
 * decompressing, the X component is guaranteed to be 0xFF, which can be
 * interpreted as an opaque alpha channel.
 */

pub const TJPF_RGBA: crate::src::turbojpeg::TJPF = 7;
/* *
 * Grayscale pixel format.  Each 1-byte pixel represents a luminance
 * (brightness) level from 0 to 255.
 */

pub const TJPF_GRAY: crate::src::turbojpeg::TJPF = 6;
/* *
 * XRGB pixel format.  The red, green, and blue components in the image are
 * stored in 4-byte pixels in the order B, G, R from highest to lowest byte
 * address within each pixel.  The X component is ignored when compressing
 * and undefined when decompressing.
 */

pub const TJPF_XRGB: crate::src::turbojpeg::TJPF = 5;
/* *
 * XBGR pixel format.  The red, green, and blue components in the image are
 * stored in 4-byte pixels in the order R, G, B from highest to lowest byte
 * address within each pixel.  The X component is ignored when compressing
 * and undefined when decompressing.
 */

pub const TJPF_XBGR: crate::src::turbojpeg::TJPF = 4;
/* *
 * BGRX pixel format.  The red, green, and blue components in the image are
 * stored in 4-byte pixels in the order B, G, R from lowest to highest byte
 * address within each pixel.  The X component is ignored when compressing
 * and undefined when decompressing.
 */

pub const TJPF_BGRX: crate::src::turbojpeg::TJPF = 3;
/* *
 * RGBX pixel format.  The red, green, and blue components in the image are
 * stored in 4-byte pixels in the order R, G, B from lowest to highest byte
 * address within each pixel.  The X component is ignored when compressing
 * and undefined when decompressing.
 */

pub const TJPF_RGBX: crate::src::turbojpeg::TJPF = 2;
/* *
 * BGR pixel format.  The red, green, and blue components in the image are
 * stored in 3-byte pixels in the order B, G, R from lowest to highest byte
 * address within each pixel.
 */

pub const TJPF_BGR: crate::src::turbojpeg::TJPF = 1;
/* *
 * RGB pixel format.  The red, green, and blue components in the image are
 * stored in 3-byte pixels in the order R, G, B from lowest to highest byte
 * address within each pixel.
 */

pub const TJPF_RGB: crate::src::turbojpeg::TJPF = 0;
/* *
 * YCCK colorspace.  YCCK (AKA "YCbCrK") is not an absolute colorspace but
 * rather a mathematical transformation of CMYK designed solely for storage
 * and transmission.  It is to CMYK as YCbCr is to RGB.  CMYK pixels can be
 * reversibly transformed into YCCK, and as with YCbCr, the chrominance
 * components in the YCCK pixels can be subsampled without incurring major
 * perceptual loss.  YCCK JPEG images can only be compressed from and
 * decompressed to CMYK pixels.
 */

pub const TJCS_YCCK: crate::src::turbojpeg::TJCS = 4;
/* *
 * CMYK colorspace.  When compressing the JPEG image, the C, M, Y, and K
 * components in the source image are reordered into image planes, but no
 * colorspace conversion or subsampling is performed.  CMYK JPEG images can
 * only be decompressed to CMYK pixels.
 */

pub const TJCS_CMYK: crate::src::turbojpeg::TJCS = 3;
/* *
 * Grayscale colorspace.  The JPEG image retains only the luminance data (Y
 * component), and any color data from the source image is discarded.
 * Grayscale JPEG images can be compressed from and decompressed to any of
 * the extended RGB pixel formats or grayscale, or they can be decompressed
 * to YUV planar images.
 */

pub const TJCS_GRAY: crate::src::turbojpeg::TJCS = 2;
/* *
 * YCbCr colorspace.  YCbCr is not an absolute colorspace but rather a
 * mathematical transformation of RGB designed solely for storage and
 * transmission.  YCbCr images must be converted to RGB before they can
 * actually be displayed.  In the YCbCr colorspace, the Y (luminance)
 * component represents the black & white portion of the original image, and
 * the Cb and Cr (chrominance) components represent the color portion of the
 * original image.  Originally, the analog equivalent of this transformation
 * allowed the same signal to drive both black & white and color televisions,
 * but JPEG images use YCbCr primarily because it allows the color data to be
 * optionally subsampled for the purposes of reducing bandwidth or disk
 * space.  YCbCr is the most common JPEG colorspace, and YCbCr JPEG images
 * can be compressed from and decompressed to any of the extended RGB pixel
 * formats or grayscale, or they can be decompressed to YUV planar images.
 */

pub const TJCS_YCbCr: crate::src::turbojpeg::TJCS = 1;
/* *
 * RGB colorspace.  When compressing the JPEG image, the R, G, and B
 * components in the source image are reordered into image planes, but no
 * colorspace conversion or subsampling is performed.  RGB JPEG images can be
 * decompressed to any of the extended RGB pixel formats or grayscale, but
 * they cannot be decompressed to YUV images.
 */

pub const TJCS_RGB: crate::src::turbojpeg::TJCS = 0;
/* *
 * The error was fatal and non-recoverable.
 */

pub const TJERR_FATAL: crate::src::turbojpeg::TJERR = 1;
/* *
 * The error was non-fatal and recoverable, but the image may still be
 * corrupt.
 */

pub const TJERR_WARNING: crate::src::turbojpeg::TJERR = 0;
/* *
 * Rotate image counter-clockwise by 90 degrees.  This transform is imperfect
 * if there are any partial MCU blocks on the right edge (see
 * #TJXOPT_PERFECT.)
 */

pub const TJXOP_ROT270: crate::src::turbojpeg::TJXOP = 7;
/* *
 * Rotate image 180 degrees.  This transform is imperfect if there are any
 * partial MCU blocks in the image (see #TJXOPT_PERFECT.)
 */

pub const TJXOP_ROT180: crate::src::turbojpeg::TJXOP = 6;
/* *
 * Rotate image clockwise by 90 degrees.  This transform is imperfect if
 * there are any partial MCU blocks on the bottom edge (see
 * #TJXOPT_PERFECT.)
 */

pub const TJXOP_ROT90: crate::src::turbojpeg::TJXOP = 5;
/* *
 * Transverse transpose image (flip/mirror along upper right to lower left
 * axis.)  This transform is imperfect if there are any partial MCU blocks in
 * the image (see #TJXOPT_PERFECT.)
 */

pub const TJXOP_TRANSVERSE: crate::src::turbojpeg::TJXOP = 4;
/* *
 * Transpose image (flip/mirror along upper left to lower right axis.)  This
 * transform is always perfect.
 */

pub const TJXOP_TRANSPOSE: crate::src::turbojpeg::TJXOP = 3;
/* *
 * Flip (mirror) image vertically.  This transform is imperfect if there are
 * any partial MCU blocks on the bottom edge (see #TJXOPT_PERFECT.)
 */

pub const TJXOP_VFLIP: crate::src::turbojpeg::TJXOP = 2;
/* *
 * Flip (mirror) image horizontally.  This transform is imperfect if there
 * are any partial MCU blocks on the right edge (see #TJXOPT_PERFECT.)
 */

pub const TJXOP_HFLIP: crate::src::turbojpeg::TJXOP = 1;
/* *
 * Do not transform the position of the image pixels
 */

pub const TJXOP_NONE: crate::src::turbojpeg::TJXOP = 0;
/*
 * Copyright (C)2009-2015, 2017 D. R. Commander.  All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * - Redistributions of source code must retain the above copyright notice,
 *   this list of conditions and the following disclaimer.
 * - Redistributions in binary form must reproduce the above copyright notice,
 *   this list of conditions and the following disclaimer in the documentation
 *   and/or other materials provided with the distribution.
 * - Neither the name of the libjpeg-turbo Project nor the names of its
 *   contributors may be used to endorse or promote products derived from this
 *   software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS",
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */
/* *
 * @addtogroup TurboJPEG
 * TurboJPEG API.  This API provides an interface for generating, decoding, and
 * transforming planar YUV and JPEG images in memory.
 *
 * @anchor YUVnotes
 * YUV Image Format Notes
 * ----------------------
 * Technically, the JPEG format uses the YCbCr colorspace (which is technically
 * not a colorspace but a color transform), but per the convention of the
 * digital video community, the TurboJPEG API uses "YUV" to refer to an image
 * format consisting of Y, Cb, and Cr image planes.
 *
 * Each plane is simply a 2D array of bytes, each byte representing the value
 * of one of the components (Y, Cb, or Cr) at a particular location in the
 * image.  The width and height of each plane are determined by the image
 * width, height, and level of chrominance subsampling.   The luminance plane
 * width is the image width padded to the nearest multiple of the horizontal
 * subsampling factor (2 in the case of 4:2:0 and 4:2:2, 4 in the case of
 * 4:1:1, 1 in the case of 4:4:4 or grayscale.)  Similarly, the luminance plane
 * height is the image height padded to the nearest multiple of the vertical
 * subsampling factor (2 in the case of 4:2:0 or 4:4:0, 1 in the case of 4:4:4
 * or grayscale.)  This is irrespective of any additional padding that may be
 * specified as an argument to the various YUV functions.  The chrominance
 * plane width is equal to the luminance plane width divided by the horizontal
 * subsampling factor, and the chrominance plane height is equal to the
 * luminance plane height divided by the vertical subsampling factor.
 *
 * For example, if the source image is 35 x 35 pixels and 4:2:2 subsampling is
 * used, then the luminance plane would be 36 x 35 bytes, and each of the
 * chrominance planes would be 18 x 35 bytes.  If you specify a line padding of
 * 4 bytes on top of this, then the luminance plane would be 36 x 35 bytes, and
 * each of the chrominance planes would be 20 x 35 bytes.
 *
 * @{
 */
/* *
 * The number of chrominance subsampling options
 */

pub const TJ_NUMSAMP: libc::c_int = 6 as libc::c_int;
/* *
 * The number of pixel formats
 */

pub const TJ_NUMPF: libc::c_int = 12 as libc::c_int;
/* *
 * The uncompressed source/destination image is stored in bottom-up (Windows,
 * OpenGL) order, not top-down (X11) order.
 */

pub const TJFLAG_BOTTOMUP: libc::c_int = 2 as libc::c_int;
/* *
 * When decompressing an image that was compressed using chrominance
 * subsampling, use the fastest chrominance upsampling algorithm available in
 * the underlying codec.  The default is to use smooth upsampling, which
 * creates a smooth transition between neighboring chrominance components in
 * order to reduce upsampling artifacts in the decompressed image.
 */

pub const TJFLAG_FASTUPSAMPLE: libc::c_int = 256 as libc::c_int;
/* *
 * Disable buffer (re)allocation.  If passed to one of the JPEG compression or
 * transform functions, this flag will cause those functions to generate an
 * error if the JPEG image buffer is invalid or too small rather than
 * attempting to allocate or reallocate that buffer.  This reproduces the
 * behavior of earlier versions of TurboJPEG.
 */

pub const TJFLAG_NOREALLOC: libc::c_int = 1024 as libc::c_int;
/* *
 * Use the fastest DCT/IDCT algorithm available in the underlying codec.  The
 * default if this flag is not specified is implementation-specific.  For
 * example, the implementation of TurboJPEG for libjpeg[-turbo] uses the fast
 * algorithm by default when compressing, because this has been shown to have
 * only a very slight effect on accuracy, but it uses the accurate algorithm
 * when decompressing, because this has been shown to have a larger effect.
 */

pub const TJFLAG_FASTDCT: libc::c_int = 2048 as libc::c_int;
/* *
 * Use the most accurate DCT/IDCT algorithm available in the underlying codec.
 * The default if this flag is not specified is implementation-specific.  For
 * example, the implementation of TurboJPEG for libjpeg[-turbo] uses the fast
 * algorithm by default when compressing, because this has been shown to have
 * only a very slight effect on accuracy, but it uses the accurate algorithm
 * when decompressing, because this has been shown to have a larger effect.
 */

pub const TJFLAG_ACCURATEDCT: libc::c_int = 4096 as libc::c_int;
/* *
 * Immediately discontinue the current compression/decompression/transform
 * operation if the underlying codec throws a warning (non-fatal error).  The
 * default behavior is to allow the operation to complete unless a fatal error
 * is encountered.
 */

pub const TJFLAG_STOPONWARNING: libc::c_int = 8192 as libc::c_int;
/* *
 * Use progressive entropy coding in JPEG images generated by the compression
 * and transform functions.  Progressive entropy coding will generally improve
 * compression relative to baseline entropy coding (the default), but it will
 * reduce compression and decompression performance considerably.
 */

pub const TJFLAG_PROGRESSIVE: libc::c_int = 16384 as libc::c_int;
/* *
 * This option will cause #tjTransform() to return an error if the transform is
 * not perfect.  Lossless transforms operate on MCU blocks, whose size depends
 * on the level of chrominance subsampling used (see #tjMCUWidth
 * and #tjMCUHeight.)  If the image's width or height is not evenly divisible
 * by the MCU block size, then there will be partial MCU blocks on the right
 * and/or bottom edges.  It is not possible to move these partial MCU blocks to
 * the top or left of the image, so any transform that would require that is
 * "imperfect."  If this option is not specified, then any partial MCU blocks
 * that cannot be transformed will be left in place, which will create
 * odd-looking strips on the right or bottom edge of the image.
 */

pub const TJXOPT_PERFECT: libc::c_int = 1 as libc::c_int;
/* *
 * This option will cause #tjTransform() to discard any partial MCU blocks that
 * cannot be transformed.
 */

pub const TJXOPT_TRIM: libc::c_int = 2 as libc::c_int;
/* *
 * This option will enable lossless cropping.  See #tjTransform() for more
 * information.
 */

pub const TJXOPT_CROP: libc::c_int = 4 as libc::c_int;
/* *
 * This option will discard the color data in the input image and produce
 * a grayscale output image.
 */

pub const TJXOPT_GRAY: libc::c_int = 8 as libc::c_int;
/* *
 * This option will prevent #tjTransform() from outputting a JPEG image for
 * this particular transform (this can be used in conjunction with a custom
 * filter to capture the transformed DCT coefficients without transcoding
 * them.)
 */

pub const TJXOPT_NOOUTPUT: libc::c_int = 16 as libc::c_int;
/* *
 * This option will enable progressive entropy coding in the output image
 * generated by this particular transform.  Progressive entropy coding will
 * generally improve compression relative to baseline entropy coding (the
 * default), but it will reduce compression and decompression performance
 * considerably.
 */

pub const TJXOPT_PROGRESSIVE: libc::c_int = 32 as libc::c_int;
/* *
 * This option will prevent #tjTransform() from copying any extra markers
 * (including EXIF and ICC profile data) from the source image to the output
 * image.
 */

pub const TJXOPT_COPYNONE: libc::c_int = 64 as libc::c_int;
/* Deprecated functions and macros */

pub const TJFLAG_FORCEMMX: libc::c_int = 8 as libc::c_int;

pub const TJFLAG_FORCESSE: libc::c_int = 16 as libc::c_int;

pub const TJFLAG_FORCESSE2: libc::c_int = 32 as libc::c_int;
/* Backward compatibility functions and macros (nothing to see here) */

pub const NUMSUBOPT: libc::c_int = crate::src::turbojpeg::TJ_NUMSAMP;

pub const TJ_420: libc::c_int = crate::src::turbojpeg::TJSAMP_420 as libc::c_int;

pub const TJ_BGR: libc::c_int = 1 as libc::c_int;

pub const TJ_ALPHAFIRST: libc::c_int = 64 as libc::c_int;

pub const TJ_YUV: libc::c_int = 512 as libc::c_int;
use ::libc;

#[c2rust::header_src = "/home/sjcrane/projects/c2rust/mozjpeg/mozjpeg-c2rust/mozjpeg-c/turbojpeg.h:43"]
pub mod turbojpeg_h {

    /* *
     * MCU block width (in pixels) for a given level of chrominance subsampling.
     * MCU block sizes:
     * - 8x8 for no subsampling or grayscale
     * - 16x8 for 4:2:2
     * - 8x16 for 4:4:0
     * - 16x16 for 4:2:0
     * - 32x8 for 4:1:1
     */

    pub static mut tjMCUWidth: [libc::c_int; 6] = [
        8 as libc::c_int,
        16 as libc::c_int,
        16 as libc::c_int,
        8 as libc::c_int,
        8 as libc::c_int,
        32 as libc::c_int,
    ];
    /* *
     * MCU block height (in pixels) for a given level of chrominance subsampling.
     * MCU block sizes:
     * - 8x8 for no subsampling or grayscale
     * - 16x8 for 4:2:2
     * - 8x16 for 4:4:0
     * - 16x16 for 4:2:0
     * - 32x8 for 4:1:1
     */

    pub static mut tjMCUHeight: [libc::c_int; 6] = [
        8 as libc::c_int,
        8 as libc::c_int,
        16 as libc::c_int,
        8 as libc::c_int,
        16 as libc::c_int,
        8 as libc::c_int,
    ];

    /* *
     * Pixel size (in bytes) for a given pixel format
     */

    pub static mut tjPixelSize: [libc::c_int; 12] = [
        3 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
    ];

    /* *
     * @}
     */
}

#[c2rust::header_src = "/usr/include/ctype.h:36"]
pub mod ctype_h {
    #[inline]

    pub unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
        return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
}

pub use crate::stddef_h::size_t;
pub use crate::stddef_h::NULL;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__sigset_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::jmorecfg_h::boolean;
pub use crate::jmorecfg_h::FALSE;
pub use crate::jmorecfg_h::JCOEF;
pub use crate::jmorecfg_h::JDIMENSION;
pub use crate::jmorecfg_h::JOCTET;
pub use crate::jmorecfg_h::JSAMPLE;
pub use crate::jmorecfg_h::MAX_COMPONENTS;
pub use crate::jmorecfg_h::TRUE;
pub use crate::jmorecfg_h::UINT16;
pub use crate::jmorecfg_h::UINT8;
pub use crate::jpegint_h::inverse_DCT_method_ptr;
pub use crate::jpegint_h::jpeg_c_coef_controller;
pub use crate::jpegint_h::jpeg_c_main_controller;
pub use crate::jpegint_h::jpeg_c_prep_controller;
pub use crate::jpegint_h::jpeg_color_converter;
pub use crate::jpegint_h::jpeg_color_deconverter;
pub use crate::jpegint_h::jpeg_color_quantizer;
pub use crate::jpegint_h::jpeg_comp_master;
pub use crate::jpegint_h::jpeg_d_coef_controller;
pub use crate::jpegint_h::jpeg_d_main_controller;
pub use crate::jpegint_h::jpeg_d_post_controller;
pub use crate::jpegint_h::jpeg_decomp_master;
pub use crate::jpegint_h::jpeg_downsampler;
pub use crate::jpegint_h::jpeg_entropy_decoder;
pub use crate::jpegint_h::jpeg_entropy_encoder;
pub use crate::jpegint_h::jpeg_forward_dct;
pub use crate::jpegint_h::jpeg_input_controller;
pub use crate::jpegint_h::jpeg_inverse_dct;
pub use crate::jpegint_h::jpeg_marker_reader;
pub use crate::jpegint_h::jpeg_marker_writer;
pub use crate::jpegint_h::jpeg_upsampler;
pub use crate::jpegint_h::CSTATE_START;
pub use crate::jpegint_h::DSTATE_READY;
pub use crate::jpegint_h::DSTATE_START;
pub use crate::jpegint_h::JBUF_CRANK_DEST;
pub use crate::jpegint_h::JBUF_PASS_THRU;
pub use crate::jpegint_h::JBUF_REQUANT;
pub use crate::jpegint_h::JBUF_SAVE_AND_PASS;
pub use crate::jpegint_h::JBUF_SAVE_SOURCE;
pub use crate::jpegint_h::J_BUF_MODE;
pub use crate::jpeglib_h::j_common_ptr;
pub use crate::jpeglib_h::j_compress_ptr;
pub use crate::jpeglib_h::j_decompress_ptr;
pub use crate::jpeglib_h::jpeg_common_struct;
pub use crate::jpeglib_h::jpeg_component_info;
pub use crate::jpeglib_h::jpeg_compress_struct;
pub use crate::jpeglib_h::jpeg_decompress_struct;
pub use crate::jpeglib_h::jpeg_destination_mgr;
pub use crate::jpeglib_h::jpeg_error_mgr;
pub use crate::jpeglib_h::jpeg_marker_parser_method;
pub use crate::jpeglib_h::jpeg_marker_struct;
pub use crate::jpeglib_h::jpeg_memory_mgr;
pub use crate::jpeglib_h::jpeg_progress_mgr;
pub use crate::jpeglib_h::jpeg_saved_marker_ptr;
pub use crate::jpeglib_h::jpeg_scan_info;
pub use crate::jpeglib_h::jpeg_source_mgr;
pub use crate::jpeglib_h::jvirt_barray_control;
pub use crate::jpeglib_h::jvirt_barray_ptr;
pub use crate::jpeglib_h::jvirt_sarray_control;
pub use crate::jpeglib_h::jvirt_sarray_ptr;
pub use crate::jpeglib_h::C2RustUnnamed_2;
pub use crate::jpeglib_h::JCS_YCbCr;
pub use crate::jpeglib_h::DCTSIZE;
pub use crate::jpeglib_h::DCTSIZE2;
pub use crate::jpeglib_h::JBLOCK;
pub use crate::jpeglib_h::JBLOCKARRAY;
pub use crate::jpeglib_h::JBLOCKROW;
pub use crate::jpeglib_h::JCOEFPTR;
pub use crate::jpeglib_h::JCP_FASTEST;
pub use crate::jpeglib_h::JCP_MAX_COMPRESSION;
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
pub use crate::jpeglib_h::JDCT_FASTEST;
pub use crate::jpeglib_h::JDCT_FLOAT;
pub use crate::jpeglib_h::JDCT_IFAST;
pub use crate::jpeglib_h::JDCT_ISLOW;
pub use crate::jpeglib_h::JDITHER_FS;
pub use crate::jpeglib_h::JDITHER_NONE;
pub use crate::jpeglib_h::JDITHER_ORDERED;
pub use crate::jpeglib_h::JHUFF_TBL;
pub use crate::jpeglib_h::JMSG_LENGTH_MAX;
pub use crate::jpeglib_h::JPEG_REACHED_SOS;
pub use crate::jpeglib_h::JPOOL_IMAGE;
pub use crate::jpeglib_h::JQUANT_TBL;
pub use crate::jpeglib_h::JSAMPARRAY;
pub use crate::jpeglib_h::JSAMPIMAGE;
pub use crate::jpeglib_h::JSAMPROW;
pub use crate::jpeglib_h::J_COLOR_SPACE;
pub use crate::jpeglib_h::J_DCT_METHOD;
pub use crate::jpeglib_h::J_DITHER_MODE;
pub use crate::src::jcapimin::jpeg_CreateCompress;
pub use crate::src::jcapimin::jpeg_abort_compress;
pub use crate::src::jcapimin::jpeg_destroy_compress;
pub use crate::src::jcapimin::jpeg_finish_compress;
pub use crate::src::jcapistd::jpeg_start_compress;
pub use crate::src::jcapistd::jpeg_write_raw_data;
pub use crate::src::jcapistd::jpeg_write_scanlines;
pub use crate::src::jccolor::jinit_color_converter;
pub use crate::src::jcmaster::jinit_c_master_control;
pub use crate::src::jcomapi::jpeg_alloc_quant_table;
pub use crate::src::jcparam::jpeg_set_colorspace;
pub use crate::src::jcparam::jpeg_set_defaults;
pub use crate::src::jcparam::jpeg_set_quality;
pub use crate::src::jcparam::jpeg_simple_progression;
pub use crate::src::jcsample::jinit_downsampler;
pub use crate::src::jctrans::jpeg_copy_critical_parameters;
pub use crate::src::jctrans::jpeg_write_coefficients;
pub use crate::src::jdapimin::jpeg_CreateDecompress;
pub use crate::src::jdapimin::jpeg_abort_decompress;
pub use crate::src::jdapimin::jpeg_destroy_decompress;
pub use crate::src::jdapimin::jpeg_finish_decompress;
pub use crate::src::jdapimin::jpeg_read_header;
pub use crate::src::jdapistd::jpeg_read_raw_data;
pub use crate::src::jdapistd::jpeg_read_scanlines;
pub use crate::src::jdapistd::jpeg_start_decompress;
pub use crate::src::jdmaster::jinit_master_decompress;
pub use crate::src::jdmaster::jpeg_calc_output_dimensions;
pub use crate::src::jdtrans::jpeg_read_coefficients;
pub use crate::src::jerror::jpeg_std_error;
pub use crate::src::jutils::jcopy_sample_rows;
pub use crate::stdlib::C2RustUnnamed_0;
pub use crate::stdlib::__jmp_buf;

pub use crate::cderror_h::JERR_BMP_BADCMAP;
pub use crate::cderror_h::JERR_BMP_BADDEPTH;
pub use crate::cderror_h::JERR_BMP_BADHEADER;
pub use crate::cderror_h::JERR_BMP_BADPLANES;
pub use crate::cderror_h::JERR_BMP_COLORSPACE;
pub use crate::cderror_h::JERR_BMP_COMPRESSED;
pub use crate::cderror_h::JERR_BMP_EMPTY;
pub use crate::cderror_h::JERR_BMP_NOT;
pub use crate::cderror_h::JERR_BMP_OUTOFRANGE;
pub use crate::cderror_h::JERR_TGA_NOTCOMP;
pub use crate::cderror_h::JMSG_FIRSTADDONCODE;
pub use crate::cderror_h::JTRC_BMP;
pub use crate::cderror_h::JTRC_BMP_MAPPED;
pub use crate::cderror_h::JTRC_BMP_OS2;
pub use crate::cderror_h::JTRC_BMP_OS2_MAPPED;
pub use crate::cderror_h_0::JERR_BAD_CMAP_FILE;
pub use crate::cderror_h_0::JERR_PPM_COLORSPACE;
pub use crate::cderror_h_0::JERR_PPM_NONNUMERIC;
pub use crate::cderror_h_0::JERR_PPM_NOT;
pub use crate::cderror_h_0::JERR_PPM_OUTOFRANGE;
pub use crate::cderror_h_0::JERR_TOO_MANY_COLORS;
pub use crate::cderror_h_0::JERR_UNGETC_FAILED;
pub use crate::cderror_h_0::JERR_UNKNOWN_FORMAT;
pub use crate::cderror_h_0::JERR_UNSUPPORTED_FORMAT;
pub use crate::cderror_h_0::JMSG_LASTADDONCODE;
pub use crate::cderror_h_0::JTRC_PGM;
pub use crate::cderror_h_0::JTRC_PGM_TEXT;
pub use crate::cderror_h_0::JTRC_PPM;
pub use crate::cderror_h_0::JTRC_PPM_TEXT;
pub use crate::cdjpeg_h::cjpeg_source_ptr;
pub use crate::cdjpeg_h::cjpeg_source_struct;
pub use crate::cdjpeg_h::djpeg_dest_ptr;
pub use crate::cdjpeg_h::djpeg_dest_struct;
pub use crate::jconfig_h::JPEG_LIB_VERSION;
pub use crate::src::rdbmp::jinit_read_bmp;
pub use crate::src::rdppm::jinit_read_ppm;
pub use crate::src::transupp::jcopy_markers_execute;
pub use crate::src::transupp::jcopy_markers_setup;
pub use crate::src::transupp::jpeg_transform_info;
pub use crate::src::transupp::jtransform_adjust_parameters;
pub use crate::src::transupp::jtransform_execute_transform;
pub use crate::src::transupp::jtransform_execute_transformation;
pub use crate::src::transupp::jtransform_request_workspace;
pub use crate::src::transupp::JCOPYOPT_ALL;
pub use crate::src::transupp::JCOPYOPT_ALL_EXCEPT_ICC;
pub use crate::src::transupp::JCOPYOPT_COMMENTS;
pub use crate::src::transupp::JCOPYOPT_NONE;
pub use crate::src::transupp::JCOPY_OPTION;
pub use crate::src::transupp::JCROP_CODE;
pub use crate::src::transupp::JCROP_FORCE;
pub use crate::src::transupp::JCROP_NEG;
pub use crate::src::transupp::JCROP_POS;
pub use crate::src::transupp::JCROP_UNSET;
pub use crate::src::transupp::JXFORM_CODE;
pub use crate::src::transupp::JXFORM_FLIP_H;
pub use crate::src::transupp::JXFORM_FLIP_V;
pub use crate::src::transupp::JXFORM_NONE;
pub use crate::src::transupp::JXFORM_ROT_180;
pub use crate::src::transupp::JXFORM_ROT_270;
pub use crate::src::transupp::JXFORM_ROT_90;
pub use crate::src::transupp::JXFORM_TRANSPOSE;
pub use crate::src::transupp::JXFORM_TRANSVERSE;
pub use crate::src::turbojpeg::ctype_h::toupper;
pub use crate::src::turbojpeg::turbojpeg_h::tjMCUHeight;
pub use crate::src::turbojpeg::turbojpeg_h::tjMCUWidth;
pub use crate::src::turbojpeg::turbojpeg_h::tjPixelSize;
pub use crate::src::wrbmp::jinit_write_bmp;
pub use crate::src::wrppm::jinit_write_ppm;
pub use crate::stdlib::__ctype_toupper_loc;
use crate::stdlib::__errno_location;
pub use crate::stdlib::__jmp_buf_tag;
pub use crate::stdlib::_setjmp;
use crate::stdlib::abs;
pub use crate::stdlib::fclose;
pub use crate::stdlib::fopen;
use crate::stdlib::free;
pub use crate::stdlib::getc;
use crate::stdlib::getenv;
pub use crate::stdlib::jmp_buf;
pub use crate::stdlib::longjmp;
use crate::stdlib::malloc;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
use crate::stdlib::putenv;
pub use crate::stdlib::snprintf;
pub use crate::stdlib::sscanf;
use crate::stdlib::strcasecmp;
use crate::stdlib::strcmp;
use crate::stdlib::strerror;
use crate::stdlib::strlen;
use crate::stdlib::strrchr;
pub use crate::stdlib::ungetc;
pub use crate::stdlib::EOF;
extern "C" {
    /*
     * Copyright (C)2009-2018 D. R. Commander.  All Rights Reserved.
     * mozjpeg Modifications:
     * Copyright (C) 2014, Mozilla Corporation.
     *
     * Redistribution and use in source and binary forms, with or without
     * modification, are permitted provided that the following conditions are met:
     *
     * - Redistributions of source code must retain the above copyright notice,
     *   this list of conditions and the following disclaimer.
     * - Redistributions in binary form must reproduce the above copyright notice,
     *   this list of conditions and the following disclaimer in the documentation
     *   and/or other materials provided with the distribution.
     * - Neither the name of the libjpeg-turbo Project nor the names of its
     *   contributors may be used to endorse or promote products derived from this
     *   software without specific prior written permission.
     *
     * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS",
     * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
     * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
     * ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE
     * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
     * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
     * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
     * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
     * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
     * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
     * POSSIBILITY OF SUCH DAMAGE.
     */
    /* TurboJPEG/LJT:  this implements the TurboJPEG API using libjpeg or
    libjpeg-turbo */
    #[no_mangle]
    pub fn jpeg_mem_dest_tj(
        _: crate::jpeglib_h::j_compress_ptr,
        _: *mut *mut libc::c_uchar,
        _: *mut libc::c_ulong,
        _: crate::jmorecfg_h::boolean,
    );
    #[no_mangle]
    pub fn jpeg_mem_src_tj(
        _: crate::jpeglib_h::j_decompress_ptr,
        _: *const libc::c_uchar,
        _: libc::c_ulong,
    );
}

pub type tjinstance = _tjinstance;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _tjinstance {
    pub cinfo: crate::jpeglib_h::jpeg_compress_struct,
    pub dinfo: crate::jpeglib_h::jpeg_decompress_struct,
    pub jerr: my_error_mgr,
    pub init: libc::c_int,
    pub headerRead: libc::c_int,
    pub errStr: [libc::c_char; 200],
    pub isInstanceError: crate::jmorecfg_h::boolean,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_error_mgr {
    pub pub_0: crate::jpeglib_h::jpeg_error_mgr,
    pub setjmp_buffer: crate::stdlib::jmp_buf,
    pub emit_message:
        Option<unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr, _: libc::c_int) -> ()>,
    pub warning: crate::jmorecfg_h::boolean,
    pub stopOnWarning: crate::jmorecfg_h::boolean,
}

pub const COMPRESS: C2RustUnnamed_116 = 1;

pub type my_error_ptr = *mut my_error_mgr;

pub const DECOMPRESS: C2RustUnnamed_116 = 2;
/* Global structures, macros, etc. */

pub type C2RustUnnamed_116 = libc::c_uint;
/* Error handling (based on example in example.txt) */

static mut errStr: [libc::c_char; 200] = [
    78, 111, 32, 101, 114, 114, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

static mut turbojpeg_message_table: [*const libc::c_char; 29] = [
    0 as *const libc::c_char,
    b"Unsupported BMP colormap format\x00" as *const u8 as *const libc::c_char,
    b"Only 8- and 24-bit BMP files are supported\x00" as *const u8 as *const libc::c_char,
    b"Invalid BMP file: bad header length\x00" as *const u8 as *const libc::c_char,
    b"Invalid BMP file: biPlanes not equal to 1\x00" as *const u8 as *const libc::c_char,
    b"BMP output must be grayscale or RGB\x00" as *const u8 as *const libc::c_char,
    b"Sorry, compressed BMPs not yet supported\x00" as *const u8 as *const libc::c_char,
    b"Empty BMP image\x00" as *const u8 as *const libc::c_char,
    b"Not a BMP file - does not start with BM\x00" as *const u8 as *const libc::c_char,
    b"Numeric value out of range in BMP file\x00" as *const u8 as *const libc::c_char,
    b"%ux%u 24-bit BMP image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u 8-bit colormapped BMP image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u 24-bit OS2 BMP image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u 8-bit colormapped OS2 BMP image\x00" as *const u8 as *const libc::c_char,
    b"PPM output must be grayscale or RGB\x00" as *const u8 as *const libc::c_char,
    b"Nonnumeric data in PPM file\x00" as *const u8 as *const libc::c_char,
    b"Not a PPM/PGM file\x00" as *const u8 as *const libc::c_char,
    b"Numeric value out of range in PPM file\x00" as *const u8 as *const libc::c_char,
    b"%ux%u PGM image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u text PGM image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u PPM image\x00" as *const u8 as *const libc::c_char,
    b"%ux%u text PPM image\x00" as *const u8 as *const libc::c_char,
    b"Targa support was not compiled\x00" as *const u8 as *const libc::c_char,
    b"Color map file is invalid or of unsupported format\x00" as *const u8 as *const libc::c_char,
    b"Output file format cannot handle %d colormap entries\x00" as *const u8 as *const libc::c_char,
    b"ungetc failed\x00" as *const u8 as *const libc::c_char,
    b"MozJPEG can\'t read the image (PNG support is disabled in this build)\x00" as *const u8
        as *const libc::c_char,
    b"Unsupported output file format\x00" as *const u8 as *const libc::c_char,
    crate::stddef_h::NULL as *const libc::c_char,
];

unsafe extern "C" fn my_error_exit(mut cinfo: crate::jpeglib_h::j_common_ptr) {
    let mut myerr: my_error_ptr = (*cinfo).err as my_error_ptr;
    Some(
        (*(*cinfo).err)
            .output_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo);
    crate::stdlib::longjmp((*myerr).setjmp_buffer.as_mut_ptr(), 1 as libc::c_int);
}
/* Based on output_message() in jerror.c */

unsafe extern "C" fn my_output_message(mut cinfo: crate::jpeglib_h::j_common_ptr) {
    Some(
        (*(*cinfo).err)
            .format_message
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(cinfo, errStr.as_mut_ptr());
}

unsafe extern "C" fn my_emit_message(
    mut cinfo: crate::jpeglib_h::j_common_ptr,
    mut msg_level: libc::c_int,
) {
    let mut myerr: my_error_ptr = (*cinfo).err as my_error_ptr;
    (*myerr).emit_message.expect("non-null function pointer")(cinfo, msg_level);
    if msg_level < 0 as libc::c_int {
        (*myerr).warning = crate::jmorecfg_h::TRUE;
        if (*myerr).stopOnWarning != 0 {
            crate::stdlib::longjmp((*myerr).setjmp_buffer.as_mut_ptr(), 1 as libc::c_int);
        }
    };
}

static mut pixelsize: [libc::c_int; 6] = [
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    1 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
];

static mut xformtypes: [crate::src::transupp::JXFORM_CODE; 8] = [
    crate::src::transupp::JXFORM_NONE,
    crate::src::transupp::JXFORM_FLIP_H,
    crate::src::transupp::JXFORM_FLIP_V,
    crate::src::transupp::JXFORM_TRANSPOSE,
    crate::src::transupp::JXFORM_TRANSVERSE,
    crate::src::transupp::JXFORM_ROT_90,
    crate::src::transupp::JXFORM_ROT_180,
    crate::src::transupp::JXFORM_ROT_270,
];

pub const NUMSF: libc::c_int = 16 as libc::c_int;

static mut sf: [crate::src::turbojpeg::tjscalingfactor; 16] = [
    {
        let mut init = crate::src::turbojpeg::tjscalingfactor {
            num: 2 as libc::c_int,
            denom: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = crate::src::turbojpeg::tjscalingfactor {
            num: 15 as libc::c_int,
            denom: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = crate::src::turbojpeg::tjscalingfactor {
            num: 7 as libc::c_int,
            denom: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = crate::src::turbojpeg::tjscalingfactor {
            num: 13 as libc::c_int,
            denom: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = crate::src::turbojpeg::tjscalingfactor {
            num: 3 as libc::c_int,
            denom: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = crate::src::turbojpeg::tjscalingfactor {
            num: 11 as libc::c_int,
            denom: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = crate::src::turbojpeg::tjscalingfactor {
            num: 5 as libc::c_int,
            denom: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = crate::src::turbojpeg::tjscalingfactor {
            num: 9 as libc::c_int,
            denom: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = crate::src::turbojpeg::tjscalingfactor {
            num: 1 as libc::c_int,
            denom: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = crate::src::turbojpeg::tjscalingfactor {
            num: 7 as libc::c_int,
            denom: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = crate::src::turbojpeg::tjscalingfactor {
            num: 3 as libc::c_int,
            denom: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = crate::src::turbojpeg::tjscalingfactor {
            num: 5 as libc::c_int,
            denom: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = crate::src::turbojpeg::tjscalingfactor {
            num: 1 as libc::c_int,
            denom: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = crate::src::turbojpeg::tjscalingfactor {
            num: 3 as libc::c_int,
            denom: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = crate::src::turbojpeg::tjscalingfactor {
            num: 1 as libc::c_int,
            denom: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = crate::src::turbojpeg::tjscalingfactor {
            num: 1 as libc::c_int,
            denom: 8 as libc::c_int,
        };
        init
    },
];

static mut pf2cs: [crate::jpeglib_h::J_COLOR_SPACE; 12] = [
    crate::jpeglib_h::JCS_EXT_RGB,
    crate::jpeglib_h::JCS_EXT_BGR,
    crate::jpeglib_h::JCS_EXT_RGBX,
    crate::jpeglib_h::JCS_EXT_BGRX,
    crate::jpeglib_h::JCS_EXT_XBGR,
    crate::jpeglib_h::JCS_EXT_XRGB,
    crate::jpeglib_h::JCS_GRAYSCALE,
    crate::jpeglib_h::JCS_EXT_RGBA,
    crate::jpeglib_h::JCS_EXT_BGRA,
    crate::jpeglib_h::JCS_EXT_ABGR,
    crate::jpeglib_h::JCS_EXT_ARGB,
    crate::jpeglib_h::JCS_CMYK,
];

static mut cs2pf: [libc::c_int; 17] = [
    crate::src::turbojpeg::TJPF_UNKNOWN as libc::c_int,
    crate::src::turbojpeg::TJPF_GRAY as libc::c_int,
    crate::src::turbojpeg::TJPF_RGB as libc::c_int,
    crate::src::turbojpeg::TJPF_UNKNOWN as libc::c_int,
    crate::src::turbojpeg::TJPF_CMYK as libc::c_int,
    crate::src::turbojpeg::TJPF_UNKNOWN as libc::c_int,
    crate::src::turbojpeg::TJPF_RGB as libc::c_int,
    crate::src::turbojpeg::TJPF_RGBX as libc::c_int,
    crate::src::turbojpeg::TJPF_BGR as libc::c_int,
    crate::src::turbojpeg::TJPF_BGRX as libc::c_int,
    crate::src::turbojpeg::TJPF_XBGR as libc::c_int,
    crate::src::turbojpeg::TJPF_XRGB as libc::c_int,
    crate::src::turbojpeg::TJPF_RGBA as libc::c_int,
    crate::src::turbojpeg::TJPF_BGRA as libc::c_int,
    crate::src::turbojpeg::TJPF_ABGR as libc::c_int,
    crate::src::turbojpeg::TJPF_ARGB as libc::c_int,
    crate::src::turbojpeg::TJPF_UNKNOWN as libc::c_int,
];

unsafe extern "C" fn getPixelFormat(
    mut pixelSize: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    if pixelSize == 1 as libc::c_int {
        return crate::src::turbojpeg::TJPF_GRAY as libc::c_int;
    }
    if pixelSize == 3 as libc::c_int {
        if flags & crate::src::turbojpeg::TJ_BGR != 0 {
            return crate::src::turbojpeg::TJPF_BGR as libc::c_int;
        } else {
            return crate::src::turbojpeg::TJPF_RGB as libc::c_int;
        }
    }
    if pixelSize == 4 as libc::c_int {
        if flags & crate::src::turbojpeg::TJ_ALPHAFIRST != 0 {
            if flags & crate::src::turbojpeg::TJ_BGR != 0 {
                return crate::src::turbojpeg::TJPF_XBGR as libc::c_int;
            } else {
                return crate::src::turbojpeg::TJPF_XRGB as libc::c_int;
            }
        } else if flags & crate::src::turbojpeg::TJ_BGR != 0 {
            return crate::src::turbojpeg::TJPF_BGRX as libc::c_int;
        } else {
            return crate::src::turbojpeg::TJPF_RGBX as libc::c_int;
        }
    }
    return -(1 as libc::c_int);
}

unsafe extern "C" fn setCompDefaults(
    mut cinfo: *mut crate::jpeglib_h::jpeg_compress_struct,
    mut pixelFormat: libc::c_int,
    mut subsamp: libc::c_int,
    mut jpegQual: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut env: *mut libc::c_char = crate::stddef_h::NULL as *mut libc::c_char;
    (*cinfo).in_color_space = pf2cs[pixelFormat as usize];
    (*cinfo).input_components = tjPixelSize[pixelFormat as usize];
    env = crate::stdlib::getenv(b"TJ_REVERT\x00" as *const u8 as *const libc::c_char);
    if !env.is_null()
        && crate::stdlib::strlen(env) > 0 as libc::c_int as libc::c_ulong
        && crate::stdlib::strcmp(env, b"1\x00" as *const u8 as *const libc::c_char) == 0
    {
        (*(*cinfo).master).compress_profile = crate::jpeglib_h::JCP_FASTEST as libc::c_int
    }
    crate::src::jcparam::jpeg_set_defaults(cinfo);
    env = crate::stdlib::getenv(b"TJ_OPTIMIZE\x00" as *const u8 as *const libc::c_char);
    if !env.is_null()
        && crate::stdlib::strlen(env) > 0 as libc::c_int as libc::c_ulong
        && crate::stdlib::strcmp(env, b"1\x00" as *const u8 as *const libc::c_char) == 0
    {
        (*cinfo).optimize_coding = crate::jmorecfg_h::TRUE
    }
    env = crate::stdlib::getenv(b"TJ_ARITHMETIC\x00" as *const u8 as *const libc::c_char);
    if !env.is_null()
        && crate::stdlib::strlen(env) > 0 as libc::c_int as libc::c_ulong
        && crate::stdlib::strcmp(env, b"1\x00" as *const u8 as *const libc::c_char) == 0
    {
        (*cinfo).arith_code = crate::jmorecfg_h::TRUE
    }
    env = crate::stdlib::getenv(b"TJ_RESTART\x00" as *const u8 as *const libc::c_char);
    if !env.is_null() && crate::stdlib::strlen(env) > 0 as libc::c_int as libc::c_ulong {
        let mut temp: libc::c_int = -(1 as libc::c_int);
        let mut tempc: libc::c_char = 0 as libc::c_int as libc::c_char;
        if crate::stdlib::sscanf(
            env,
            b"%d%c\x00" as *const u8 as *const libc::c_char,
            &mut temp as *mut libc::c_int,
            &mut tempc as *mut libc::c_char,
        ) >= 1 as libc::c_int
            && temp >= 0 as libc::c_int
            && temp <= 65535 as libc::c_int
        {
            if ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = tempc as libc::c_int;
                        __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                            __c
                        } else {
                            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                        })
                    } else {
                        __res = toupper(tempc as libc::c_int)
                    }
                } else {
                    __res = *(*crate::stdlib::__ctype_toupper_loc())
                        .offset(tempc as libc::c_int as isize)
                }
                __res
            }) == 'B' as i32
            {
                (*cinfo).restart_interval = temp as libc::c_uint;
                (*cinfo).restart_in_rows = 0 as libc::c_int
            } else {
                (*cinfo).restart_in_rows = temp
            }
        }
    }
    if jpegQual >= 0 as libc::c_int {
        crate::src::jcparam::jpeg_set_quality(cinfo, jpegQual, crate::jmorecfg_h::TRUE);
        if jpegQual >= 96 as libc::c_int || flags & crate::src::turbojpeg::TJFLAG_ACCURATEDCT != 0 {
            (*cinfo).dct_method = crate::jpeglib_h::JDCT_ISLOW
        } else {
            (*cinfo).dct_method = crate::jpeglib_h::JDCT_FASTEST as crate::jpeglib_h::J_DCT_METHOD
        }
    }
    if subsamp == crate::src::turbojpeg::TJSAMP_GRAY as libc::c_int {
        crate::src::jcparam::jpeg_set_colorspace(cinfo, crate::jpeglib_h::JCS_GRAYSCALE);
    } else if pixelFormat == crate::src::turbojpeg::TJPF_CMYK as libc::c_int {
        crate::src::jcparam::jpeg_set_colorspace(cinfo, crate::jpeglib_h::JCS_YCCK);
    } else {
        crate::src::jcparam::jpeg_set_colorspace(cinfo, crate::jpeglib_h::JCS_YCbCr);
    }
    if flags & crate::src::turbojpeg::TJFLAG_PROGRESSIVE != 0 {
        crate::src::jcparam::jpeg_simple_progression(cinfo);
    } else {
        env = crate::stdlib::getenv(b"TJ_PROGRESSIVE\x00" as *const u8 as *const libc::c_char);
        if !env.is_null()
            && crate::stdlib::strlen(env) > 0 as libc::c_int as libc::c_ulong
            && crate::stdlib::strcmp(env, b"1\x00" as *const u8 as *const libc::c_char) == 0
        {
            crate::src::jcparam::jpeg_simple_progression(cinfo);
        }
    }
    /* Set scan pattern again as colorspace might have changed */
    if (*(*cinfo).master).compress_profile == crate::jpeglib_h::JCP_MAX_COMPRESSION as libc::c_int {
        crate::src::jcparam::jpeg_simple_progression(cinfo);
    }
    (*(*cinfo).comp_info.offset(0 as libc::c_int as isize)).h_samp_factor =
        tjMCUWidth[subsamp as usize] / 8 as libc::c_int;
    (*(*cinfo).comp_info.offset(1 as libc::c_int as isize)).h_samp_factor = 1 as libc::c_int;
    (*(*cinfo).comp_info.offset(2 as libc::c_int as isize)).h_samp_factor = 1 as libc::c_int;
    if (*cinfo).num_components > 3 as libc::c_int {
        (*(*cinfo).comp_info.offset(3 as libc::c_int as isize)).h_samp_factor =
            tjMCUWidth[subsamp as usize] / 8 as libc::c_int
    }
    (*(*cinfo).comp_info.offset(0 as libc::c_int as isize)).v_samp_factor =
        tjMCUHeight[subsamp as usize] / 8 as libc::c_int;
    (*(*cinfo).comp_info.offset(1 as libc::c_int as isize)).v_samp_factor = 1 as libc::c_int;
    (*(*cinfo).comp_info.offset(2 as libc::c_int as isize)).v_samp_factor = 1 as libc::c_int;
    if (*cinfo).num_components > 3 as libc::c_int {
        (*(*cinfo).comp_info.offset(3 as libc::c_int as isize)).v_samp_factor =
            tjMCUHeight[subsamp as usize] / 8 as libc::c_int
    }
    return retval;
}

unsafe extern "C" fn getSubsamp(mut dinfo: crate::jpeglib_h::j_decompress_ptr) -> libc::c_int {
    let mut retval: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    /* The sampling factors actually have no meaning with grayscale JPEG files,
    and in fact it's possible to generate grayscale JPEGs with sampling
    factors > 1 (even though those sampling factors are ignored by the
    decompressor.)  Thus, we need to treat grayscale as a special case. */
    if (*dinfo).num_components == 1 as libc::c_int
        && (*dinfo).jpeg_color_space as libc::c_uint
            == crate::jpeglib_h::JCS_GRAYSCALE as libc::c_int as libc::c_uint
    {
        return crate::src::turbojpeg::TJSAMP_GRAY as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < crate::src::turbojpeg::NUMSUBOPT {
        if (*dinfo).num_components == pixelsize[i as usize]
            || ((*dinfo).jpeg_color_space as libc::c_uint
                == crate::jpeglib_h::JCS_YCCK as libc::c_int as libc::c_uint
                || (*dinfo).jpeg_color_space as libc::c_uint
                    == crate::jpeglib_h::JCS_CMYK as libc::c_int as libc::c_uint)
                && pixelsize[i as usize] == 3 as libc::c_int
                && (*dinfo).num_components == 4 as libc::c_int
        {
            if (*(*dinfo).comp_info.offset(0 as libc::c_int as isize)).h_samp_factor
                == tjMCUWidth[i as usize] / 8 as libc::c_int
                && (*(*dinfo).comp_info.offset(0 as libc::c_int as isize)).v_samp_factor
                    == tjMCUHeight[i as usize] / 8 as libc::c_int
            {
                let mut match_0: libc::c_int = 0 as libc::c_int;
                k = 1 as libc::c_int;
                while k < (*dinfo).num_components {
                    let mut href: libc::c_int = 1 as libc::c_int;
                    let mut vref: libc::c_int = 1 as libc::c_int;
                    if ((*dinfo).jpeg_color_space as libc::c_uint
                        == crate::jpeglib_h::JCS_YCCK as libc::c_int as libc::c_uint
                        || (*dinfo).jpeg_color_space as libc::c_uint
                            == crate::jpeglib_h::JCS_CMYK as libc::c_int as libc::c_uint)
                        && k == 3 as libc::c_int
                    {
                        href = tjMCUWidth[i as usize] / 8 as libc::c_int;
                        vref = tjMCUHeight[i as usize] / 8 as libc::c_int
                    }
                    if (*(*dinfo).comp_info.offset(k as isize)).h_samp_factor == href
                        && (*(*dinfo).comp_info.offset(k as isize)).v_samp_factor == vref
                    {
                        match_0 += 1
                    }
                    k += 1
                }
                if match_0 == (*dinfo).num_components - 1 as libc::c_int {
                    retval = i;
                    break;
                }
            }
            /* Handle 4:2:2 and 4:4:0 images whose sampling factors are specified
            in non-standard ways. */
            if (*(*dinfo).comp_info.offset(0 as libc::c_int as isize)).h_samp_factor
                == 2 as libc::c_int
                && (*(*dinfo).comp_info.offset(0 as libc::c_int as isize)).v_samp_factor
                    == 2 as libc::c_int
                && (i == crate::src::turbojpeg::TJSAMP_422 as libc::c_int
                    || i == crate::src::turbojpeg::TJSAMP_440 as libc::c_int)
            {
                let mut match_1: libc::c_int = 0 as libc::c_int;
                k = 1 as libc::c_int;
                while k < (*dinfo).num_components {
                    let mut href_0: libc::c_int = tjMCUHeight[i as usize] / 8 as libc::c_int;
                    let mut vref_0: libc::c_int = tjMCUWidth[i as usize] / 8 as libc::c_int;
                    if ((*dinfo).jpeg_color_space as libc::c_uint
                        == crate::jpeglib_h::JCS_YCCK as libc::c_int as libc::c_uint
                        || (*dinfo).jpeg_color_space as libc::c_uint
                            == crate::jpeglib_h::JCS_CMYK as libc::c_int as libc::c_uint)
                        && k == 3 as libc::c_int
                    {
                        vref_0 = 2 as libc::c_int;
                        href_0 = vref_0
                    }
                    if (*(*dinfo).comp_info.offset(k as isize)).h_samp_factor == href_0
                        && (*(*dinfo).comp_info.offset(k as isize)).v_samp_factor == vref_0
                    {
                        match_1 += 1
                    }
                    k += 1
                }
                if match_1 == (*dinfo).num_components - 1 as libc::c_int {
                    retval = i;
                    break;
                }
            }
        }
        i += 1
    }
    return retval;
}
/* *
 * Returns a descriptive error message explaining why the last command failed.
 *
 * @param handle a handle to a TurboJPEG compressor, decompressor, or
 * transformer instance, or NULL if the error was generated by a global
 * function (but note that retrieving the error message for a global function
 * is not thread-safe.)
 *
 * @return a descriptive error message explaining why the last command failed.
 */
/* General API functions */
#[no_mangle]

pub unsafe extern "C" fn tjGetErrorStr2(
    mut handle: crate::src::turbojpeg::tjhandle,
) -> *mut libc::c_char {
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    if !this.is_null() && (*this).isInstanceError != 0 {
        (*this).isInstanceError = crate::jmorecfg_h::FALSE;
        return (*this).errStr.as_mut_ptr();
    } else {
        return errStr.as_mut_ptr();
    };
}
#[no_mangle]

pub unsafe extern "C" fn tjGetErrorStr() -> *mut libc::c_char {
    return errStr.as_mut_ptr();
}
/* *
 * Returns a code indicating the severity of the last error.  See
 * @ref TJERR "Error codes".
 *
 * @param handle a handle to a TurboJPEG compressor, decompressor or
 * transformer instance
 *
 * @return a code indicating the severity of the last error.  See
 * @ref TJERR "Error codes".
 */
#[no_mangle]

pub unsafe extern "C" fn tjGetErrorCode(
    mut handle: crate::src::turbojpeg::tjhandle,
) -> libc::c_int {
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    if !this.is_null() && (*this).jerr.warning != 0 {
        return crate::src::turbojpeg::TJERR_WARNING as libc::c_int;
    } else {
        return crate::src::turbojpeg::TJERR_FATAL as libc::c_int;
    };
}
/* *
 * Destroy a TurboJPEG compressor, decompressor, or transformer instance.
 *
 * @param handle a handle to a TurboJPEG compressor, decompressor or
 * transformer instance
 *
 * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2().)
 */
#[no_mangle]

pub unsafe extern "C" fn tjDestroy(mut handle: crate::src::turbojpeg::tjhandle) -> libc::c_int {
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    let mut cinfo: crate::jpeglib_h::j_compress_ptr =
        crate::stddef_h::NULL as crate::jpeglib_h::j_compress_ptr;
    let mut dinfo: crate::jpeglib_h::j_decompress_ptr =
        crate::stddef_h::NULL as crate::jpeglib_h::j_decompress_ptr;
    if this.is_null() {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"Invalid handle\x00" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    cinfo = &mut (*this).cinfo;
    dinfo = &mut (*this).dinfo;
    (*this).jerr.warning = crate::jmorecfg_h::FALSE;
    (*this).isInstanceError = crate::jmorecfg_h::FALSE;
    if crate::stdlib::_setjmp((*this).jerr.setjmp_buffer.as_mut_ptr()) != 0 {
        return -(1 as libc::c_int);
    }
    if (*this).init & COMPRESS as libc::c_int != 0 {
        crate::src::jcapimin::jpeg_destroy_compress(cinfo);
    }
    if (*this).init & DECOMPRESS as libc::c_int != 0 {
        crate::src::jdapimin::jpeg_destroy_decompress(dinfo);
    }
    crate::stdlib::free(this as *mut libc::c_void);
    return 0 as libc::c_int;
}
/* *
 * Free an image buffer previously allocated by TurboJPEG.  You should always
 * use this function to free JPEG destination buffer(s) that were automatically
 * (re)allocated by the compression and transform functions or that were
 * manually allocated using #tjAlloc().
 *
 * @param buffer address of the buffer to free
 *
 * @sa tjAlloc()
 */
/* These are exposed mainly because Windows can't malloc() and free() across
DLL boundaries except when the CRT DLL is used, and we don't use the CRT DLL
with turbojpeg.dll for compatibility reasons.  However, these functions
can potentially be used for other purposes by different implementations. */
#[no_mangle]

pub unsafe extern "C" fn tjFree(mut buf: *mut libc::c_uchar) {
    if !buf.is_null() {
        crate::stdlib::free(buf as *mut libc::c_void);
    };
}
/* *
 * Allocate an image buffer for use with TurboJPEG.  You should always use
 * this function to allocate the JPEG destination buffer(s) for the compression
 * and transform functions unless you are disabling automatic buffer
 * (re)allocation (by setting #TJFLAG_NOREALLOC.)
 *
 * @param bytes the number of bytes to allocate
 *
 * @return a pointer to a newly-allocated buffer with the specified number of
 * bytes.
 *
 * @sa tjFree()
 */
#[no_mangle]

pub unsafe extern "C" fn tjAlloc(mut bytes: libc::c_int) -> *mut libc::c_uchar {
    return crate::stdlib::malloc(bytes as libc::c_ulong) as *mut libc::c_uchar;
}
/* Compressor  */

unsafe extern "C" fn _tjInitCompress(mut this: *mut tjinstance) -> crate::src::turbojpeg::tjhandle {
    static mut buffer: [libc::c_uchar; 1] = [0; 1];
    let mut buf: *mut libc::c_uchar = buffer.as_mut_ptr();
    let mut size: libc::c_ulong = 1 as libc::c_int as libc::c_ulong;
    /* This is also straight out of example.txt */
    (*this).cinfo.err = crate::src::jerror::jpeg_std_error(&mut (*this).jerr.pub_0);
    (*this).jerr.pub_0.error_exit =
        Some(my_error_exit as unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ());
    (*this).jerr.pub_0.output_message =
        Some(my_output_message as unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ());
    (*this).jerr.emit_message = (*this).jerr.pub_0.emit_message;
    (*this).jerr.pub_0.emit_message = Some(
        my_emit_message
            as unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr, _: libc::c_int) -> (),
    );
    (*this).jerr.pub_0.addon_message_table = turbojpeg_message_table.as_mut_ptr();
    (*this).jerr.pub_0.first_addon_message = crate::cderror_h::JMSG_FIRSTADDONCODE as libc::c_int;
    (*this).jerr.pub_0.last_addon_message = crate::cderror_h_0::JMSG_LASTADDONCODE as libc::c_int;
    if crate::stdlib::_setjmp((*this).jerr.setjmp_buffer.as_mut_ptr()) != 0 {
        /* If we get here, the JPEG code has signaled an error. */
        if !this.is_null() {
            crate::stdlib::free(this as *mut libc::c_void);
        }
        return crate::stddef_h::NULL as *mut libc::c_void;
    }
    crate::src::jcapimin::jpeg_CreateCompress(
        &mut (*this).cinfo,
        crate::jconfig_h::JPEG_LIB_VERSION,
        ::std::mem::size_of::<crate::jpeglib_h::jpeg_compress_struct>() as libc::c_ulong,
    );
    /* Make an initial call so it will create the destination manager */
    jpeg_mem_dest_tj(&mut (*this).cinfo, &mut buf, &mut size, 0 as libc::c_int);
    (*this).init |= COMPRESS as libc::c_int;
    return this as crate::src::turbojpeg::tjhandle;
}
/* *
 * Pad the given width to the nearest 32-bit boundary
 */
/* *
 * Compute the scaled value of <tt>dimension</tt> using the given scaling
 * factor.  This macro performs the integer equivalent of <tt>ceil(dimension *
 * scalingFactor)</tt>.
 */
/* *
 * Create a TurboJPEG compressor instance.
 *
 * @return a handle to the newly-created instance, or NULL if an error
 * occurred (see #tjGetErrorStr2().)
 */
#[no_mangle]

pub unsafe extern "C" fn tjInitCompress() -> crate::src::turbojpeg::tjhandle {
    let mut this: *mut tjinstance = crate::stddef_h::NULL as *mut tjinstance;
    this = crate::stdlib::malloc(::std::mem::size_of::<tjinstance>() as libc::c_ulong)
        as *mut tjinstance;
    if this.is_null() {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"tjInitCompress(): Memory allocation failure\x00" as *const u8 as *const libc::c_char,
        );
        return crate::stddef_h::NULL as *mut libc::c_void;
    }
    crate::stdlib::memset(
        this as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<tjinstance>() as libc::c_ulong,
    );
    crate::stdlib::snprintf(
        (*this).errStr.as_mut_ptr(),
        crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
        b"No error\x00" as *const u8 as *const libc::c_char,
    );
    return _tjInitCompress(this);
}
/* *
 * The maximum size of the buffer (in bytes) required to hold a JPEG image with
 * the given parameters.  The number of bytes returned by this function is
 * larger than the size of the uncompressed source image.  The reason for this
 * is that the JPEG format uses 16-bit coefficients, and it is thus possible
 * for a very high-quality JPEG image with very high-frequency content to
 * expand rather than compress when converted to the JPEG format.  Such images
 * represent a very rare corner case, but since there is no way to predict the
 * size of a JPEG image prior to compression, the corner case has to be
 * handled.
 *
 * @param width width (in pixels) of the image
 *
 * @param height height (in pixels) of the image
 *
 * @param jpegSubsamp the level of chrominance subsampling to be used when
 * generating the JPEG image (see @ref TJSAMP
 * "Chrominance subsampling options".)
 *
 * @return the maximum size of the buffer (in bytes) required to hold the
 * image, or -1 if the arguments are out of bounds.
 */
#[no_mangle]

pub unsafe extern "C" fn tjBufSize(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut jpegSubsamp: libc::c_int,
) -> libc::c_ulong {
    let mut retval: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut mcuw: libc::c_int = 0;
    let mut mcuh: libc::c_int = 0;
    let mut chromasf: libc::c_int = 0;
    if width < 1 as libc::c_int
        || height < 1 as libc::c_int
        || jpegSubsamp < 0 as libc::c_int
        || jpegSubsamp >= crate::src::turbojpeg::NUMSUBOPT
    {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjBufSize(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int) as libc::c_ulong
    } else {
        /* This allows for rare corner cases in which a JPEG image can actually be
        larger than the uncompressed input (we wouldn't mention it if it hadn't
        happened before.) */
        mcuw = tjMCUWidth[jpegSubsamp as usize];
        mcuh = tjMCUHeight[jpegSubsamp as usize];
        chromasf = if jpegSubsamp == crate::src::turbojpeg::TJSAMP_GRAY as libc::c_int {
            0 as libc::c_int
        } else {
            (4 as libc::c_int * 64 as libc::c_int) / (mcuw * mcuh)
        };
        retval = ((width + mcuw - 1 as libc::c_int & !(mcuw - 1 as libc::c_int))
            * (height + mcuh - 1 as libc::c_int & !(mcuh - 1 as libc::c_int))
            * (2 as libc::c_int + chromasf)
            + 2048 as libc::c_int) as libc::c_ulong
    }
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn TJBUFSIZE(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> libc::c_ulong {
    let mut retval: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    if width < 1 as libc::c_int || height < 1 as libc::c_int {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"TJBUFSIZE(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int) as libc::c_ulong
    } else {
        /* This allows for rare corner cases in which a JPEG image can actually be
        larger than the uncompressed input (we wouldn't mention it if it hadn't
        happened before.) */
        retval = ((width + 16 as libc::c_int - 1 as libc::c_int
            & !(16 as libc::c_int - 1 as libc::c_int))
            * (height + 16 as libc::c_int - 1 as libc::c_int
                & !(16 as libc::c_int - 1 as libc::c_int))
            * 6 as libc::c_int
            + 2048 as libc::c_int) as libc::c_ulong
    }
    return retval;
}
/* *
 * The size of the buffer (in bytes) required to hold a YUV planar image with
 * the given parameters.
 *
 * @param width width (in pixels) of the image
 *
 * @param pad the width of each line in each plane of the image is padded to
 * the nearest multiple of this number of bytes (must be a power of 2.)
 *
 * @param height height (in pixels) of the image
 *
 * @param subsamp level of chrominance subsampling in the image (see
 * @ref TJSAMP "Chrominance subsampling options".)
 *
 * @return the size of the buffer (in bytes) required to hold the image, or
 * -1 if the arguments are out of bounds.
 */
#[no_mangle]

pub unsafe extern "C" fn tjBufSizeYUV2(
    mut width: libc::c_int,
    mut pad: libc::c_int,
    mut height: libc::c_int,
    mut subsamp: libc::c_int,
) -> libc::c_ulong {
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut nc: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if subsamp < 0 as libc::c_int || subsamp >= crate::src::turbojpeg::NUMSUBOPT {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjBufSizeYUV2(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else {
        nc = if subsamp == crate::src::turbojpeg::TJSAMP_GRAY as libc::c_int {
            1 as libc::c_int
        } else {
            3 as libc::c_int
        };
        i = 0 as libc::c_int;
        while i < nc {
            let mut pw: libc::c_int = tjPlaneWidth(i, width, subsamp);
            let mut stride: libc::c_int = pw + pad - 1 as libc::c_int & !(pad - 1 as libc::c_int);
            let mut ph: libc::c_int = tjPlaneHeight(i, height, subsamp);
            if pw < 0 as libc::c_int || ph < 0 as libc::c_int {
                return -(1 as libc::c_int) as libc::c_ulong;
            } else {
                retval += stride * ph
            }
            i += 1
        }
    }
    return retval as libc::c_ulong;
}
#[no_mangle]

pub unsafe extern "C" fn tjBufSizeYUV(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut subsamp: libc::c_int,
) -> libc::c_ulong {
    return tjBufSizeYUV2(width, 4 as libc::c_int, height, subsamp);
}
#[no_mangle]

pub unsafe extern "C" fn TJBUFSIZEYUV(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut subsamp: libc::c_int,
) -> libc::c_ulong {
    return tjBufSizeYUV(width, height, subsamp);
}
/* *
 * The plane width of a YUV image plane with the given parameters.  Refer to
 * @ref YUVnotes "YUV Image Format Notes" for a description of plane width.
 *
 * @param componentID ID number of the image plane (0 = Y, 1 = U/Cb, 2 = V/Cr)
 *
 * @param width width (in pixels) of the YUV image
 *
 * @param subsamp level of chrominance subsampling in the image (see
 * @ref TJSAMP "Chrominance subsampling options".)
 *
 * @return the plane width of a YUV image plane with the given parameters, or
 * -1 if the arguments are out of bounds.
 */
#[no_mangle]

pub unsafe extern "C" fn tjPlaneWidth(
    mut componentID: libc::c_int,
    mut width: libc::c_int,
    mut subsamp: libc::c_int,
) -> libc::c_int {
    let mut pw: libc::c_int = 0;
    let mut nc: libc::c_int = 0;
    let mut retval: libc::c_int = 0 as libc::c_int;
    if width < 1 as libc::c_int
        || subsamp < 0 as libc::c_int
        || subsamp >= crate::src::turbojpeg::TJ_NUMSAMP
    {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjPlaneWidth(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else {
        nc = if subsamp == crate::src::turbojpeg::TJSAMP_GRAY as libc::c_int {
            1 as libc::c_int
        } else {
            3 as libc::c_int
        };
        if componentID < 0 as libc::c_int || componentID >= nc {
            crate::stdlib::snprintf(
                errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                b"tjPlaneWidth(): Invalid argument\x00" as *const u8 as *const libc::c_char,
            );
            retval = -(1 as libc::c_int)
        } else {
            pw = width + tjMCUWidth[subsamp as usize] / 8 as libc::c_int - 1 as libc::c_int
                & !(tjMCUWidth[subsamp as usize] / 8 as libc::c_int - 1 as libc::c_int);
            if componentID == 0 as libc::c_int {
                retval = pw
            } else {
                retval = pw * 8 as libc::c_int / tjMCUWidth[subsamp as usize]
            }
        }
    }
    return retval;
}
/* *
 * The plane height of a YUV image plane with the given parameters.  Refer to
 * @ref YUVnotes "YUV Image Format Notes" for a description of plane height.
 *
 * @param componentID ID number of the image plane (0 = Y, 1 = U/Cb, 2 = V/Cr)
 *
 * @param height height (in pixels) of the YUV image
 *
 * @param subsamp level of chrominance subsampling in the image (see
 * @ref TJSAMP "Chrominance subsampling options".)
 *
 * @return the plane height of a YUV image plane with the given parameters, or
 * -1 if the arguments are out of bounds.
 */
#[no_mangle]

pub unsafe extern "C" fn tjPlaneHeight(
    mut componentID: libc::c_int,
    mut height: libc::c_int,
    mut subsamp: libc::c_int,
) -> libc::c_int {
    let mut ph: libc::c_int = 0;
    let mut nc: libc::c_int = 0;
    let mut retval: libc::c_int = 0 as libc::c_int;
    if height < 1 as libc::c_int
        || subsamp < 0 as libc::c_int
        || subsamp >= crate::src::turbojpeg::TJ_NUMSAMP
    {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjPlaneHeight(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else {
        nc = if subsamp == crate::src::turbojpeg::TJSAMP_GRAY as libc::c_int {
            1 as libc::c_int
        } else {
            3 as libc::c_int
        };
        if componentID < 0 as libc::c_int || componentID >= nc {
            crate::stdlib::snprintf(
                errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                b"tjPlaneHeight(): Invalid argument\x00" as *const u8 as *const libc::c_char,
            );
            retval = -(1 as libc::c_int)
        } else {
            ph = height + tjMCUHeight[subsamp as usize] / 8 as libc::c_int - 1 as libc::c_int
                & !(tjMCUHeight[subsamp as usize] / 8 as libc::c_int - 1 as libc::c_int);
            if componentID == 0 as libc::c_int {
                retval = ph
            } else {
                retval = ph * 8 as libc::c_int / tjMCUHeight[subsamp as usize]
            }
        }
    }
    return retval;
}
/* *
 * The size of the buffer (in bytes) required to hold a YUV image plane with
 * the given parameters.
 *
 * @param componentID ID number of the image plane (0 = Y, 1 = U/Cb, 2 = V/Cr)
 *
 * @param width width (in pixels) of the YUV image.  NOTE: this is the width of
 * the whole image, not the plane width.
 *
 * @param stride bytes per line in the image plane.  Setting this to 0 is the
 * equivalent of setting it to the plane width.
 *
 * @param height height (in pixels) of the YUV image.  NOTE: this is the height
 * of the whole image, not the plane height.
 *
 * @param subsamp level of chrominance subsampling in the image (see
 * @ref TJSAMP "Chrominance subsampling options".)
 *
 * @return the size of the buffer (in bytes) required to hold the YUV image
 * plane, or -1 if the arguments are out of bounds.
 */
#[no_mangle]

pub unsafe extern "C" fn tjPlaneSizeYUV(
    mut componentID: libc::c_int,
    mut width: libc::c_int,
    mut stride: libc::c_int,
    mut height: libc::c_int,
    mut subsamp: libc::c_int,
) -> libc::c_ulong {
    let mut retval: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut pw: libc::c_int = 0;
    let mut ph: libc::c_int = 0;
    if width < 1 as libc::c_int
        || height < 1 as libc::c_int
        || subsamp < 0 as libc::c_int
        || subsamp >= crate::src::turbojpeg::NUMSUBOPT
    {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjPlaneSizeYUV(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int) as libc::c_ulong
    } else {
        pw = tjPlaneWidth(componentID, width, subsamp);
        ph = tjPlaneHeight(componentID, height, subsamp);
        if pw < 0 as libc::c_int || ph < 0 as libc::c_int {
            return -(1 as libc::c_int) as libc::c_ulong;
        }
        if stride == 0 as libc::c_int {
            stride = pw
        } else {
            stride = crate::stdlib::abs(stride)
        }
        retval = (stride * (ph - 1 as libc::c_int) + pw) as libc::c_ulong
    }
    return retval;
}
/* *
 * Compress an RGB, grayscale, or CMYK image into a JPEG image.
 *
 * @param handle a handle to a TurboJPEG compressor or transformer instance
 *
 * @param srcBuf pointer to an image buffer containing RGB, grayscale, or
 * CMYK pixels to be compressed
 *
 * @param width width (in pixels) of the source image
 *
 * @param pitch bytes per line in the source image.  Normally, this should be
 * <tt>width * #tjPixelSize[pixelFormat]</tt> if the image is unpadded, or
 * <tt>#TJPAD(width * #tjPixelSize[pixelFormat])</tt> if each line of the image
 * is padded to the nearest 32-bit boundary, as is the case for Windows
 * bitmaps.  You can also be clever and use this parameter to skip lines, etc.
 * Setting this parameter to 0 is the equivalent of setting it to
 * <tt>width * #tjPixelSize[pixelFormat]</tt>.
 *
 * @param height height (in pixels) of the source image
 *
 * @param pixelFormat pixel format of the source image (see @ref TJPF
 * "Pixel formats".)
 *
 * @param jpegBuf address of a pointer to an image buffer that will receive the
 * JPEG image.  TurboJPEG has the ability to reallocate the JPEG buffer
 * to accommodate the size of the JPEG image.  Thus, you can choose to:
 * -# pre-allocate the JPEG buffer with an arbitrary size using #tjAlloc() and
 * let TurboJPEG grow the buffer as needed,
 * -# set <tt>*jpegBuf</tt> to NULL to tell TurboJPEG to allocate the buffer
 * for you, or
 * -# pre-allocate the buffer to a "worst case" size determined by calling
 * #tjBufSize().  This should ensure that the buffer never has to be
 * re-allocated (setting #TJFLAG_NOREALLOC guarantees that it won't be.)
 * .
 * If you choose option 1, <tt>*jpegSize</tt> should be set to the size of your
 * pre-allocated buffer.  In any case, unless you have set #TJFLAG_NOREALLOC,
 * you should always check <tt>*jpegBuf</tt> upon return from this function, as
 * it may have changed.
 *
 * @param jpegSize pointer to an unsigned long variable that holds the size of
 * the JPEG image buffer.  If <tt>*jpegBuf</tt> points to a pre-allocated
 * buffer, then <tt>*jpegSize</tt> should be set to the size of the buffer.
 * Upon return, <tt>*jpegSize</tt> will contain the size of the JPEG image (in
 * bytes.)  If <tt>*jpegBuf</tt> points to a JPEG image buffer that is being
 * reused from a previous call to one of the JPEG compression functions, then
 * <tt>*jpegSize</tt> is ignored.
 *
 * @param jpegSubsamp the level of chrominance subsampling to be used when
 * generating the JPEG image (see @ref TJSAMP
 * "Chrominance subsampling options".)
 *
 * @param jpegQual the image quality of the generated JPEG image (1 = worst,
 * 100 = best)
 *
 * @param flags the bitwise OR of one or more of the @ref TJFLAG_ACCURATEDCT
 * "flags"
 *
 * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2()
 * and #tjGetErrorCode().)
*/
#[no_mangle]

pub unsafe extern "C" fn tjCompress2(
    mut handle: crate::src::turbojpeg::tjhandle,
    mut srcBuf: *const libc::c_uchar,
    mut width: libc::c_int,
    mut pitch: libc::c_int,
    mut height: libc::c_int,
    mut pixelFormat: libc::c_int,
    mut jpegBuf: *mut *mut libc::c_uchar,
    mut jpegSize: *mut libc::c_ulong,
    mut jpegSubsamp: libc::c_int,
    mut jpegQual: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut alloc: libc::c_int = 1 as libc::c_int;
    let mut row_pointer: *mut crate::jpeglib_h::JSAMPROW =
        crate::stddef_h::NULL as *mut crate::jpeglib_h::JSAMPROW;
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    let mut cinfo: crate::jpeglib_h::j_compress_ptr =
        crate::stddef_h::NULL as crate::jpeglib_h::j_compress_ptr;
    if this.is_null() {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"Invalid handle\x00" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    cinfo = &mut (*this).cinfo;
    (*this).jerr.warning = crate::jmorecfg_h::FALSE;
    (*this).isInstanceError = crate::jmorecfg_h::FALSE;
    (*this).jerr.stopOnWarning = if flags & crate::src::turbojpeg::TJFLAG_STOPONWARNING != 0 {
        crate::jmorecfg_h::TRUE
    } else {
        crate::jmorecfg_h::FALSE
    };
    if (*this).init & COMPRESS as libc::c_int == 0 as libc::c_int {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjCompress2(): Instance has not been initialized for compression\x00" as *const u8
                as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjCompress2(): Instance has not been initialized for compression\x00" as *const u8
                as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else if srcBuf.is_null()
        || width <= 0 as libc::c_int
        || pitch < 0 as libc::c_int
        || height <= 0 as libc::c_int
        || pixelFormat < 0 as libc::c_int
        || pixelFormat >= crate::src::turbojpeg::TJ_NUMPF
        || jpegBuf.is_null()
        || jpegSize.is_null()
        || jpegSubsamp < 0 as libc::c_int
        || jpegSubsamp >= crate::src::turbojpeg::NUMSUBOPT
        || jpegQual < 0 as libc::c_int
        || jpegQual > 100 as libc::c_int
    {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjCompress2(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjCompress2(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else {
        if pitch == 0 as libc::c_int {
            pitch = width * tjPixelSize[pixelFormat as usize]
        }
        row_pointer = crate::stdlib::malloc(
            (::std::mem::size_of::<crate::jpeglib_h::JSAMPROW>() as libc::c_ulong)
                .wrapping_mul(height as libc::c_ulong),
        ) as *mut crate::jpeglib_h::JSAMPROW;
        if row_pointer.is_null() {
            crate::stdlib::snprintf(
                (*this).errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                b"tjCompress2(): Memory allocation failure\x00" as *const u8 as *const libc::c_char,
            );
            (*this).isInstanceError = crate::jmorecfg_h::TRUE;
            crate::stdlib::snprintf(
                errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                b"tjCompress2(): Memory allocation failure\x00" as *const u8 as *const libc::c_char,
            );
            retval = -(1 as libc::c_int)
        } else if crate::stdlib::_setjmp((*this).jerr.setjmp_buffer.as_mut_ptr()) != 0 {
            /* If we get here, the JPEG code has signaled an error. */
            retval = -(1 as libc::c_int)
        } else {
            (*cinfo).image_width = width as crate::jmorecfg_h::JDIMENSION;
            (*cinfo).image_height = height as crate::jmorecfg_h::JDIMENSION;
            if flags & crate::src::turbojpeg::TJFLAG_FORCEMMX != 0 {
                crate::stdlib::putenv(
                    b"JSIMD_FORCEMMX=1\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            } else if flags & crate::src::turbojpeg::TJFLAG_FORCESSE != 0 {
                crate::stdlib::putenv(
                    b"JSIMD_FORCESSE=1\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            } else if flags & crate::src::turbojpeg::TJFLAG_FORCESSE2 != 0 {
                crate::stdlib::putenv(
                    b"JSIMD_FORCESSE2=1\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            if flags & crate::src::turbojpeg::TJFLAG_NOREALLOC != 0 {
                alloc = 0 as libc::c_int;
                *jpegSize = tjBufSize(width, height, jpegSubsamp)
            }
            jpeg_mem_dest_tj(cinfo, jpegBuf, jpegSize, alloc);
            if setCompDefaults(cinfo, pixelFormat, jpegSubsamp, jpegQual, flags)
                == -(1 as libc::c_int)
            {
                return -(1 as libc::c_int);
            }
            crate::src::jcapistd::jpeg_start_compress(cinfo, crate::jmorecfg_h::TRUE);
            i = 0 as libc::c_int;
            while i < height {
                if flags & crate::src::turbojpeg::TJFLAG_BOTTOMUP != 0 {
                    let ref mut fresh0 = *row_pointer.offset(i as isize);
                    *fresh0 = &*srcBuf.offset(((height - i - 1 as libc::c_int) * pitch) as isize)
                        as *const libc::c_uchar
                        as crate::jpeglib_h::JSAMPROW
                } else {
                    let ref mut fresh1 = *row_pointer.offset(i as isize);
                    *fresh1 = &*srcBuf.offset((i * pitch) as isize) as *const libc::c_uchar
                        as crate::jpeglib_h::JSAMPROW
                }
                i += 1
            }
            while (*cinfo).next_scanline < (*cinfo).image_height {
                crate::src::jcapistd::jpeg_write_scanlines(
                    cinfo,
                    &mut *row_pointer.offset((*cinfo).next_scanline as isize),
                    (*cinfo).image_height.wrapping_sub((*cinfo).next_scanline),
                );
            }
            crate::src::jcapimin::jpeg_finish_compress(cinfo);
        }
    }
    if (*cinfo).global_state > crate::jpegint_h::CSTATE_START {
        crate::src::jcapimin::jpeg_abort_compress(cinfo);
    }
    if !row_pointer.is_null() {
        crate::stdlib::free(row_pointer as *mut libc::c_void);
    }
    if (*this).jerr.warning != 0 {
        retval = -(1 as libc::c_int)
    }
    (*this).jerr.stopOnWarning = crate::jmorecfg_h::FALSE;
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn tjCompress(
    mut handle: crate::src::turbojpeg::tjhandle,
    mut srcBuf: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut pitch: libc::c_int,
    mut height: libc::c_int,
    mut pixelSize: libc::c_int,
    mut jpegBuf: *mut libc::c_uchar,
    mut jpegSize: *mut libc::c_ulong,
    mut jpegSubsamp: libc::c_int,
    mut jpegQual: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut size: libc::c_ulong = 0;
    if flags & crate::src::turbojpeg::TJ_YUV != 0 {
        size = tjBufSizeYUV(width, height, jpegSubsamp);
        retval = tjEncodeYUV2(
            handle,
            srcBuf,
            width,
            pitch,
            height,
            getPixelFormat(pixelSize, flags),
            jpegBuf,
            jpegSubsamp,
            flags,
        )
    } else {
        retval = tjCompress2(
            handle,
            srcBuf,
            width,
            pitch,
            height,
            getPixelFormat(pixelSize, flags),
            &mut jpegBuf,
            &mut size,
            jpegSubsamp,
            jpegQual,
            flags | crate::src::turbojpeg::TJFLAG_NOREALLOC,
        )
    }
    *jpegSize = size;
    return retval;
}
/* *
 * Encode an RGB or grayscale image into separate Y, U (Cb), and V (Cr) image
 * planes.  This function uses the accelerated color conversion routines in the
 * underlying codec but does not execute any of the other steps in the JPEG
 * compression process.
 *
 * @param handle a handle to a TurboJPEG compressor or transformer instance
 *
 * @param srcBuf pointer to an image buffer containing RGB or grayscale pixels
 * to be encoded
 *
 * @param width width (in pixels) of the source image
 *
 * @param pitch bytes per line in the source image.  Normally, this should be
 * <tt>width * #tjPixelSize[pixelFormat]</tt> if the image is unpadded, or
 * <tt>#TJPAD(width * #tjPixelSize[pixelFormat])</tt> if each line of the image
 * is padded to the nearest 32-bit boundary, as is the case for Windows
 * bitmaps.  You can also be clever and use this parameter to skip lines, etc.
 * Setting this parameter to 0 is the equivalent of setting it to
 * <tt>width * #tjPixelSize[pixelFormat]</tt>.
 *
 * @param height height (in pixels) of the source image
 *
 * @param pixelFormat pixel format of the source image (see @ref TJPF
 * "Pixel formats".)
 *
 * @param dstPlanes an array of pointers to Y, U (Cb), and V (Cr) image planes
 * (or just a Y plane, if generating a grayscale image) that will receive the
 * encoded image.  These planes can be contiguous or non-contiguous in memory.
 * Use #tjPlaneSizeYUV() to determine the appropriate size for each plane based
 * on the image width, height, strides, and level of chrominance subsampling.
 * Refer to @ref YUVnotes "YUV Image Format Notes" for more details.
 *
 * @param strides an array of integers, each specifying the number of bytes per
 * line in the corresponding plane of the output image.  Setting the stride for
 * any plane to 0 is the same as setting it to the plane width (see
 * @ref YUVnotes "YUV Image Format Notes".)  If <tt>strides</tt> is NULL, then
 * the strides for all planes will be set to their respective plane widths.
 * You can adjust the strides in order to add an arbitrary amount of line
 * padding to each plane or to encode an RGB or grayscale image into a
 * subregion of a larger YUV planar image.
 *
 * @param subsamp the level of chrominance subsampling to be used when
 * generating the YUV image (see @ref TJSAMP
 * "Chrominance subsampling options".)  To generate images suitable for X
 * Video, <tt>subsamp</tt> should be set to @ref TJSAMP_420.  This produces an
 * image compatible with the I420 (AKA "YUV420P") format.
 *
 * @param flags the bitwise OR of one or more of the @ref TJFLAG_ACCURATEDCT
 * "flags"
 *
 * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2()
 * and #tjGetErrorCode().)
*/
#[no_mangle]

pub unsafe extern "C" fn tjEncodeYUVPlanes(
    mut handle: crate::src::turbojpeg::tjhandle,
    mut srcBuf: *const libc::c_uchar,
    mut width: libc::c_int,
    mut pitch: libc::c_int,
    mut height: libc::c_int,
    mut pixelFormat: libc::c_int,
    mut dstPlanes: *mut *mut libc::c_uchar,
    mut strides: *mut libc::c_int,
    mut subsamp: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut row_pointer: *mut crate::jpeglib_h::JSAMPROW =
        crate::stddef_h::NULL as *mut crate::jpeglib_h::JSAMPROW;
    let mut _tmpbuf: [*mut crate::jmorecfg_h::JSAMPLE; 10] =
        [0 as *mut crate::jmorecfg_h::JSAMPLE; 10];
    let mut _tmpbuf2: [*mut crate::jmorecfg_h::JSAMPLE; 10] =
        [0 as *mut crate::jmorecfg_h::JSAMPLE; 10];
    let mut tmpbuf: [*mut crate::jpeglib_h::JSAMPROW; 10] =
        [0 as *mut crate::jpeglib_h::JSAMPROW; 10];
    let mut tmpbuf2: [*mut crate::jpeglib_h::JSAMPROW; 10] =
        [0 as *mut crate::jpeglib_h::JSAMPROW; 10];
    let mut outbuf: [*mut crate::jpeglib_h::JSAMPROW; 10] =
        [0 as *mut crate::jpeglib_h::JSAMPROW; 10];
    let mut i: libc::c_int = 0;
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut row: libc::c_int = 0;
    let mut pw0: libc::c_int = 0;
    let mut ph0: libc::c_int = 0;
    let mut pw: [libc::c_int; 10] = [0; 10];
    let mut ph: [libc::c_int; 10] = [0; 10];
    let mut ptr: *mut crate::jmorecfg_h::JSAMPLE = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    let mut cinfo: crate::jpeglib_h::j_compress_ptr =
        crate::stddef_h::NULL as crate::jpeglib_h::j_compress_ptr;
    if this.is_null() {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"Invalid handle\x00" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    cinfo = &mut (*this).cinfo;
    (*this).jerr.warning = crate::jmorecfg_h::FALSE;
    (*this).isInstanceError = crate::jmorecfg_h::FALSE;
    (*this).jerr.stopOnWarning = if flags & crate::src::turbojpeg::TJFLAG_STOPONWARNING != 0 {
        crate::jmorecfg_h::TRUE
    } else {
        crate::jmorecfg_h::FALSE
    };
    i = 0 as libc::c_int;
    while i < crate::jmorecfg_h::MAX_COMPONENTS {
        tmpbuf[i as usize] = crate::stddef_h::NULL as *mut crate::jpeglib_h::JSAMPROW;
        _tmpbuf[i as usize] = crate::stddef_h::NULL as *mut crate::jmorecfg_h::JSAMPLE;
        tmpbuf2[i as usize] = crate::stddef_h::NULL as *mut crate::jpeglib_h::JSAMPROW;
        _tmpbuf2[i as usize] = crate::stddef_h::NULL as *mut crate::jmorecfg_h::JSAMPLE;
        outbuf[i as usize] = crate::stddef_h::NULL as *mut crate::jpeglib_h::JSAMPROW;
        i += 1
    }
    if (*this).init & COMPRESS as libc::c_int == 0 as libc::c_int {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjEncodeYUVPlanes(): Instance has not been initialized for compression\x00"
                as *const u8 as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjEncodeYUVPlanes(): Instance has not been initialized for compression\x00"
                as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else if srcBuf.is_null()
        || width <= 0 as libc::c_int
        || pitch < 0 as libc::c_int
        || height <= 0 as libc::c_int
        || pixelFormat < 0 as libc::c_int
        || pixelFormat >= crate::src::turbojpeg::TJ_NUMPF
        || dstPlanes.is_null()
        || (*dstPlanes.offset(0 as libc::c_int as isize)).is_null()
        || subsamp < 0 as libc::c_int
        || subsamp >= crate::src::turbojpeg::NUMSUBOPT
    {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjEncodeYUVPlanes(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjEncodeYUVPlanes(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else if subsamp != crate::src::turbojpeg::TJSAMP_GRAY as libc::c_int
        && ((*dstPlanes.offset(1 as libc::c_int as isize)).is_null()
            || (*dstPlanes.offset(2 as libc::c_int as isize)).is_null())
    {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjEncodeYUVPlanes(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjEncodeYUVPlanes(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else if pixelFormat == crate::src::turbojpeg::TJPF_CMYK as libc::c_int {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjEncodeYUVPlanes(): Cannot generate YUV images from CMYK pixels\x00" as *const u8
                as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjEncodeYUVPlanes(): Cannot generate YUV images from CMYK pixels\x00" as *const u8
                as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else {
        if pitch == 0 as libc::c_int {
            pitch = width * tjPixelSize[pixelFormat as usize]
        }
        if crate::stdlib::_setjmp((*this).jerr.setjmp_buffer.as_mut_ptr()) != 0 {
            /* If we get here, the JPEG code has signaled an error. */
            retval = -(1 as libc::c_int)
        } else {
            (*cinfo).image_width = width as crate::jmorecfg_h::JDIMENSION;
            (*cinfo).image_height = height as crate::jmorecfg_h::JDIMENSION;
            if flags & crate::src::turbojpeg::TJFLAG_FORCEMMX != 0 {
                crate::stdlib::putenv(
                    b"JSIMD_FORCEMMX=1\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            } else if flags & crate::src::turbojpeg::TJFLAG_FORCESSE != 0 {
                crate::stdlib::putenv(
                    b"JSIMD_FORCESSE=1\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            } else if flags & crate::src::turbojpeg::TJFLAG_FORCESSE2 != 0 {
                crate::stdlib::putenv(
                    b"JSIMD_FORCESSE2=1\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            if setCompDefaults(cinfo, pixelFormat, subsamp, -(1 as libc::c_int), flags)
                == -(1 as libc::c_int)
            {
                return -(1 as libc::c_int);
            }
            /* Execute only the parts of jpeg_start_compress() that we need.  If we
            were to call the whole jpeg_start_compress() function, then it would try
            to write the file headers, which could overflow the output buffer if the
            YUV image were very small. */
            if (*cinfo).global_state != crate::jpegint_h::CSTATE_START {
                crate::stdlib::snprintf(
                    (*this).errStr.as_mut_ptr(),
                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    b"tjEncodeYUVPlanes(): libjpeg API is in the wrong state\x00" as *const u8
                        as *const libc::c_char,
                );
                (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                crate::stdlib::snprintf(
                    errStr.as_mut_ptr(),
                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    b"tjEncodeYUVPlanes(): libjpeg API is in the wrong state\x00" as *const u8
                        as *const libc::c_char,
                );
                retval = -(1 as libc::c_int)
            } else {
                Some(
                    (*(*cinfo).err)
                        .reset_error_mgr
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    cinfo as crate::jpeglib_h::j_common_ptr
                );
                crate::src::jcmaster::jinit_c_master_control(cinfo, crate::jmorecfg_h::FALSE);
                crate::src::jccolor::jinit_color_converter(cinfo);
                crate::src::jcsample::jinit_downsampler(cinfo);
                Some(
                    (*(*cinfo).cconvert)
                        .start_pass
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(cinfo);
                pw0 = width + (*cinfo).max_h_samp_factor - 1 as libc::c_int
                    & !((*cinfo).max_h_samp_factor - 1 as libc::c_int);
                ph0 = height + (*cinfo).max_v_samp_factor - 1 as libc::c_int
                    & !((*cinfo).max_v_samp_factor - 1 as libc::c_int);
                row_pointer = crate::stdlib::malloc(
                    (::std::mem::size_of::<crate::jpeglib_h::JSAMPROW>() as libc::c_ulong)
                        .wrapping_mul(ph0 as libc::c_ulong),
                ) as *mut crate::jpeglib_h::JSAMPROW;
                if row_pointer.is_null() {
                    crate::stdlib::snprintf(
                        (*this).errStr.as_mut_ptr(),
                        crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        b"tjEncodeYUVPlanes(): Memory allocation failure\x00" as *const u8
                            as *const libc::c_char,
                    );
                    (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                    crate::stdlib::snprintf(
                        errStr.as_mut_ptr(),
                        crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        b"tjEncodeYUVPlanes(): Memory allocation failure\x00" as *const u8
                            as *const libc::c_char,
                    );
                    retval = -(1 as libc::c_int)
                } else {
                    i = 0 as libc::c_int;
                    while i < height {
                        if flags & crate::src::turbojpeg::TJFLAG_BOTTOMUP != 0 {
                            let ref mut fresh2 = *row_pointer.offset(i as isize);
                            *fresh2 = &*srcBuf
                                .offset(((height - i - 1 as libc::c_int) * pitch) as isize)
                                as *const libc::c_uchar
                                as crate::jpeglib_h::JSAMPROW
                        } else {
                            let ref mut fresh3 = *row_pointer.offset(i as isize);
                            *fresh3 = &*srcBuf.offset((i * pitch) as isize) as *const libc::c_uchar
                                as crate::jpeglib_h::JSAMPROW
                        }
                        i += 1
                    }
                    if height < ph0 {
                        i = height;
                        while i < ph0 {
                            let ref mut fresh4 = *row_pointer.offset(i as isize);
                            *fresh4 = *row_pointer.offset((height - 1 as libc::c_int) as isize);
                            i += 1
                        }
                    }
                    i = 0 as libc::c_int;
                    loop {
                        if !(i < (*cinfo).num_components) {
                            current_block = 939350892795860671;
                            break;
                        }
                        compptr = &mut *(*cinfo).comp_info.offset(i as isize)
                            as *mut crate::jpeglib_h::jpeg_component_info;
                        _tmpbuf[i as usize] = crate::stdlib::malloc(
                            ((*compptr)
                                .width_in_blocks
                                .wrapping_mul((*cinfo).max_h_samp_factor as libc::c_uint)
                                .wrapping_mul(8 as libc::c_int as libc::c_uint)
                                .wrapping_div((*compptr).h_samp_factor as libc::c_uint)
                                .wrapping_add(32 as libc::c_int as libc::c_uint)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                & !(32 as libc::c_int - 1 as libc::c_int) as libc::c_uint)
                                .wrapping_mul((*cinfo).max_v_samp_factor as libc::c_uint)
                                .wrapping_add(32 as libc::c_int as libc::c_uint)
                                as libc::c_ulong,
                        )
                            as *mut crate::jmorecfg_h::JSAMPLE;
                        if _tmpbuf[i as usize].is_null() {
                            crate::stdlib::snprintf(
                                (*this).errStr.as_mut_ptr(),
                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                b"%s\x00" as *const u8 as *const libc::c_char,
                                b"tjEncodeYUVPlanes(): Memory allocation failure\x00" as *const u8
                                    as *const libc::c_char,
                            );
                            (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                            crate::stdlib::snprintf(
                                errStr.as_mut_ptr(),
                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                b"%s\x00" as *const u8 as *const libc::c_char,
                                b"tjEncodeYUVPlanes(): Memory allocation failure\x00" as *const u8
                                    as *const libc::c_char,
                            );
                            retval = -(1 as libc::c_int);
                            current_block = 11274834565634332203;
                            break;
                        } else {
                            tmpbuf[i as usize] = crate::stdlib::malloc(
                                (::std::mem::size_of::<crate::jpeglib_h::JSAMPROW>()
                                    as libc::c_ulong)
                                    .wrapping_mul((*cinfo).max_v_samp_factor as libc::c_ulong),
                            )
                                as *mut crate::jpeglib_h::JSAMPROW;
                            if tmpbuf[i as usize].is_null() {
                                crate::stdlib::snprintf(
                                    (*this).errStr.as_mut_ptr(),
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                    b"%s\x00" as *const u8 as *const libc::c_char,
                                    b"tjEncodeYUVPlanes(): Memory allocation failure\x00"
                                        as *const u8
                                        as *const libc::c_char,
                                );
                                (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                                crate::stdlib::snprintf(
                                    errStr.as_mut_ptr(),
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                    b"%s\x00" as *const u8 as *const libc::c_char,
                                    b"tjEncodeYUVPlanes(): Memory allocation failure\x00"
                                        as *const u8
                                        as *const libc::c_char,
                                );
                                retval = -(1 as libc::c_int);
                                current_block = 11274834565634332203;
                                break;
                            } else {
                                row = 0 as libc::c_int;
                                while row < (*cinfo).max_v_samp_factor {
                                    let mut _tmpbuf_aligned: *mut libc::c_uchar = ((_tmpbuf
                                        [i as usize]
                                        as crate::stddef_h::size_t)
                                        .wrapping_add(32 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        & !(32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                                        as *mut libc::c_uchar;
                                    let ref mut fresh5 = *tmpbuf[i as usize].offset(row as isize);
                                    *fresh5 = &mut *_tmpbuf_aligned.offset(
                                        ((*compptr)
                                            .width_in_blocks
                                            .wrapping_mul(
                                                (*cinfo).max_h_samp_factor as libc::c_uint,
                                            )
                                            .wrapping_mul(8 as libc::c_int as libc::c_uint)
                                            .wrapping_div((*compptr).h_samp_factor as libc::c_uint)
                                            .wrapping_add(32 as libc::c_int as libc::c_uint)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                            & !(32 as libc::c_int - 1 as libc::c_int)
                                                as libc::c_uint)
                                            .wrapping_mul(row as libc::c_uint)
                                            as isize,
                                    )
                                        as *mut libc::c_uchar;
                                    row += 1
                                }
                                _tmpbuf2[i as usize] = crate::stdlib::malloc(
                                    ((*compptr)
                                        .width_in_blocks
                                        .wrapping_mul(8 as libc::c_int as libc::c_uint)
                                        .wrapping_add(32 as libc::c_int as libc::c_uint)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                        & !(32 as libc::c_int - 1 as libc::c_int) as libc::c_uint)
                                        .wrapping_mul((*compptr).v_samp_factor as libc::c_uint)
                                        .wrapping_add(32 as libc::c_int as libc::c_uint)
                                        as libc::c_ulong,
                                )
                                    as *mut crate::jmorecfg_h::JSAMPLE;
                                if _tmpbuf2[i as usize].is_null() {
                                    crate::stdlib::snprintf(
                                        (*this).errStr.as_mut_ptr(),
                                        crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                        b"%s\x00" as *const u8 as *const libc::c_char,
                                        b"tjEncodeYUVPlanes(): Memory allocation failure\x00"
                                            as *const u8
                                            as *const libc::c_char,
                                    );
                                    (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                                    crate::stdlib::snprintf(
                                        errStr.as_mut_ptr(),
                                        crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                        b"%s\x00" as *const u8 as *const libc::c_char,
                                        b"tjEncodeYUVPlanes(): Memory allocation failure\x00"
                                            as *const u8
                                            as *const libc::c_char,
                                    );
                                    retval = -(1 as libc::c_int);
                                    current_block = 11274834565634332203;
                                    break;
                                } else {
                                    tmpbuf2[i as usize] = crate::stdlib::malloc(
                                        (::std::mem::size_of::<crate::jpeglib_h::JSAMPROW>()
                                            as libc::c_ulong)
                                            .wrapping_mul(
                                                (*compptr).v_samp_factor as libc::c_ulong,
                                            ),
                                    )
                                        as *mut crate::jpeglib_h::JSAMPROW;
                                    if tmpbuf2[i as usize].is_null() {
                                        crate::stdlib::snprintf(
                                            (*this).errStr.as_mut_ptr(),
                                            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                            b"%s\x00" as *const u8 as *const libc::c_char,
                                            b"tjEncodeYUVPlanes(): Memory allocation failure\x00"
                                                as *const u8
                                                as *const libc::c_char,
                                        );
                                        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                                        crate::stdlib::snprintf(
                                            errStr.as_mut_ptr(),
                                            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                            b"%s\x00" as *const u8 as *const libc::c_char,
                                            b"tjEncodeYUVPlanes(): Memory allocation failure\x00"
                                                as *const u8
                                                as *const libc::c_char,
                                        );
                                        retval = -(1 as libc::c_int);
                                        current_block = 11274834565634332203;
                                        break;
                                    } else {
                                        row = 0 as libc::c_int;
                                        while row < (*compptr).v_samp_factor {
                                            let mut _tmpbuf2_aligned: *mut libc::c_uchar =
                                                ((_tmpbuf2[i as usize] as crate::stddef_h::size_t)
                                                    .wrapping_add(
                                                        32 as libc::c_int as libc::c_ulong,
                                                    )
                                                    .wrapping_sub(
                                                        1 as libc::c_int as libc::c_ulong,
                                                    )
                                                    & !(32 as libc::c_int - 1 as libc::c_int)
                                                        as libc::c_ulong)
                                                    as *mut libc::c_uchar;
                                            let ref mut fresh6 =
                                                *tmpbuf2[i as usize].offset(row as isize);
                                            *fresh6 = &mut *_tmpbuf2_aligned.offset(
                                                ((*compptr)
                                                    .width_in_blocks
                                                    .wrapping_mul(8 as libc::c_int as libc::c_uint)
                                                    .wrapping_add(32 as libc::c_int as libc::c_uint)
                                                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                                    & !(32 as libc::c_int - 1 as libc::c_int)
                                                        as libc::c_uint)
                                                    .wrapping_mul(row as libc::c_uint)
                                                    as isize,
                                            )
                                                as *mut libc::c_uchar;
                                            row += 1
                                        }
                                        pw[i as usize] = pw0 * (*compptr).h_samp_factor
                                            / (*cinfo).max_h_samp_factor;
                                        ph[i as usize] = ph0 * (*compptr).v_samp_factor
                                            / (*cinfo).max_v_samp_factor;
                                        outbuf[i as usize] = crate::stdlib::malloc(
                                            (::std::mem::size_of::<crate::jpeglib_h::JSAMPROW>()
                                                as libc::c_ulong)
                                                .wrapping_mul(ph[i as usize] as libc::c_ulong),
                                        )
                                            as *mut crate::jpeglib_h::JSAMPROW;
                                        if outbuf[i as usize].is_null() {
                                            crate::stdlib::snprintf((*this).errStr.as_mut_ptr(),
                                                     crate::jpeglib_h::JMSG_LENGTH_MAX as
                                                         libc::c_ulong,
                                                     b"%s\x00" as *const u8 as
                                                         *const libc::c_char,
                                                     b"tjEncodeYUVPlanes(): Memory allocation failure\x00"
                                                         as *const u8 as
                                                         *const libc::c_char);
                                            (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                                            crate::stdlib::snprintf(errStr.as_mut_ptr(),
                                                     crate::jpeglib_h::JMSG_LENGTH_MAX as
                                                         libc::c_ulong,
                                                     b"%s\x00" as *const u8 as
                                                         *const libc::c_char,
                                                     b"tjEncodeYUVPlanes(): Memory allocation failure\x00"
                                                         as *const u8 as
                                                         *const libc::c_char);
                                            retval = -(1 as libc::c_int);
                                            current_block = 11274834565634332203;
                                            break;
                                        } else {
                                            ptr = *dstPlanes.offset(i as isize);
                                            row = 0 as libc::c_int;
                                            while row < ph[i as usize] {
                                                let ref mut fresh7 =
                                                    *outbuf[i as usize].offset(row as isize);
                                                *fresh7 = ptr;
                                                ptr = ptr.offset(if !strides.is_null()
                                                    && *strides.offset(i as isize)
                                                        != 0 as libc::c_int
                                                {
                                                    *strides.offset(i as isize)
                                                } else {
                                                    pw[i as usize]
                                                }
                                                    as isize);
                                                row += 1
                                            }
                                            i += 1
                                        }
                                    }
                                }
                            }
                        }
                    }
                    match current_block {
                        11274834565634332203 => {}
                        _ => {
                            if crate::stdlib::_setjmp((*this).jerr.setjmp_buffer.as_mut_ptr()) != 0
                            {
                                /* If we get here, the JPEG code has signaled an error. */
                                retval = -(1 as libc::c_int)
                            } else {
                                row = 0 as libc::c_int;
                                while row < ph0 {
                                    Some(
                                        (*(*cinfo).cconvert)
                                            .color_convert
                                            .expect("non-null function pointer"),
                                    )
                                    .expect("non-null function pointer")(
                                        cinfo,
                                        &mut *row_pointer.offset(row as isize),
                                        tmpbuf.as_mut_ptr(),
                                        0 as libc::c_int as crate::jmorecfg_h::JDIMENSION,
                                        (*cinfo).max_v_samp_factor,
                                    );
                                    (*(*cinfo).downsample)
                                        .downsample
                                        .expect("non-null function pointer")(
                                        cinfo,
                                        tmpbuf.as_mut_ptr(),
                                        0 as libc::c_int as crate::jmorecfg_h::JDIMENSION,
                                        tmpbuf2.as_mut_ptr(),
                                        0 as libc::c_int as crate::jmorecfg_h::JDIMENSION,
                                    );
                                    i = 0 as libc::c_int;
                                    compptr = (*cinfo).comp_info;
                                    while i < (*cinfo).num_components {
                                        crate::src::jutils::jcopy_sample_rows(
                                            tmpbuf2[i as usize],
                                            0 as libc::c_int,
                                            outbuf[i as usize],
                                            row * (*compptr).v_samp_factor
                                                / (*cinfo).max_v_samp_factor,
                                            (*compptr).v_samp_factor,
                                            pw[i as usize] as crate::jmorecfg_h::JDIMENSION,
                                        );
                                        i += 1;
                                        compptr = compptr.offset(1)
                                    }
                                    row += (*cinfo).max_v_samp_factor
                                }
                                (*cinfo).next_scanline = ((*cinfo).next_scanline as libc::c_uint)
                                    .wrapping_add(height as libc::c_uint)
                                    as crate::jmorecfg_h::JDIMENSION
                                    as crate::jmorecfg_h::JDIMENSION;
                                crate::src::jcapimin::jpeg_abort_compress(cinfo);
                            }
                        }
                    }
                }
            }
        }
    }
    if (*cinfo).global_state > crate::jpegint_h::CSTATE_START {
        crate::src::jcapimin::jpeg_abort_compress(cinfo);
    }
    if !row_pointer.is_null() {
        crate::stdlib::free(row_pointer as *mut libc::c_void);
    }
    i = 0 as libc::c_int;
    while i < crate::jmorecfg_h::MAX_COMPONENTS {
        if !tmpbuf[i as usize].is_null() {
            crate::stdlib::free(tmpbuf[i as usize] as *mut libc::c_void);
        }
        if !_tmpbuf[i as usize].is_null() {
            crate::stdlib::free(_tmpbuf[i as usize] as *mut libc::c_void);
        }
        if !tmpbuf2[i as usize].is_null() {
            crate::stdlib::free(tmpbuf2[i as usize] as *mut libc::c_void);
        }
        if !_tmpbuf2[i as usize].is_null() {
            crate::stdlib::free(_tmpbuf2[i as usize] as *mut libc::c_void);
        }
        if !outbuf[i as usize].is_null() {
            crate::stdlib::free(outbuf[i as usize] as *mut libc::c_void);
        }
        i += 1
    }
    if (*this).jerr.warning != 0 {
        retval = -(1 as libc::c_int)
    }
    (*this).jerr.stopOnWarning = crate::jmorecfg_h::FALSE;
    return retval;
}
/* *
 * Encode an RGB or grayscale image into a YUV planar image.  This function
 * uses the accelerated color conversion routines in the underlying
 * codec but does not execute any of the other steps in the JPEG compression
 * process.
 *
 * @param handle a handle to a TurboJPEG compressor or transformer instance
 *
 * @param srcBuf pointer to an image buffer containing RGB or grayscale pixels
 * to be encoded
 *
 * @param width width (in pixels) of the source image
 *
 * @param pitch bytes per line in the source image.  Normally, this should be
 * <tt>width * #tjPixelSize[pixelFormat]</tt> if the image is unpadded, or
 * <tt>#TJPAD(width * #tjPixelSize[pixelFormat])</tt> if each line of the image
 * is padded to the nearest 32-bit boundary, as is the case for Windows
 * bitmaps.  You can also be clever and use this parameter to skip lines, etc.
 * Setting this parameter to 0 is the equivalent of setting it to
 * <tt>width * #tjPixelSize[pixelFormat]</tt>.
 *
 * @param height height (in pixels) of the source image
 *
 * @param pixelFormat pixel format of the source image (see @ref TJPF
 * "Pixel formats".)
 *
 * @param dstBuf pointer to an image buffer that will receive the YUV image.
 * Use #tjBufSizeYUV2() to determine the appropriate size for this buffer based
 * on the image width, height, padding, and level of chrominance subsampling.
 * The Y, U (Cb), and V (Cr) image planes will be stored sequentially in the
 * buffer (refer to @ref YUVnotes "YUV Image Format Notes".)
 *
 * @param pad the width of each line in each plane of the YUV image will be
 * padded to the nearest multiple of this number of bytes (must be a power of
 * 2.)  To generate images suitable for X Video, <tt>pad</tt> should be set to
 * 4.
 *
 * @param subsamp the level of chrominance subsampling to be used when
 * generating the YUV image (see @ref TJSAMP
 * "Chrominance subsampling options".)  To generate images suitable for X
 * Video, <tt>subsamp</tt> should be set to @ref TJSAMP_420.  This produces an
 * image compatible with the I420 (AKA "YUV420P") format.
 *
 * @param flags the bitwise OR of one or more of the @ref TJFLAG_ACCURATEDCT
 * "flags"
 *
 * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2()
 * and #tjGetErrorCode().)
*/
#[no_mangle]

pub unsafe extern "C" fn tjEncodeYUV3(
    mut handle: crate::src::turbojpeg::tjhandle,
    mut srcBuf: *const libc::c_uchar,
    mut width: libc::c_int,
    mut pitch: libc::c_int,
    mut height: libc::c_int,
    mut pixelFormat: libc::c_int,
    mut dstBuf: *mut libc::c_uchar,
    mut pad: libc::c_int,
    mut subsamp: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut dstPlanes: [*mut libc::c_uchar; 3] = [0 as *mut libc::c_uchar; 3];
    let mut pw0: libc::c_int = 0;
    let mut ph0: libc::c_int = 0;
    let mut strides: [libc::c_int; 3] = [0; 3];
    let mut retval: libc::c_int = -(1 as libc::c_int);
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    if this.is_null() {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjEncodeYUV3(): Invalid handle\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else {
        (*this).isInstanceError = crate::jmorecfg_h::FALSE;
        if width <= 0 as libc::c_int
            || height <= 0 as libc::c_int
            || dstBuf.is_null()
            || pad < 0 as libc::c_int
            || !(pad & pad - 1 as libc::c_int == 0 as libc::c_int)
            || subsamp < 0 as libc::c_int
            || subsamp >= crate::src::turbojpeg::NUMSUBOPT
        {
            crate::stdlib::snprintf(
                (*this).errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                b"tjEncodeYUV3(): Invalid argument\x00" as *const u8 as *const libc::c_char,
            );
            (*this).isInstanceError = crate::jmorecfg_h::TRUE;
            crate::stdlib::snprintf(
                errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                b"tjEncodeYUV3(): Invalid argument\x00" as *const u8 as *const libc::c_char,
            );
            retval = -(1 as libc::c_int)
        } else {
            pw0 = tjPlaneWidth(0 as libc::c_int, width, subsamp);
            ph0 = tjPlaneHeight(0 as libc::c_int, height, subsamp);
            dstPlanes[0 as libc::c_int as usize] = dstBuf;
            strides[0 as libc::c_int as usize] =
                pw0 + pad - 1 as libc::c_int & !(pad - 1 as libc::c_int);
            if subsamp == crate::src::turbojpeg::TJSAMP_GRAY as libc::c_int {
                strides[2 as libc::c_int as usize] = 0 as libc::c_int;
                strides[1 as libc::c_int as usize] = strides[2 as libc::c_int as usize];
                dstPlanes[2 as libc::c_int as usize] = crate::stddef_h::NULL as *mut libc::c_uchar;
                dstPlanes[1 as libc::c_int as usize] = dstPlanes[2 as libc::c_int as usize]
            } else {
                let mut pw1: libc::c_int = tjPlaneWidth(1 as libc::c_int, width, subsamp);
                let mut ph1: libc::c_int = tjPlaneHeight(1 as libc::c_int, height, subsamp);
                strides[2 as libc::c_int as usize] =
                    pw1 + pad - 1 as libc::c_int & !(pad - 1 as libc::c_int);
                strides[1 as libc::c_int as usize] = strides[2 as libc::c_int as usize];
                dstPlanes[1 as libc::c_int as usize] = dstPlanes[0 as libc::c_int as usize]
                    .offset((strides[0 as libc::c_int as usize] * ph0) as isize);
                dstPlanes[2 as libc::c_int as usize] = dstPlanes[1 as libc::c_int as usize]
                    .offset((strides[1 as libc::c_int as usize] * ph1) as isize)
            }
            return tjEncodeYUVPlanes(
                handle,
                srcBuf,
                width,
                pitch,
                height,
                pixelFormat,
                dstPlanes.as_mut_ptr(),
                strides.as_mut_ptr(),
                subsamp,
                flags,
            );
        }
    }
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn tjEncodeYUV2(
    mut handle: crate::src::turbojpeg::tjhandle,
    mut srcBuf: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut pitch: libc::c_int,
    mut height: libc::c_int,
    mut pixelFormat: libc::c_int,
    mut dstBuf: *mut libc::c_uchar,
    mut subsamp: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    return tjEncodeYUV3(
        handle,
        srcBuf,
        width,
        pitch,
        height,
        pixelFormat,
        dstBuf,
        4 as libc::c_int,
        subsamp,
        flags,
    );
}
#[no_mangle]

pub unsafe extern "C" fn tjEncodeYUV(
    mut handle: crate::src::turbojpeg::tjhandle,
    mut srcBuf: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut pitch: libc::c_int,
    mut height: libc::c_int,
    mut pixelSize: libc::c_int,
    mut dstBuf: *mut libc::c_uchar,
    mut subsamp: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    return tjEncodeYUV2(
        handle,
        srcBuf,
        width,
        pitch,
        height,
        getPixelFormat(pixelSize, flags),
        dstBuf,
        subsamp,
        flags,
    );
}
/* *
 * Compress a set of Y, U (Cb), and V (Cr) image planes into a JPEG image.
 *
 * @param handle a handle to a TurboJPEG compressor or transformer instance
 *
 * @param srcPlanes an array of pointers to Y, U (Cb), and V (Cr) image planes
 * (or just a Y plane, if compressing a grayscale image) that contain a YUV
 * image to be compressed.  These planes can be contiguous or non-contiguous in
 * memory.  The size of each plane should match the value returned by
 * #tjPlaneSizeYUV() for the given image width, height, strides, and level of
 * chrominance subsampling.  Refer to @ref YUVnotes "YUV Image Format Notes"
 * for more details.
 *
 * @param width width (in pixels) of the source image.  If the width is not an
 * even multiple of the MCU block width (see #tjMCUWidth), then an intermediate
 * buffer copy will be performed within TurboJPEG.
 *
 * @param strides an array of integers, each specifying the number of bytes per
 * line in the corresponding plane of the YUV source image.  Setting the stride
 * for any plane to 0 is the same as setting it to the plane width (see
 * @ref YUVnotes "YUV Image Format Notes".)  If <tt>strides</tt> is NULL, then
 * the strides for all planes will be set to their respective plane widths.
 * You can adjust the strides in order to specify an arbitrary amount of line
 * padding in each plane or to create a JPEG image from a subregion of a larger
 * YUV planar image.
 *
 * @param height height (in pixels) of the source image.  If the height is not
 * an even multiple of the MCU block height (see #tjMCUHeight), then an
 * intermediate buffer copy will be performed within TurboJPEG.
 *
 * @param subsamp the level of chrominance subsampling used in the source
 * image (see @ref TJSAMP "Chrominance subsampling options".)
 *
 * @param jpegBuf address of a pointer to an image buffer that will receive the
 * JPEG image.  TurboJPEG has the ability to reallocate the JPEG buffer to
 * accommodate the size of the JPEG image.  Thus, you can choose to:
 * -# pre-allocate the JPEG buffer with an arbitrary size using #tjAlloc() and
 * let TurboJPEG grow the buffer as needed,
 * -# set <tt>*jpegBuf</tt> to NULL to tell TurboJPEG to allocate the buffer
 * for you, or
 * -# pre-allocate the buffer to a "worst case" size determined by calling
 * #tjBufSize().  This should ensure that the buffer never has to be
 * re-allocated (setting #TJFLAG_NOREALLOC guarantees that it won't be.)
 * .
 * If you choose option 1, <tt>*jpegSize</tt> should be set to the size of your
 * pre-allocated buffer.  In any case, unless you have set #TJFLAG_NOREALLOC,
 * you should always check <tt>*jpegBuf</tt> upon return from this function, as
 * it may have changed.
 *
 * @param jpegSize pointer to an unsigned long variable that holds the size of
 * the JPEG image buffer.  If <tt>*jpegBuf</tt> points to a pre-allocated
 * buffer, then <tt>*jpegSize</tt> should be set to the size of the buffer.
 * Upon return, <tt>*jpegSize</tt> will contain the size of the JPEG image (in
 * bytes.)  If <tt>*jpegBuf</tt> points to a JPEG image buffer that is being
 * reused from a previous call to one of the JPEG compression functions, then
 * <tt>*jpegSize</tt> is ignored.
 *
 * @param jpegQual the image quality of the generated JPEG image (1 = worst,
 * 100 = best)
 *
 * @param flags the bitwise OR of one or more of the @ref TJFLAG_ACCURATEDCT
 * "flags"
 *
 * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2()
 * and #tjGetErrorCode().)
*/
#[no_mangle]

pub unsafe extern "C" fn tjCompressFromYUVPlanes(
    mut handle: crate::src::turbojpeg::tjhandle,
    mut srcPlanes: *mut *const libc::c_uchar,
    mut width: libc::c_int,
    mut strides: *const libc::c_int,
    mut height: libc::c_int,
    mut subsamp: libc::c_int,
    mut jpegBuf: *mut *mut libc::c_uchar,
    mut jpegSize: *mut libc::c_ulong,
    mut jpegQual: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut alloc: libc::c_int = 1 as libc::c_int;
    let mut pw: [libc::c_int; 10] = [0; 10];
    let mut ph: [libc::c_int; 10] = [0; 10];
    let mut iw: [libc::c_int; 10] = [0; 10];
    let mut tmpbufsize: libc::c_int = 0 as libc::c_int;
    let mut usetmpbuf: libc::c_int = 0 as libc::c_int;
    let mut th: [libc::c_int; 10] = [0; 10];
    let mut _tmpbuf: *mut crate::jmorecfg_h::JSAMPLE =
        crate::stddef_h::NULL as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ptr: *mut crate::jmorecfg_h::JSAMPLE = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut inbuf: [*mut crate::jpeglib_h::JSAMPROW; 10] =
        [0 as *mut crate::jpeglib_h::JSAMPROW; 10];
    let mut tmpbuf: [*mut crate::jpeglib_h::JSAMPROW; 10] =
        [0 as *mut crate::jpeglib_h::JSAMPROW; 10];
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    let mut cinfo: crate::jpeglib_h::j_compress_ptr =
        crate::stddef_h::NULL as crate::jpeglib_h::j_compress_ptr;
    if this.is_null() {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"Invalid handle\x00" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    cinfo = &mut (*this).cinfo;
    (*this).jerr.warning = crate::jmorecfg_h::FALSE;
    (*this).isInstanceError = crate::jmorecfg_h::FALSE;
    (*this).jerr.stopOnWarning = if flags & crate::src::turbojpeg::TJFLAG_STOPONWARNING != 0 {
        crate::jmorecfg_h::TRUE
    } else {
        crate::jmorecfg_h::FALSE
    };
    i = 0 as libc::c_int;
    while i < crate::jmorecfg_h::MAX_COMPONENTS {
        tmpbuf[i as usize] = crate::stddef_h::NULL as *mut crate::jpeglib_h::JSAMPROW;
        inbuf[i as usize] = crate::stddef_h::NULL as *mut crate::jpeglib_h::JSAMPROW;
        i += 1
    }
    if (*this).init & COMPRESS as libc::c_int == 0 as libc::c_int {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjCompressFromYUVPlanes(): Instance has not been initialized for compression\x00"
                as *const u8 as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjCompressFromYUVPlanes(): Instance has not been initialized for compression\x00"
                as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else if srcPlanes.is_null()
        || (*srcPlanes.offset(0 as libc::c_int as isize)).is_null()
        || width <= 0 as libc::c_int
        || height <= 0 as libc::c_int
        || subsamp < 0 as libc::c_int
        || subsamp >= crate::src::turbojpeg::NUMSUBOPT
        || jpegBuf.is_null()
        || jpegSize.is_null()
        || jpegQual < 0 as libc::c_int
        || jpegQual > 100 as libc::c_int
    {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjCompressFromYUVPlanes(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjCompressFromYUVPlanes(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else if subsamp != crate::src::turbojpeg::TJSAMP_GRAY as libc::c_int
        && ((*srcPlanes.offset(1 as libc::c_int as isize)).is_null()
            || (*srcPlanes.offset(2 as libc::c_int as isize)).is_null())
    {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjCompressFromYUVPlanes(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjCompressFromYUVPlanes(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else if crate::stdlib::_setjmp((*this).jerr.setjmp_buffer.as_mut_ptr()) != 0 {
        /* If we get here, the JPEG code has signaled an error. */
        retval = -(1 as libc::c_int)
    } else {
        (*cinfo).image_width = width as crate::jmorecfg_h::JDIMENSION;
        (*cinfo).image_height = height as crate::jmorecfg_h::JDIMENSION;
        if flags & crate::src::turbojpeg::TJFLAG_FORCEMMX != 0 {
            crate::stdlib::putenv(
                b"JSIMD_FORCEMMX=1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if flags & crate::src::turbojpeg::TJFLAG_FORCESSE != 0 {
            crate::stdlib::putenv(
                b"JSIMD_FORCESSE=1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if flags & crate::src::turbojpeg::TJFLAG_FORCESSE2 != 0 {
            crate::stdlib::putenv(
                b"JSIMD_FORCESSE2=1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        if flags & crate::src::turbojpeg::TJFLAG_NOREALLOC != 0 {
            alloc = 0 as libc::c_int;
            *jpegSize = tjBufSize(width, height, subsamp)
        }
        jpeg_mem_dest_tj(cinfo, jpegBuf, jpegSize, alloc);
        if setCompDefaults(
            cinfo,
            crate::src::turbojpeg::TJPF_RGB as libc::c_int,
            subsamp,
            jpegQual,
            flags,
        ) == -(1 as libc::c_int)
        {
            return -(1 as libc::c_int);
        }
        (*cinfo).raw_data_in = crate::jmorecfg_h::TRUE;
        crate::src::jcapistd::jpeg_start_compress(cinfo, crate::jmorecfg_h::TRUE);
        i = 0 as libc::c_int;
        loop {
            if !(i < (*cinfo).num_components) {
                current_block = 7385833325316299293;
                break;
            }
            let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
                &mut *(*cinfo).comp_info.offset(i as isize)
                    as *mut crate::jpeglib_h::jpeg_component_info;
            let mut ih: libc::c_int = 0;
            iw[i as usize] = (*compptr)
                .width_in_blocks
                .wrapping_mul(crate::jpeglib_h::DCTSIZE as libc::c_uint)
                as libc::c_int;
            ih = (*compptr)
                .height_in_blocks
                .wrapping_mul(crate::jpeglib_h::DCTSIZE as libc::c_uint)
                as libc::c_int;
            pw[i as usize] = ((*cinfo)
                .image_width
                .wrapping_add((*cinfo).max_h_samp_factor as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                & !((*cinfo).max_h_samp_factor - 1 as libc::c_int) as libc::c_uint)
                .wrapping_mul((*compptr).h_samp_factor as libc::c_uint)
                .wrapping_div((*cinfo).max_h_samp_factor as libc::c_uint)
                as libc::c_int;
            ph[i as usize] = ((*cinfo)
                .image_height
                .wrapping_add((*cinfo).max_v_samp_factor as libc::c_uint)
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                & !((*cinfo).max_v_samp_factor - 1 as libc::c_int) as libc::c_uint)
                .wrapping_mul((*compptr).v_samp_factor as libc::c_uint)
                .wrapping_div((*cinfo).max_v_samp_factor as libc::c_uint)
                as libc::c_int;
            if iw[i as usize] != pw[i as usize] || ih != ph[i as usize] {
                usetmpbuf = 1 as libc::c_int
            }
            th[i as usize] = (*compptr).v_samp_factor * crate::jpeglib_h::DCTSIZE;
            tmpbufsize += iw[i as usize] * th[i as usize];
            inbuf[i as usize] = crate::stdlib::malloc(
                (::std::mem::size_of::<crate::jpeglib_h::JSAMPROW>() as libc::c_ulong)
                    .wrapping_mul(ph[i as usize] as libc::c_ulong),
            ) as *mut crate::jpeglib_h::JSAMPROW;
            if inbuf[i as usize].is_null() {
                crate::stdlib::snprintf(
                    (*this).errStr.as_mut_ptr(),
                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    b"tjCompressFromYUVPlanes(): Memory allocation failure\x00" as *const u8
                        as *const libc::c_char,
                );
                (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                crate::stdlib::snprintf(
                    errStr.as_mut_ptr(),
                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    b"tjCompressFromYUVPlanes(): Memory allocation failure\x00" as *const u8
                        as *const libc::c_char,
                );
                retval = -(1 as libc::c_int);
                current_block = 4995605523671280494;
                break;
            } else {
                ptr = *srcPlanes.offset(i as isize) as *mut crate::jmorecfg_h::JSAMPLE;
                row = 0 as libc::c_int;
                while row < ph[i as usize] {
                    let ref mut fresh8 = *inbuf[i as usize].offset(row as isize);
                    *fresh8 = ptr;
                    ptr = ptr.offset(if !strides.is_null()
                        && *strides.offset(i as isize) != 0 as libc::c_int
                    {
                        *strides.offset(i as isize)
                    } else {
                        pw[i as usize]
                    } as isize);
                    row += 1
                }
                i += 1
            }
        }
        match current_block {
            4995605523671280494 => {}
            _ => {
                if usetmpbuf != 0 {
                    _tmpbuf = crate::stdlib::malloc(
                        (::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>() as libc::c_ulong)
                            .wrapping_mul(tmpbufsize as libc::c_ulong),
                    ) as *mut crate::jmorecfg_h::JSAMPLE;
                    if _tmpbuf.is_null() {
                        crate::stdlib::snprintf(
                            (*this).errStr.as_mut_ptr(),
                            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            b"tjCompressFromYUVPlanes(): Memory allocation failure\x00" as *const u8
                                as *const libc::c_char,
                        );
                        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                        crate::stdlib::snprintf(
                            errStr.as_mut_ptr(),
                            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            b"tjCompressFromYUVPlanes(): Memory allocation failure\x00" as *const u8
                                as *const libc::c_char,
                        );
                        retval = -(1 as libc::c_int);
                        current_block = 4995605523671280494;
                    } else {
                        ptr = _tmpbuf;
                        i = 0 as libc::c_int;
                        loop {
                            if !(i < (*cinfo).num_components) {
                                current_block = 6406431739208918833;
                                break;
                            }
                            tmpbuf[i as usize] = crate::stdlib::malloc(
                                (::std::mem::size_of::<crate::jpeglib_h::JSAMPROW>()
                                    as libc::c_ulong)
                                    .wrapping_mul(th[i as usize] as libc::c_ulong),
                            )
                                as *mut crate::jpeglib_h::JSAMPROW;
                            if tmpbuf[i as usize].is_null() {
                                crate::stdlib::snprintf(
                                    (*this).errStr.as_mut_ptr(),
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                    b"%s\x00" as *const u8 as *const libc::c_char,
                                    b"tjCompressFromYUVPlanes(): Memory allocation failure\x00"
                                        as *const u8
                                        as *const libc::c_char,
                                );
                                (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                                crate::stdlib::snprintf(
                                    errStr.as_mut_ptr(),
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                    b"%s\x00" as *const u8 as *const libc::c_char,
                                    b"tjCompressFromYUVPlanes(): Memory allocation failure\x00"
                                        as *const u8
                                        as *const libc::c_char,
                                );
                                retval = -(1 as libc::c_int);
                                current_block = 4995605523671280494;
                                break;
                            } else {
                                row = 0 as libc::c_int;
                                while row < th[i as usize] {
                                    let ref mut fresh9 = *tmpbuf[i as usize].offset(row as isize);
                                    *fresh9 = ptr;
                                    ptr = ptr.offset(iw[i as usize] as isize);
                                    row += 1
                                }
                                i += 1
                            }
                        }
                    }
                } else {
                    current_block = 6406431739208918833;
                }
                match current_block {
                    4995605523671280494 => {}
                    _ => {
                        if crate::stdlib::_setjmp((*this).jerr.setjmp_buffer.as_mut_ptr()) != 0 {
                            /* If we get here, the JPEG code has signaled an error. */
                            retval = -(1 as libc::c_int)
                        } else {
                            row = 0 as libc::c_int;
                            while row < (*cinfo).image_height as libc::c_int {
                                let mut yuvptr: [crate::jpeglib_h::JSAMPARRAY; 10] =
                                    [0 as *mut crate::jpeglib_h::JSAMPROW; 10];
                                let mut crow: [libc::c_int; 10] = [0; 10];
                                i = 0 as libc::c_int;
                                while i < (*cinfo).num_components {
                                    let mut compptr_0: *mut crate::jpeglib_h::jpeg_component_info =
                                        &mut *(*cinfo).comp_info.offset(i as isize)
                                            as *mut crate::jpeglib_h::jpeg_component_info;
                                    crow[i as usize] = row * (*compptr_0).v_samp_factor
                                        / (*cinfo).max_v_samp_factor;
                                    if usetmpbuf != 0 {
                                        let mut j: libc::c_int = 0;
                                        let mut k: libc::c_int = 0;
                                        j = 0 as libc::c_int;
                                        while j
                                            < (if th[i as usize] < ph[i as usize] - crow[i as usize]
                                            {
                                                th[i as usize]
                                            } else {
                                                (ph[i as usize]) - crow[i as usize]
                                            })
                                        {
                                            crate::stdlib::memcpy(
                                                *tmpbuf[i as usize].offset(j as isize)
                                                    as *mut libc::c_void,
                                                *inbuf[i as usize]
                                                    .offset((crow[i as usize] + j) as isize)
                                                    as *const libc::c_void,
                                                pw[i as usize] as libc::c_ulong,
                                            );
                                            /* Duplicate last sample in row to fill out MCU */
                                            k = pw[i as usize];
                                            while k < iw[i as usize] {
                                                *(*tmpbuf[i as usize].offset(j as isize))
                                                    .offset(k as isize) = *(*tmpbuf[i as usize]
                                                    .offset(j as isize))
                                                .offset(
                                                    (pw[i as usize] - 1 as libc::c_int) as isize,
                                                );
                                                k += 1
                                            }
                                            j += 1
                                        }
                                        /* Duplicate last row to fill out MCU */
                                        j = ph[i as usize] - crow[i as usize];
                                        while j < th[i as usize] {
                                            crate::stdlib::memcpy(
                                                *tmpbuf[i as usize].offset(j as isize)
                                                    as *mut libc::c_void,
                                                *tmpbuf[i as usize].offset(
                                                    (ph[i as usize]
                                                        - crow[i as usize]
                                                        - 1 as libc::c_int)
                                                        as isize,
                                                )
                                                    as *const libc::c_void,
                                                iw[i as usize] as libc::c_ulong,
                                            );
                                            j += 1
                                        }
                                        yuvptr[i as usize] = tmpbuf[i as usize]
                                    } else {
                                        yuvptr[i as usize] = &mut *(*inbuf
                                            .as_mut_ptr()
                                            .offset(i as isize))
                                        .offset(*crow.as_mut_ptr().offset(i as isize) as isize)
                                            as *mut crate::jpeglib_h::JSAMPROW
                                    }
                                    i += 1
                                }
                                crate::src::jcapistd::jpeg_write_raw_data(
                                    cinfo,
                                    yuvptr.as_mut_ptr(),
                                    ((*cinfo).max_v_samp_factor * crate::jpeglib_h::DCTSIZE)
                                        as crate::jmorecfg_h::JDIMENSION,
                                );
                                row += (*cinfo).max_v_samp_factor * crate::jpeglib_h::DCTSIZE
                            }
                            crate::src::jcapimin::jpeg_finish_compress(cinfo);
                        }
                    }
                }
            }
        }
    }
    if (*cinfo).global_state > crate::jpegint_h::CSTATE_START {
        crate::src::jcapimin::jpeg_abort_compress(cinfo);
    }
    i = 0 as libc::c_int;
    while i < crate::jmorecfg_h::MAX_COMPONENTS {
        if !tmpbuf[i as usize].is_null() {
            crate::stdlib::free(tmpbuf[i as usize] as *mut libc::c_void);
        }
        if !inbuf[i as usize].is_null() {
            crate::stdlib::free(inbuf[i as usize] as *mut libc::c_void);
        }
        i += 1
    }
    if !_tmpbuf.is_null() {
        crate::stdlib::free(_tmpbuf as *mut libc::c_void);
    }
    if (*this).jerr.warning != 0 {
        retval = -(1 as libc::c_int)
    }
    (*this).jerr.stopOnWarning = crate::jmorecfg_h::FALSE;
    return retval;
}
/* *
 * Compress a YUV planar image into a JPEG image.
 *
 * @param handle a handle to a TurboJPEG compressor or transformer instance
 *
 * @param srcBuf pointer to an image buffer containing a YUV planar image to be
 * compressed.  The size of this buffer should match the value returned by
 * #tjBufSizeYUV2() for the given image width, height, padding, and level of
 * chrominance subsampling.  The Y, U (Cb), and V (Cr) image planes should be
 * stored sequentially in the source buffer (refer to @ref YUVnotes
 * "YUV Image Format Notes".)
 *
 * @param width width (in pixels) of the source image.  If the width is not an
 * even multiple of the MCU block width (see #tjMCUWidth), then an intermediate
 * buffer copy will be performed within TurboJPEG.
 *
 * @param pad the line padding used in the source image.  For instance, if each
 * line in each plane of the YUV image is padded to the nearest multiple of 4
 * bytes, then <tt>pad</tt> should be set to 4.
 *
 * @param height height (in pixels) of the source image.  If the height is not
 * an even multiple of the MCU block height (see #tjMCUHeight), then an
 * intermediate buffer copy will be performed within TurboJPEG.
 *
 * @param subsamp the level of chrominance subsampling used in the source
 * image (see @ref TJSAMP "Chrominance subsampling options".)
 *
 * @param jpegBuf address of a pointer to an image buffer that will receive the
 * JPEG image.  TurboJPEG has the ability to reallocate the JPEG buffer to
 * accommodate the size of the JPEG image.  Thus, you can choose to:
 * -# pre-allocate the JPEG buffer with an arbitrary size using #tjAlloc() and
 * let TurboJPEG grow the buffer as needed,
 * -# set <tt>*jpegBuf</tt> to NULL to tell TurboJPEG to allocate the buffer
 * for you, or
 * -# pre-allocate the buffer to a "worst case" size determined by calling
 * #tjBufSize().  This should ensure that the buffer never has to be
 * re-allocated (setting #TJFLAG_NOREALLOC guarantees that it won't be.)
 * .
 * If you choose option 1, <tt>*jpegSize</tt> should be set to the size of your
 * pre-allocated buffer.  In any case, unless you have set #TJFLAG_NOREALLOC,
 * you should always check <tt>*jpegBuf</tt> upon return from this function, as
 * it may have changed.
 *
 * @param jpegSize pointer to an unsigned long variable that holds the size of
 * the JPEG image buffer.  If <tt>*jpegBuf</tt> points to a pre-allocated
 * buffer, then <tt>*jpegSize</tt> should be set to the size of the buffer.
 * Upon return, <tt>*jpegSize</tt> will contain the size of the JPEG image (in
 * bytes.)  If <tt>*jpegBuf</tt> points to a JPEG image buffer that is being
 * reused from a previous call to one of the JPEG compression functions, then
 * <tt>*jpegSize</tt> is ignored.
 *
 * @param jpegQual the image quality of the generated JPEG image (1 = worst,
 * 100 = best)
 *
 * @param flags the bitwise OR of one or more of the @ref TJFLAG_ACCURATEDCT
 * "flags"
 *
 * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2()
 * and #tjGetErrorCode().)
*/
#[no_mangle]

pub unsafe extern "C" fn tjCompressFromYUV(
    mut handle: crate::src::turbojpeg::tjhandle,
    mut srcBuf: *const libc::c_uchar,
    mut width: libc::c_int,
    mut pad: libc::c_int,
    mut height: libc::c_int,
    mut subsamp: libc::c_int,
    mut jpegBuf: *mut *mut libc::c_uchar,
    mut jpegSize: *mut libc::c_ulong,
    mut jpegQual: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut srcPlanes: [*const libc::c_uchar; 3] = [0 as *const libc::c_uchar; 3];
    let mut pw0: libc::c_int = 0;
    let mut ph0: libc::c_int = 0;
    let mut strides: [libc::c_int; 3] = [0; 3];
    let mut retval: libc::c_int = -(1 as libc::c_int);
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    if this.is_null() {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjCompressFromYUV(): Invalid handle\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else {
        (*this).isInstanceError = crate::jmorecfg_h::FALSE;
        if srcBuf.is_null()
            || width <= 0 as libc::c_int
            || pad < 1 as libc::c_int
            || height <= 0 as libc::c_int
            || subsamp < 0 as libc::c_int
            || subsamp >= crate::src::turbojpeg::NUMSUBOPT
        {
            crate::stdlib::snprintf(
                (*this).errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                b"tjCompressFromYUV(): Invalid argument\x00" as *const u8 as *const libc::c_char,
            );
            (*this).isInstanceError = crate::jmorecfg_h::TRUE;
            crate::stdlib::snprintf(
                errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                b"tjCompressFromYUV(): Invalid argument\x00" as *const u8 as *const libc::c_char,
            );
            retval = -(1 as libc::c_int)
        } else {
            pw0 = tjPlaneWidth(0 as libc::c_int, width, subsamp);
            ph0 = tjPlaneHeight(0 as libc::c_int, height, subsamp);
            srcPlanes[0 as libc::c_int as usize] = srcBuf;
            strides[0 as libc::c_int as usize] =
                pw0 + pad - 1 as libc::c_int & !(pad - 1 as libc::c_int);
            if subsamp == crate::src::turbojpeg::TJSAMP_GRAY as libc::c_int {
                strides[2 as libc::c_int as usize] = 0 as libc::c_int;
                strides[1 as libc::c_int as usize] = strides[2 as libc::c_int as usize];
                srcPlanes[2 as libc::c_int as usize] =
                    crate::stddef_h::NULL as *const libc::c_uchar;
                srcPlanes[1 as libc::c_int as usize] = srcPlanes[2 as libc::c_int as usize]
            } else {
                let mut pw1: libc::c_int = tjPlaneWidth(1 as libc::c_int, width, subsamp);
                let mut ph1: libc::c_int = tjPlaneHeight(1 as libc::c_int, height, subsamp);
                strides[2 as libc::c_int as usize] =
                    pw1 + pad - 1 as libc::c_int & !(pad - 1 as libc::c_int);
                strides[1 as libc::c_int as usize] = strides[2 as libc::c_int as usize];
                srcPlanes[1 as libc::c_int as usize] = srcPlanes[0 as libc::c_int as usize]
                    .offset((strides[0 as libc::c_int as usize] * ph0) as isize);
                srcPlanes[2 as libc::c_int as usize] = srcPlanes[1 as libc::c_int as usize]
                    .offset((strides[1 as libc::c_int as usize] * ph1) as isize)
            }
            return tjCompressFromYUVPlanes(
                handle,
                srcPlanes.as_mut_ptr(),
                width,
                strides.as_mut_ptr(),
                height,
                subsamp,
                jpegBuf,
                jpegSize,
                jpegQual,
                flags,
            );
        }
    }
    return retval;
}
/* Decompressor */

unsafe extern "C" fn _tjInitDecompress(
    mut this: *mut tjinstance,
) -> crate::src::turbojpeg::tjhandle {
    static mut buffer: [libc::c_uchar; 1] = [0; 1];
    /* This is also straight out of example.txt */
    (*this).dinfo.err = crate::src::jerror::jpeg_std_error(&mut (*this).jerr.pub_0);
    (*this).jerr.pub_0.error_exit =
        Some(my_error_exit as unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ());
    (*this).jerr.pub_0.output_message =
        Some(my_output_message as unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr) -> ());
    (*this).jerr.emit_message = (*this).jerr.pub_0.emit_message;
    (*this).jerr.pub_0.emit_message = Some(
        my_emit_message
            as unsafe extern "C" fn(_: crate::jpeglib_h::j_common_ptr, _: libc::c_int) -> (),
    );
    (*this).jerr.pub_0.addon_message_table = turbojpeg_message_table.as_mut_ptr();
    (*this).jerr.pub_0.first_addon_message = crate::cderror_h::JMSG_FIRSTADDONCODE as libc::c_int;
    (*this).jerr.pub_0.last_addon_message = crate::cderror_h_0::JMSG_LASTADDONCODE as libc::c_int;
    if crate::stdlib::_setjmp((*this).jerr.setjmp_buffer.as_mut_ptr()) != 0 {
        /* If we get here, the JPEG code has signaled an error. */
        if !this.is_null() {
            crate::stdlib::free(this as *mut libc::c_void);
        }
        return crate::stddef_h::NULL as *mut libc::c_void;
    }
    crate::src::jdapimin::jpeg_CreateDecompress(
        &mut (*this).dinfo,
        crate::jconfig_h::JPEG_LIB_VERSION,
        ::std::mem::size_of::<crate::jpeglib_h::jpeg_decompress_struct>() as libc::c_ulong,
    );
    /* Make an initial call so it will create the source manager */
    jpeg_mem_src_tj(
        &mut (*this).dinfo,
        buffer.as_mut_ptr(),
        1 as libc::c_int as libc::c_ulong,
    );
    (*this).init |= DECOMPRESS as libc::c_int;
    return this as crate::src::turbojpeg::tjhandle;
}
/* *
 * Create a TurboJPEG decompressor instance.
 *
 * @return a handle to the newly-created instance, or NULL if an error
 * occurred (see #tjGetErrorStr2().)
*/
#[no_mangle]

pub unsafe extern "C" fn tjInitDecompress() -> crate::src::turbojpeg::tjhandle {
    let mut this: *mut tjinstance = 0 as *mut tjinstance;
    this = crate::stdlib::malloc(::std::mem::size_of::<tjinstance>() as libc::c_ulong)
        as *mut tjinstance;
    if this.is_null() {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"tjInitDecompress(): Memory allocation failure\x00" as *const u8
                as *const libc::c_char,
        );
        return crate::stddef_h::NULL as *mut libc::c_void;
    }
    crate::stdlib::memset(
        this as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<tjinstance>() as libc::c_ulong,
    );
    crate::stdlib::snprintf(
        (*this).errStr.as_mut_ptr(),
        crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
        b"No error\x00" as *const u8 as *const libc::c_char,
    );
    return _tjInitDecompress(this);
}
/* *
 * Retrieve information about a JPEG image without decompressing it.
 *
 * @param handle a handle to a TurboJPEG decompressor or transformer instance
 *
 * @param jpegBuf pointer to a buffer containing a JPEG image
 *
 * @param jpegSize size of the JPEG image (in bytes)
 *
 * @param width pointer to an integer variable that will receive the width (in
 * pixels) of the JPEG image
 *
 * @param height pointer to an integer variable that will receive the height
 * (in pixels) of the JPEG image
 *
 * @param jpegSubsamp pointer to an integer variable that will receive the
 * level of chrominance subsampling used when the JPEG image was compressed
 * (see @ref TJSAMP "Chrominance subsampling options".)
 *
 * @param jpegColorspace pointer to an integer variable that will receive one
 * of the JPEG colorspace constants, indicating the colorspace of the JPEG
 * image (see @ref TJCS "JPEG colorspaces".)
 *
 * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2()
 * and #tjGetErrorCode().)
*/
#[no_mangle]

pub unsafe extern "C" fn tjDecompressHeader3(
    mut handle: crate::src::turbojpeg::tjhandle,
    mut jpegBuf: *const libc::c_uchar,
    mut jpegSize: libc::c_ulong,
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
    mut jpegSubsamp: *mut libc::c_int,
    mut jpegColorspace: *mut libc::c_int,
) -> libc::c_int {
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    let mut dinfo: crate::jpeglib_h::j_decompress_ptr =
        crate::stddef_h::NULL as crate::jpeglib_h::j_decompress_ptr;
    if this.is_null() {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"Invalid handle\x00" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    dinfo = &mut (*this).dinfo;
    (*this).jerr.warning = crate::jmorecfg_h::FALSE;
    (*this).isInstanceError = crate::jmorecfg_h::FALSE;
    if (*this).init & DECOMPRESS as libc::c_int == 0 as libc::c_int {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecompressHeader3(): Instance has not been initialized for decompression\x00"
                as *const u8 as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecompressHeader3(): Instance has not been initialized for decompression\x00"
                as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else if jpegBuf.is_null()
        || jpegSize <= 0 as libc::c_int as libc::c_ulong
        || width.is_null()
        || height.is_null()
        || jpegSubsamp.is_null()
        || jpegColorspace.is_null()
    {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecompressHeader3(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecompressHeader3(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else {
        if crate::stdlib::_setjmp((*this).jerr.setjmp_buffer.as_mut_ptr()) != 0 {
            /* If we get here, the JPEG code has signaled an error. */
            return -(1 as libc::c_int);
        }
        jpeg_mem_src_tj(dinfo, jpegBuf, jpegSize);
        crate::src::jdapimin::jpeg_read_header(dinfo, crate::jmorecfg_h::TRUE);
        *width = (*dinfo).image_width as libc::c_int;
        *height = (*dinfo).image_height as libc::c_int;
        *jpegSubsamp = getSubsamp(dinfo);
        match (*dinfo).jpeg_color_space as libc::c_uint {
            1 => *jpegColorspace = crate::src::turbojpeg::TJCS_GRAY as libc::c_int,
            2 => *jpegColorspace = crate::src::turbojpeg::TJCS_RGB as libc::c_int,
            3 => *jpegColorspace = crate::src::turbojpeg::TJCS_YCbCr as libc::c_int,
            4 => *jpegColorspace = crate::src::turbojpeg::TJCS_CMYK as libc::c_int,
            5 => *jpegColorspace = crate::src::turbojpeg::TJCS_YCCK as libc::c_int,
            _ => *jpegColorspace = -(1 as libc::c_int),
        }
        crate::src::jdapimin::jpeg_abort_decompress(dinfo);
        if *jpegSubsamp < 0 as libc::c_int {
            crate::stdlib::snprintf(
                (*this).errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                b"tjDecompressHeader3(): Could not determine subsampling type for JPEG image\x00"
                    as *const u8 as *const libc::c_char,
            );
            (*this).isInstanceError = crate::jmorecfg_h::TRUE;
            crate::stdlib::snprintf(
                errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                b"tjDecompressHeader3(): Could not determine subsampling type for JPEG image\x00"
                    as *const u8 as *const libc::c_char,
            );
            retval = -(1 as libc::c_int)
        } else if *jpegColorspace < 0 as libc::c_int {
            crate::stdlib::snprintf(
                (*this).errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                b"tjDecompressHeader3(): Could not determine colorspace of JPEG image\x00"
                    as *const u8 as *const libc::c_char,
            );
            (*this).isInstanceError = crate::jmorecfg_h::TRUE;
            crate::stdlib::snprintf(
                errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                b"tjDecompressHeader3(): Could not determine colorspace of JPEG image\x00"
                    as *const u8 as *const libc::c_char,
            );
            retval = -(1 as libc::c_int)
        } else if *width < 1 as libc::c_int || *height < 1 as libc::c_int {
            crate::stdlib::snprintf(
                (*this).errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                b"tjDecompressHeader3(): Invalid data returned in header\x00" as *const u8
                    as *const libc::c_char,
            );
            (*this).isInstanceError = crate::jmorecfg_h::TRUE;
            crate::stdlib::snprintf(
                errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                b"tjDecompressHeader3(): Invalid data returned in header\x00" as *const u8
                    as *const libc::c_char,
            );
            retval = -(1 as libc::c_int)
        }
    }
    if (*this).jerr.warning != 0 {
        retval = -(1 as libc::c_int)
    }
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn tjDecompressHeader2(
    mut handle: crate::src::turbojpeg::tjhandle,
    mut jpegBuf: *mut libc::c_uchar,
    mut jpegSize: libc::c_ulong,
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
    mut jpegSubsamp: *mut libc::c_int,
) -> libc::c_int {
    let mut jpegColorspace: libc::c_int = 0;
    return tjDecompressHeader3(
        handle,
        jpegBuf,
        jpegSize,
        width,
        height,
        jpegSubsamp,
        &mut jpegColorspace,
    );
}
#[no_mangle]

pub unsafe extern "C" fn tjDecompressHeader(
    mut handle: crate::src::turbojpeg::tjhandle,
    mut jpegBuf: *mut libc::c_uchar,
    mut jpegSize: libc::c_ulong,
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
) -> libc::c_int {
    let mut jpegSubsamp: libc::c_int = 0;
    return tjDecompressHeader2(handle, jpegBuf, jpegSize, width, height, &mut jpegSubsamp);
}
/* *
 * Returns a list of fractional scaling factors that the JPEG decompressor in
 * this implementation of TurboJPEG supports.
 *
 * @param numscalingfactors pointer to an integer variable that will receive
 * the number of elements in the list
 *
 * @return a pointer to a list of fractional scaling factors, or NULL if an
 * error is encountered (see #tjGetErrorStr2().)
*/
#[no_mangle]

pub unsafe extern "C" fn tjGetScalingFactors(
    mut numscalingfactors: *mut libc::c_int,
) -> *mut crate::src::turbojpeg::tjscalingfactor {
    if numscalingfactors.is_null() {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"tjGetScalingFactors(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        return crate::stddef_h::NULL as *mut crate::src::turbojpeg::tjscalingfactor;
    }
    *numscalingfactors = NUMSF;
    return sf.as_ptr() as *mut crate::src::turbojpeg::tjscalingfactor;
}
/* *
 * Decompress a JPEG image to an RGB, grayscale, or CMYK image.
 *
 * @param handle a handle to a TurboJPEG decompressor or transformer instance
 *
 * @param jpegBuf pointer to a buffer containing the JPEG image to decompress
 *
 * @param jpegSize size of the JPEG image (in bytes)
 *
 * @param dstBuf pointer to an image buffer that will receive the decompressed
 * image.  This buffer should normally be <tt>pitch * scaledHeight</tt> bytes
 * in size, where <tt>scaledHeight</tt> can be determined by calling
 * #TJSCALED() with the JPEG image height and one of the scaling factors
 * returned by #tjGetScalingFactors().  The <tt>dstBuf</tt> pointer may also be
 * used to decompress into a specific region of a larger buffer.
 *
 * @param width desired width (in pixels) of the destination image.  If this is
 * different than the width of the JPEG image being decompressed, then
 * TurboJPEG will use scaling in the JPEG decompressor to generate the largest
 * possible image that will fit within the desired width.  If <tt>width</tt> is
 * set to 0, then only the height will be considered when determining the
 * scaled image size.
 *
 * @param pitch bytes per line in the destination image.  Normally, this is
 * <tt>scaledWidth * #tjPixelSize[pixelFormat]</tt> if the decompressed image
 * is unpadded, else <tt>#TJPAD(scaledWidth * #tjPixelSize[pixelFormat])</tt>
 * if each line of the decompressed image is padded to the nearest 32-bit
 * boundary, as is the case for Windows bitmaps.  (NOTE: <tt>scaledWidth</tt>
 * can be determined by calling #TJSCALED() with the JPEG image width and one
 * of the scaling factors returned by #tjGetScalingFactors().)  You can also be
 * clever and use the pitch parameter to skip lines, etc.  Setting this
 * parameter to 0 is the equivalent of setting it to
 * <tt>scaledWidth * #tjPixelSize[pixelFormat]</tt>.
 *
 * @param height desired height (in pixels) of the destination image.  If this
 * is different than the height of the JPEG image being decompressed, then
 * TurboJPEG will use scaling in the JPEG decompressor to generate the largest
 * possible image that will fit within the desired height.  If <tt>height</tt>
 * is set to 0, then only the width will be considered when determining the
 * scaled image size.
 *
 * @param pixelFormat pixel format of the destination image (see @ref
 * TJPF "Pixel formats".)
 *
 * @param flags the bitwise OR of one or more of the @ref TJFLAG_ACCURATEDCT
 * "flags"
 *
 * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2()
 * and #tjGetErrorCode().)
 */
#[no_mangle]

pub unsafe extern "C" fn tjDecompress2(
    mut handle: crate::src::turbojpeg::tjhandle,
    mut jpegBuf: *const libc::c_uchar,
    mut jpegSize: libc::c_ulong,
    mut dstBuf: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut pitch: libc::c_int,
    mut height: libc::c_int,
    mut pixelFormat: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut row_pointer: *mut crate::jpeglib_h::JSAMPROW =
        crate::stddef_h::NULL as *mut crate::jpeglib_h::JSAMPROW;
    let mut i: libc::c_int = 0;
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut jpegwidth: libc::c_int = 0;
    let mut jpegheight: libc::c_int = 0;
    let mut scaledw: libc::c_int = 0;
    let mut scaledh: libc::c_int = 0;
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    let mut dinfo: crate::jpeglib_h::j_decompress_ptr =
        crate::stddef_h::NULL as crate::jpeglib_h::j_decompress_ptr;
    if this.is_null() {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"Invalid handle\x00" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    dinfo = &mut (*this).dinfo;
    (*this).jerr.warning = crate::jmorecfg_h::FALSE;
    (*this).isInstanceError = crate::jmorecfg_h::FALSE;
    (*this).jerr.stopOnWarning = if flags & crate::src::turbojpeg::TJFLAG_STOPONWARNING != 0 {
        crate::jmorecfg_h::TRUE
    } else {
        crate::jmorecfg_h::FALSE
    };
    if (*this).init & DECOMPRESS as libc::c_int == 0 as libc::c_int {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecompress2(): Instance has not been initialized for decompression\x00" as *const u8
                as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecompress2(): Instance has not been initialized for decompression\x00" as *const u8
                as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else if jpegBuf.is_null()
        || jpegSize <= 0 as libc::c_int as libc::c_ulong
        || dstBuf.is_null()
        || width < 0 as libc::c_int
        || pitch < 0 as libc::c_int
        || height < 0 as libc::c_int
        || pixelFormat < 0 as libc::c_int
        || pixelFormat >= crate::src::turbojpeg::TJ_NUMPF
    {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecompress2(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecompress2(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else {
        if flags & crate::src::turbojpeg::TJFLAG_FORCEMMX != 0 {
            crate::stdlib::putenv(
                b"JSIMD_FORCEMMX=1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if flags & crate::src::turbojpeg::TJFLAG_FORCESSE != 0 {
            crate::stdlib::putenv(
                b"JSIMD_FORCESSE=1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if flags & crate::src::turbojpeg::TJFLAG_FORCESSE2 != 0 {
            crate::stdlib::putenv(
                b"JSIMD_FORCESSE2=1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        if crate::stdlib::_setjmp((*this).jerr.setjmp_buffer.as_mut_ptr()) != 0 {
            /* If we get here, the JPEG code has signaled an error. */
            retval = -(1 as libc::c_int)
        } else {
            jpeg_mem_src_tj(dinfo, jpegBuf, jpegSize);
            crate::src::jdapimin::jpeg_read_header(dinfo, crate::jmorecfg_h::TRUE);
            (*this).dinfo.out_color_space = pf2cs[pixelFormat as usize];
            if flags & crate::src::turbojpeg::TJFLAG_FASTDCT != 0 {
                (*this).dinfo.dct_method =
                    crate::jpeglib_h::JDCT_FASTEST as crate::jpeglib_h::J_DCT_METHOD
            }
            if flags & crate::src::turbojpeg::TJFLAG_FASTUPSAMPLE != 0 {
                (*dinfo).do_fancy_upsampling = crate::jmorecfg_h::FALSE
            }
            jpegwidth = (*dinfo).image_width as libc::c_int;
            jpegheight = (*dinfo).image_height as libc::c_int;
            if width == 0 as libc::c_int {
                width = jpegwidth
            }
            if height == 0 as libc::c_int {
                height = jpegheight
            }
            i = 0 as libc::c_int;
            while i < NUMSF {
                scaledw = (jpegwidth * sf[i as usize].num + sf[i as usize].denom
                    - 1 as libc::c_int)
                    / sf[i as usize].denom;
                scaledh = (jpegheight * sf[i as usize].num + sf[i as usize].denom
                    - 1 as libc::c_int)
                    / sf[i as usize].denom;
                if scaledw <= width && scaledh <= height {
                    break;
                }
                i += 1
            }
            if i >= NUMSF {
                crate::stdlib::snprintf(
                    (*this).errStr.as_mut_ptr(),
                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    b"tjDecompress2(): Could not scale down to desired image dimensions\x00"
                        as *const u8 as *const libc::c_char,
                );
                (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                crate::stdlib::snprintf(
                    errStr.as_mut_ptr(),
                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    b"tjDecompress2(): Could not scale down to desired image dimensions\x00"
                        as *const u8 as *const libc::c_char,
                );
                retval = -(1 as libc::c_int)
            } else {
                width = scaledw;
                height = scaledh;
                (*dinfo).scale_num = sf[i as usize].num as libc::c_uint;
                (*dinfo).scale_denom = sf[i as usize].denom as libc::c_uint;
                crate::src::jdapistd::jpeg_start_decompress(dinfo);
                if pitch == 0 as libc::c_int {
                    pitch = (*dinfo)
                        .output_width
                        .wrapping_mul(tjPixelSize[pixelFormat as usize] as libc::c_uint)
                        as libc::c_int
                }
                row_pointer = crate::stdlib::malloc(
                    (::std::mem::size_of::<crate::jpeglib_h::JSAMPROW>() as libc::c_ulong)
                        .wrapping_mul((*dinfo).output_height as libc::c_ulong),
                ) as *mut crate::jpeglib_h::JSAMPROW;
                if row_pointer.is_null() {
                    crate::stdlib::snprintf(
                        (*this).errStr.as_mut_ptr(),
                        crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        b"tjDecompress2(): Memory allocation failure\x00" as *const u8
                            as *const libc::c_char,
                    );
                    (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                    crate::stdlib::snprintf(
                        errStr.as_mut_ptr(),
                        crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        b"tjDecompress2(): Memory allocation failure\x00" as *const u8
                            as *const libc::c_char,
                    );
                    retval = -(1 as libc::c_int)
                } else if crate::stdlib::_setjmp((*this).jerr.setjmp_buffer.as_mut_ptr()) != 0 {
                    /* If we get here, the JPEG code has signaled an error. */
                    retval = -(1 as libc::c_int)
                } else {
                    i = 0 as libc::c_int;
                    while i < (*dinfo).output_height as libc::c_int {
                        if flags & crate::src::turbojpeg::TJFLAG_BOTTOMUP != 0 {
                            let ref mut fresh10 = *row_pointer.offset(i as isize);
                            *fresh10 = &mut *dstBuf.offset(
                                (*dinfo)
                                    .output_height
                                    .wrapping_sub(i as libc::c_uint)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(pitch as libc::c_uint)
                                    as isize,
                            ) as *mut libc::c_uchar
                        } else {
                            let ref mut fresh11 = *row_pointer.offset(i as isize);
                            *fresh11 =
                                &mut *dstBuf.offset((i * pitch) as isize) as *mut libc::c_uchar
                        }
                        i += 1
                    }
                    while (*dinfo).output_scanline < (*dinfo).output_height {
                        crate::src::jdapistd::jpeg_read_scanlines(
                            dinfo,
                            &mut *row_pointer.offset((*dinfo).output_scanline as isize),
                            (*dinfo)
                                .output_height
                                .wrapping_sub((*dinfo).output_scanline),
                        );
                    }
                    crate::src::jdapimin::jpeg_finish_decompress(dinfo);
                }
            }
        }
    }
    if (*dinfo).global_state > crate::jpegint_h::DSTATE_START {
        crate::src::jdapimin::jpeg_abort_decompress(dinfo);
    }
    if !row_pointer.is_null() {
        crate::stdlib::free(row_pointer as *mut libc::c_void);
    }
    if (*this).jerr.warning != 0 {
        retval = -(1 as libc::c_int)
    }
    (*this).jerr.stopOnWarning = crate::jmorecfg_h::FALSE;
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn tjDecompress(
    mut handle: crate::src::turbojpeg::tjhandle,
    mut jpegBuf: *mut libc::c_uchar,
    mut jpegSize: libc::c_ulong,
    mut dstBuf: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut pitch: libc::c_int,
    mut height: libc::c_int,
    mut pixelSize: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    if flags & crate::src::turbojpeg::TJ_YUV != 0 {
        return tjDecompressToYUV(handle, jpegBuf, jpegSize, dstBuf, flags);
    } else {
        return tjDecompress2(
            handle,
            jpegBuf,
            jpegSize,
            dstBuf,
            width,
            pitch,
            height,
            getPixelFormat(pixelSize, flags),
            flags,
        );
    };
}

unsafe extern "C" fn setDecodeDefaults(
    mut dinfo: *mut crate::jpeglib_h::jpeg_decompress_struct,
    mut pixelFormat: libc::c_int,
    mut subsamp: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    (*dinfo).scale_denom = 1 as libc::c_int as libc::c_uint;
    (*dinfo).scale_num = (*dinfo).scale_denom;
    if subsamp == crate::src::turbojpeg::TJSAMP_GRAY as libc::c_int {
        (*dinfo).comps_in_scan = 1 as libc::c_int;
        (*dinfo).num_components = (*dinfo).comps_in_scan;
        (*dinfo).jpeg_color_space = crate::jpeglib_h::JCS_GRAYSCALE
    } else {
        (*dinfo).comps_in_scan = 3 as libc::c_int;
        (*dinfo).num_components = (*dinfo).comps_in_scan;
        (*dinfo).jpeg_color_space = crate::jpeglib_h::JCS_YCbCr
    }
    (*dinfo).comp_info = Some(
        (*(*dinfo).mem)
            .alloc_small
            .expect("non-null function pointer"),
    )
    .expect("non-null function pointer")(
        dinfo as crate::jpeglib_h::j_common_ptr,
        crate::jpeglib_h::JPOOL_IMAGE,
        ((*dinfo).num_components as libc::c_ulong).wrapping_mul(::std::mem::size_of::<
            crate::jpeglib_h::jpeg_component_info,
        >() as libc::c_ulong),
    ) as *mut crate::jpeglib_h::jpeg_component_info;
    i = 0 as libc::c_int;
    while i < (*dinfo).num_components {
        let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
            &mut *(*dinfo).comp_info.offset(i as isize)
                as *mut crate::jpeglib_h::jpeg_component_info;
        (*compptr).h_samp_factor = if i == 0 as libc::c_int {
            (tjMCUWidth[subsamp as usize]) / 8 as libc::c_int
        } else {
            1 as libc::c_int
        };
        (*compptr).v_samp_factor = if i == 0 as libc::c_int {
            (tjMCUHeight[subsamp as usize]) / 8 as libc::c_int
        } else {
            1 as libc::c_int
        };
        (*compptr).component_index = i;
        (*compptr).component_id = i + 1 as libc::c_int;
        (*compptr).ac_tbl_no = if i == 0 as libc::c_int {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        };
        (*compptr).dc_tbl_no = (*compptr).ac_tbl_no;
        (*compptr).quant_tbl_no = (*compptr).dc_tbl_no;
        (*dinfo).cur_comp_info[i as usize] = compptr;
        i += 1
    }
    (*dinfo).data_precision = 8 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        if (*dinfo).quant_tbl_ptrs[i as usize].is_null() {
            (*dinfo).quant_tbl_ptrs[i as usize] =
                crate::src::jcomapi::jpeg_alloc_quant_table(dinfo as crate::jpeglib_h::j_common_ptr)
        }
        i += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn my_read_markers(
    mut dinfo: crate::jpeglib_h::j_decompress_ptr,
) -> libc::c_int {
    return crate::jpeglib_h::JPEG_REACHED_SOS;
}
#[no_mangle]

pub unsafe extern "C" fn my_reset_marker_reader(mut dinfo: crate::jpeglib_h::j_decompress_ptr) {}
/* *
 * Decode a set of Y, U (Cb), and V (Cr) image planes into an RGB or grayscale
 * image.  This function uses the accelerated color conversion routines in the
 * underlying codec but does not execute any of the other steps in the JPEG
 * decompression process.
 *
 * @param handle a handle to a TurboJPEG decompressor or transformer instance
 *
 * @param srcPlanes an array of pointers to Y, U (Cb), and V (Cr) image planes
 * (or just a Y plane, if decoding a grayscale image) that contain a YUV image
 * to be decoded.  These planes can be contiguous or non-contiguous in memory.
 * The size of each plane should match the value returned by #tjPlaneSizeYUV()
 * for the given image width, height, strides, and level of chrominance
 * subsampling.  Refer to @ref YUVnotes "YUV Image Format Notes" for more
 * details.
 *
 * @param strides an array of integers, each specifying the number of bytes per
 * line in the corresponding plane of the YUV source image.  Setting the stride
 * for any plane to 0 is the same as setting it to the plane width (see
 * @ref YUVnotes "YUV Image Format Notes".)  If <tt>strides</tt> is NULL, then
 * the strides for all planes will be set to their respective plane widths.
 * You can adjust the strides in order to specify an arbitrary amount of line
 * padding in each plane or to decode a subregion of a larger YUV planar image.
 *
 * @param subsamp the level of chrominance subsampling used in the YUV source
 * image (see @ref TJSAMP "Chrominance subsampling options".)
 *
 * @param dstBuf pointer to an image buffer that will receive the decoded
 * image.  This buffer should normally be <tt>pitch * height</tt> bytes in
 * size, but the <tt>dstBuf</tt> pointer can also be used to decode into a
 * specific region of a larger buffer.
 *
 * @param width width (in pixels) of the source and destination images
 *
 * @param pitch bytes per line in the destination image.  Normally, this should
 * be <tt>width * #tjPixelSize[pixelFormat]</tt> if the destination image is
 * unpadded, or <tt>#TJPAD(width * #tjPixelSize[pixelFormat])</tt> if each line
 * of the destination image should be padded to the nearest 32-bit boundary, as
 * is the case for Windows bitmaps.  You can also be clever and use the pitch
 * parameter to skip lines, etc.  Setting this parameter to 0 is the equivalent
 * of setting it to <tt>width * #tjPixelSize[pixelFormat]</tt>.
 *
 * @param height height (in pixels) of the source and destination images
 *
 * @param pixelFormat pixel format of the destination image (see @ref TJPF
 * "Pixel formats".)
 *
 * @param flags the bitwise OR of one or more of the @ref TJFLAG_ACCURATEDCT
 * "flags"
 *
 * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2()
 * and #tjGetErrorCode().)
 */
#[no_mangle]

pub unsafe extern "C" fn tjDecodeYUVPlanes(
    mut handle: crate::src::turbojpeg::tjhandle,
    mut srcPlanes: *mut *const libc::c_uchar,
    mut strides: *const libc::c_int,
    mut subsamp: libc::c_int,
    mut dstBuf: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut pitch: libc::c_int,
    mut height: libc::c_int,
    mut pixelFormat: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut row_pointer: *mut crate::jpeglib_h::JSAMPROW =
        crate::stddef_h::NULL as *mut crate::jpeglib_h::JSAMPROW;
    let mut _tmpbuf: [*mut crate::jmorecfg_h::JSAMPLE; 10] =
        [0 as *mut crate::jmorecfg_h::JSAMPLE; 10];
    let mut tmpbuf: [*mut crate::jpeglib_h::JSAMPROW; 10] =
        [0 as *mut crate::jpeglib_h::JSAMPROW; 10];
    let mut inbuf: [*mut crate::jpeglib_h::JSAMPROW; 10] =
        [0 as *mut crate::jpeglib_h::JSAMPROW; 10];
    let mut i: libc::c_int = 0;
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut row: libc::c_int = 0;
    let mut pw0: libc::c_int = 0;
    let mut ph0: libc::c_int = 0;
    let mut pw: [libc::c_int; 10] = [0; 10];
    let mut ph: [libc::c_int; 10] = [0; 10];
    let mut ptr: *mut crate::jmorecfg_h::JSAMPLE = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
        0 as *mut crate::jpeglib_h::jpeg_component_info;
    let mut old_read_markers: Option<
        unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> libc::c_int,
    > = None;
    let mut old_reset_marker_reader: Option<
        unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> (),
    > = None;
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    let mut dinfo: crate::jpeglib_h::j_decompress_ptr =
        crate::stddef_h::NULL as crate::jpeglib_h::j_decompress_ptr;
    if this.is_null() {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"Invalid handle\x00" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    dinfo = &mut (*this).dinfo;
    (*this).jerr.warning = crate::jmorecfg_h::FALSE;
    (*this).isInstanceError = crate::jmorecfg_h::FALSE;
    (*this).jerr.stopOnWarning = if flags & crate::src::turbojpeg::TJFLAG_STOPONWARNING != 0 {
        crate::jmorecfg_h::TRUE
    } else {
        crate::jmorecfg_h::FALSE
    };
    i = 0 as libc::c_int;
    while i < crate::jmorecfg_h::MAX_COMPONENTS {
        tmpbuf[i as usize] = crate::stddef_h::NULL as *mut crate::jpeglib_h::JSAMPROW;
        _tmpbuf[i as usize] = crate::stddef_h::NULL as *mut crate::jmorecfg_h::JSAMPLE;
        inbuf[i as usize] = crate::stddef_h::NULL as *mut crate::jpeglib_h::JSAMPROW;
        i += 1
    }
    if (*this).init & DECOMPRESS as libc::c_int == 0 as libc::c_int {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecodeYUVPlanes(): Instance has not been initialized for decompression\x00"
                as *const u8 as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecodeYUVPlanes(): Instance has not been initialized for decompression\x00"
                as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else if srcPlanes.is_null()
        || (*srcPlanes.offset(0 as libc::c_int as isize)).is_null()
        || subsamp < 0 as libc::c_int
        || subsamp >= crate::src::turbojpeg::NUMSUBOPT
        || dstBuf.is_null()
        || width <= 0 as libc::c_int
        || pitch < 0 as libc::c_int
        || height <= 0 as libc::c_int
        || pixelFormat < 0 as libc::c_int
        || pixelFormat >= crate::src::turbojpeg::TJ_NUMPF
    {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecodeYUVPlanes(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecodeYUVPlanes(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else if subsamp != crate::src::turbojpeg::TJSAMP_GRAY as libc::c_int
        && ((*srcPlanes.offset(1 as libc::c_int as isize)).is_null()
            || (*srcPlanes.offset(2 as libc::c_int as isize)).is_null())
    {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecodeYUVPlanes(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecodeYUVPlanes(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else if crate::stdlib::_setjmp((*this).jerr.setjmp_buffer.as_mut_ptr()) != 0 {
        /* If we get here, the JPEG code has signaled an error. */
        retval = -(1 as libc::c_int)
    } else if pixelFormat == crate::src::turbojpeg::TJPF_CMYK as libc::c_int {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecodeYUVPlanes(): Cannot decode YUV images into CMYK pixels.\x00" as *const u8
                as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecodeYUVPlanes(): Cannot decode YUV images into CMYK pixels.\x00" as *const u8
                as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else {
        if pitch == 0 as libc::c_int {
            pitch = width * tjPixelSize[pixelFormat as usize]
        }
        (*dinfo).image_width = width as crate::jmorecfg_h::JDIMENSION;
        (*dinfo).image_height = height as crate::jmorecfg_h::JDIMENSION;
        if flags & crate::src::turbojpeg::TJFLAG_FORCEMMX != 0 {
            crate::stdlib::putenv(
                b"JSIMD_FORCEMMX=1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if flags & crate::src::turbojpeg::TJFLAG_FORCESSE != 0 {
            crate::stdlib::putenv(
                b"JSIMD_FORCESSE=1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if flags & crate::src::turbojpeg::TJFLAG_FORCESSE2 != 0 {
            crate::stdlib::putenv(
                b"JSIMD_FORCESSE2=1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        if setDecodeDefaults(dinfo, pixelFormat, subsamp, flags) == -(1 as libc::c_int) {
            retval = -(1 as libc::c_int)
        } else {
            old_read_markers = (*(*dinfo).marker).read_markers;
            (*(*dinfo).marker).read_markers = Some(
                my_read_markers
                    as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> libc::c_int,
            );
            old_reset_marker_reader = (*(*dinfo).marker).reset_marker_reader;
            (*(*dinfo).marker).reset_marker_reader = Some(
                my_reset_marker_reader
                    as unsafe extern "C" fn(_: crate::jpeglib_h::j_decompress_ptr) -> (),
            );
            crate::src::jdapimin::jpeg_read_header(dinfo, crate::jmorecfg_h::TRUE);
            (*(*dinfo).marker).read_markers = old_read_markers;
            (*(*dinfo).marker).reset_marker_reader = old_reset_marker_reader;
            (*this).dinfo.out_color_space = pf2cs[pixelFormat as usize];
            if flags & crate::src::turbojpeg::TJFLAG_FASTDCT != 0 {
                (*this).dinfo.dct_method =
                    crate::jpeglib_h::JDCT_FASTEST as crate::jpeglib_h::J_DCT_METHOD
            }
            (*dinfo).do_fancy_upsampling = crate::jmorecfg_h::FALSE;
            (*dinfo).Se = crate::jpeglib_h::DCTSIZE2 - 1 as libc::c_int;
            crate::src::jdmaster::jinit_master_decompress(dinfo);
            Some(
                (*(*dinfo).upsample)
                    .start_pass
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(dinfo);
            pw0 = width + (*dinfo).max_h_samp_factor - 1 as libc::c_int
                & !((*dinfo).max_h_samp_factor - 1 as libc::c_int);
            ph0 = height + (*dinfo).max_v_samp_factor - 1 as libc::c_int
                & !((*dinfo).max_v_samp_factor - 1 as libc::c_int);
            if pitch == 0 as libc::c_int {
                pitch = (*dinfo)
                    .output_width
                    .wrapping_mul(tjPixelSize[pixelFormat as usize] as libc::c_uint)
                    as libc::c_int
            }
            row_pointer = crate::stdlib::malloc(
                (::std::mem::size_of::<crate::jpeglib_h::JSAMPROW>() as libc::c_ulong)
                    .wrapping_mul(ph0 as libc::c_ulong),
            ) as *mut crate::jpeglib_h::JSAMPROW;
            if row_pointer.is_null() {
                crate::stdlib::snprintf(
                    (*this).errStr.as_mut_ptr(),
                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    b"tjDecodeYUVPlanes(): Memory allocation failure\x00" as *const u8
                        as *const libc::c_char,
                );
                (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                crate::stdlib::snprintf(
                    errStr.as_mut_ptr(),
                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    b"tjDecodeYUVPlanes(): Memory allocation failure\x00" as *const u8
                        as *const libc::c_char,
                );
                retval = -(1 as libc::c_int)
            } else {
                i = 0 as libc::c_int;
                while i < height {
                    if flags & crate::src::turbojpeg::TJFLAG_BOTTOMUP != 0 {
                        let ref mut fresh12 = *row_pointer.offset(i as isize);
                        *fresh12 = &mut *dstBuf
                            .offset(((height - i - 1 as libc::c_int) * pitch) as isize)
                            as *mut libc::c_uchar
                    } else {
                        let ref mut fresh13 = *row_pointer.offset(i as isize);
                        *fresh13 = &mut *dstBuf.offset((i * pitch) as isize) as *mut libc::c_uchar
                    }
                    i += 1
                }
                if height < ph0 {
                    i = height;
                    while i < ph0 {
                        let ref mut fresh14 = *row_pointer.offset(i as isize);
                        *fresh14 = *row_pointer.offset((height - 1 as libc::c_int) as isize);
                        i += 1
                    }
                }
                i = 0 as libc::c_int;
                loop {
                    if !(i < (*dinfo).num_components) {
                        current_block = 228501038991332163;
                        break;
                    }
                    compptr = &mut *(*dinfo).comp_info.offset(i as isize)
                        as *mut crate::jpeglib_h::jpeg_component_info;
                    _tmpbuf[i as usize] = crate::stdlib::malloc(
                        ((*compptr)
                            .width_in_blocks
                            .wrapping_mul(8 as libc::c_int as libc::c_uint)
                            .wrapping_add(32 as libc::c_int as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            & !(32 as libc::c_int - 1 as libc::c_int) as libc::c_uint)
                            .wrapping_mul((*compptr).v_samp_factor as libc::c_uint)
                            .wrapping_add(32 as libc::c_int as libc::c_uint)
                            as libc::c_ulong,
                    ) as *mut crate::jmorecfg_h::JSAMPLE;
                    if _tmpbuf[i as usize].is_null() {
                        crate::stdlib::snprintf(
                            (*this).errStr.as_mut_ptr(),
                            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            b"tjDecodeYUVPlanes(): Memory allocation failure\x00" as *const u8
                                as *const libc::c_char,
                        );
                        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                        crate::stdlib::snprintf(
                            errStr.as_mut_ptr(),
                            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            b"tjDecodeYUVPlanes(): Memory allocation failure\x00" as *const u8
                                as *const libc::c_char,
                        );
                        retval = -(1 as libc::c_int);
                        current_block = 5377760937109324610;
                        break;
                    } else {
                        tmpbuf[i as usize] = crate::stdlib::malloc(
                            (::std::mem::size_of::<crate::jpeglib_h::JSAMPROW>() as libc::c_ulong)
                                .wrapping_mul((*compptr).v_samp_factor as libc::c_ulong),
                        )
                            as *mut crate::jpeglib_h::JSAMPROW;
                        if tmpbuf[i as usize].is_null() {
                            crate::stdlib::snprintf(
                                (*this).errStr.as_mut_ptr(),
                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                b"%s\x00" as *const u8 as *const libc::c_char,
                                b"tjDecodeYUVPlanes(): Memory allocation failure\x00" as *const u8
                                    as *const libc::c_char,
                            );
                            (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                            crate::stdlib::snprintf(
                                errStr.as_mut_ptr(),
                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                b"%s\x00" as *const u8 as *const libc::c_char,
                                b"tjDecodeYUVPlanes(): Memory allocation failure\x00" as *const u8
                                    as *const libc::c_char,
                            );
                            retval = -(1 as libc::c_int);
                            current_block = 5377760937109324610;
                            break;
                        } else {
                            row = 0 as libc::c_int;
                            while row < (*compptr).v_samp_factor {
                                let mut _tmpbuf_aligned: *mut libc::c_uchar = ((_tmpbuf[i as usize]
                                    as crate::stddef_h::size_t)
                                    .wrapping_add(32 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    & !(32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                                    as *mut libc::c_uchar;
                                let ref mut fresh15 = *tmpbuf[i as usize].offset(row as isize);
                                *fresh15 = &mut *_tmpbuf_aligned.offset(
                                    ((*compptr)
                                        .width_in_blocks
                                        .wrapping_mul(8 as libc::c_int as libc::c_uint)
                                        .wrapping_add(32 as libc::c_int as libc::c_uint)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                        & !(32 as libc::c_int - 1 as libc::c_int) as libc::c_uint)
                                        .wrapping_mul(row as libc::c_uint)
                                        as isize,
                                ) as *mut libc::c_uchar;
                                row += 1
                            }
                            pw[i as usize] =
                                pw0 * (*compptr).h_samp_factor / (*dinfo).max_h_samp_factor;
                            ph[i as usize] =
                                ph0 * (*compptr).v_samp_factor / (*dinfo).max_v_samp_factor;
                            inbuf[i as usize] = crate::stdlib::malloc(
                                (::std::mem::size_of::<crate::jpeglib_h::JSAMPROW>()
                                    as libc::c_ulong)
                                    .wrapping_mul(ph[i as usize] as libc::c_ulong),
                            )
                                as *mut crate::jpeglib_h::JSAMPROW;
                            if inbuf[i as usize].is_null() {
                                crate::stdlib::snprintf(
                                    (*this).errStr.as_mut_ptr(),
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                    b"%s\x00" as *const u8 as *const libc::c_char,
                                    b"tjDecodeYUVPlanes(): Memory allocation failure\x00"
                                        as *const u8
                                        as *const libc::c_char,
                                );
                                (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                                crate::stdlib::snprintf(
                                    errStr.as_mut_ptr(),
                                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                    b"%s\x00" as *const u8 as *const libc::c_char,
                                    b"tjDecodeYUVPlanes(): Memory allocation failure\x00"
                                        as *const u8
                                        as *const libc::c_char,
                                );
                                retval = -(1 as libc::c_int);
                                current_block = 5377760937109324610;
                                break;
                            } else {
                                ptr = *srcPlanes.offset(i as isize)
                                    as *mut crate::jmorecfg_h::JSAMPLE;
                                row = 0 as libc::c_int;
                                while row < ph[i as usize] {
                                    let ref mut fresh16 = *inbuf[i as usize].offset(row as isize);
                                    *fresh16 = ptr;
                                    ptr = ptr.offset(if !strides.is_null()
                                        && *strides.offset(i as isize) != 0 as libc::c_int
                                    {
                                        *strides.offset(i as isize)
                                    } else {
                                        pw[i as usize]
                                    }
                                        as isize);
                                    row += 1
                                }
                                i += 1
                            }
                        }
                    }
                }
                match current_block {
                    5377760937109324610 => {}
                    _ => {
                        if crate::stdlib::_setjmp((*this).jerr.setjmp_buffer.as_mut_ptr()) != 0 {
                            /* If we get here, the JPEG code has signaled an error. */
                            retval = -(1 as libc::c_int)
                        } else {
                            row = 0 as libc::c_int;
                            while row < ph0 {
                                let mut inrow: crate::jmorecfg_h::JDIMENSION =
                                    0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
                                let mut outrow: crate::jmorecfg_h::JDIMENSION =
                                    0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
                                i = 0 as libc::c_int;
                                compptr = (*dinfo).comp_info;
                                while i < (*dinfo).num_components {
                                    crate::src::jutils::jcopy_sample_rows(
                                        inbuf[i as usize],
                                        row * (*compptr).v_samp_factor / (*dinfo).max_v_samp_factor,
                                        tmpbuf[i as usize],
                                        0 as libc::c_int,
                                        (*compptr).v_samp_factor,
                                        pw[i as usize] as crate::jmorecfg_h::JDIMENSION,
                                    );
                                    i += 1;
                                    compptr = compptr.offset(1)
                                }
                                (*(*dinfo).upsample)
                                    .upsample
                                    .expect("non-null function pointer")(
                                    dinfo,
                                    tmpbuf.as_mut_ptr(),
                                    &mut inrow,
                                    (*dinfo).max_v_samp_factor as crate::jmorecfg_h::JDIMENSION,
                                    &mut *row_pointer.offset(row as isize),
                                    &mut outrow,
                                    (*dinfo).max_v_samp_factor as crate::jmorecfg_h::JDIMENSION,
                                );
                                row += (*dinfo).max_v_samp_factor
                            }
                            crate::src::jdapimin::jpeg_abort_decompress(dinfo);
                        }
                    }
                }
            }
        }
    }
    if (*dinfo).global_state > crate::jpegint_h::DSTATE_START {
        crate::src::jdapimin::jpeg_abort_decompress(dinfo);
    }
    if !row_pointer.is_null() {
        crate::stdlib::free(row_pointer as *mut libc::c_void);
    }
    i = 0 as libc::c_int;
    while i < crate::jmorecfg_h::MAX_COMPONENTS {
        if !tmpbuf[i as usize].is_null() {
            crate::stdlib::free(tmpbuf[i as usize] as *mut libc::c_void);
        }
        if !_tmpbuf[i as usize].is_null() {
            crate::stdlib::free(_tmpbuf[i as usize] as *mut libc::c_void);
        }
        if !inbuf[i as usize].is_null() {
            crate::stdlib::free(inbuf[i as usize] as *mut libc::c_void);
        }
        i += 1
    }
    if (*this).jerr.warning != 0 {
        retval = -(1 as libc::c_int)
    }
    (*this).jerr.stopOnWarning = crate::jmorecfg_h::FALSE;
    return retval;
}
/* *
 * Decode a YUV planar image into an RGB or grayscale image.  This function
 * uses the accelerated color conversion routines in the underlying
 * codec but does not execute any of the other steps in the JPEG decompression
 * process.
 *
 * @param handle a handle to a TurboJPEG decompressor or transformer instance
 *
 * @param srcBuf pointer to an image buffer containing a YUV planar image to be
 * decoded.  The size of this buffer should match the value returned by
 * #tjBufSizeYUV2() for the given image width, height, padding, and level of
 * chrominance subsampling.  The Y, U (Cb), and V (Cr) image planes should be
 * stored sequentially in the source buffer (refer to @ref YUVnotes
 * "YUV Image Format Notes".)
 *
 * @param pad Use this parameter to specify that the width of each line in each
 * plane of the YUV source image is padded to the nearest multiple of this
 * number of bytes (must be a power of 2.)
 *
 * @param subsamp the level of chrominance subsampling used in the YUV source
 * image (see @ref TJSAMP "Chrominance subsampling options".)
 *
 * @param dstBuf pointer to an image buffer that will receive the decoded
 * image.  This buffer should normally be <tt>pitch * height</tt> bytes in
 * size, but the <tt>dstBuf</tt> pointer can also be used to decode into a
 * specific region of a larger buffer.
 *
 * @param width width (in pixels) of the source and destination images
 *
 * @param pitch bytes per line in the destination image.  Normally, this should
 * be <tt>width * #tjPixelSize[pixelFormat]</tt> if the destination image is
 * unpadded, or <tt>#TJPAD(width * #tjPixelSize[pixelFormat])</tt> if each line
 * of the destination image should be padded to the nearest 32-bit boundary, as
 * is the case for Windows bitmaps.  You can also be clever and use the pitch
 * parameter to skip lines, etc.  Setting this parameter to 0 is the equivalent
 * of setting it to <tt>width * #tjPixelSize[pixelFormat]</tt>.
 *
 * @param height height (in pixels) of the source and destination images
 *
 * @param pixelFormat pixel format of the destination image (see @ref TJPF
 * "Pixel formats".)
 *
 * @param flags the bitwise OR of one or more of the @ref TJFLAG_ACCURATEDCT
 * "flags"
 *
 * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2()
 * and #tjGetErrorCode().)
 */
#[no_mangle]

pub unsafe extern "C" fn tjDecodeYUV(
    mut handle: crate::src::turbojpeg::tjhandle,
    mut srcBuf: *const libc::c_uchar,
    mut pad: libc::c_int,
    mut subsamp: libc::c_int,
    mut dstBuf: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut pitch: libc::c_int,
    mut height: libc::c_int,
    mut pixelFormat: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut srcPlanes: [*const libc::c_uchar; 3] = [0 as *const libc::c_uchar; 3];
    let mut pw0: libc::c_int = 0;
    let mut ph0: libc::c_int = 0;
    let mut strides: [libc::c_int; 3] = [0; 3];
    let mut retval: libc::c_int = -(1 as libc::c_int);
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    if this.is_null() {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecodeYUV(): Invalid handle\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else {
        (*this).isInstanceError = crate::jmorecfg_h::FALSE;
        if srcBuf.is_null()
            || pad < 0 as libc::c_int
            || !(pad & pad - 1 as libc::c_int == 0 as libc::c_int)
            || subsamp < 0 as libc::c_int
            || subsamp >= crate::src::turbojpeg::NUMSUBOPT
            || width <= 0 as libc::c_int
            || height <= 0 as libc::c_int
        {
            crate::stdlib::snprintf(
                (*this).errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                b"tjDecodeYUV(): Invalid argument\x00" as *const u8 as *const libc::c_char,
            );
            (*this).isInstanceError = crate::jmorecfg_h::TRUE;
            crate::stdlib::snprintf(
                errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                b"tjDecodeYUV(): Invalid argument\x00" as *const u8 as *const libc::c_char,
            );
            retval = -(1 as libc::c_int)
        } else {
            pw0 = tjPlaneWidth(0 as libc::c_int, width, subsamp);
            ph0 = tjPlaneHeight(0 as libc::c_int, height, subsamp);
            srcPlanes[0 as libc::c_int as usize] = srcBuf;
            strides[0 as libc::c_int as usize] =
                pw0 + pad - 1 as libc::c_int & !(pad - 1 as libc::c_int);
            if subsamp == crate::src::turbojpeg::TJSAMP_GRAY as libc::c_int {
                strides[2 as libc::c_int as usize] = 0 as libc::c_int;
                strides[1 as libc::c_int as usize] = strides[2 as libc::c_int as usize];
                srcPlanes[2 as libc::c_int as usize] =
                    crate::stddef_h::NULL as *const libc::c_uchar;
                srcPlanes[1 as libc::c_int as usize] = srcPlanes[2 as libc::c_int as usize]
            } else {
                let mut pw1: libc::c_int = tjPlaneWidth(1 as libc::c_int, width, subsamp);
                let mut ph1: libc::c_int = tjPlaneHeight(1 as libc::c_int, height, subsamp);
                strides[2 as libc::c_int as usize] =
                    pw1 + pad - 1 as libc::c_int & !(pad - 1 as libc::c_int);
                strides[1 as libc::c_int as usize] = strides[2 as libc::c_int as usize];
                srcPlanes[1 as libc::c_int as usize] = srcPlanes[0 as libc::c_int as usize]
                    .offset((strides[0 as libc::c_int as usize] * ph0) as isize);
                srcPlanes[2 as libc::c_int as usize] = srcPlanes[1 as libc::c_int as usize]
                    .offset((strides[1 as libc::c_int as usize] * ph1) as isize)
            }
            return tjDecodeYUVPlanes(
                handle,
                srcPlanes.as_mut_ptr(),
                strides.as_mut_ptr(),
                subsamp,
                dstBuf,
                width,
                pitch,
                height,
                pixelFormat,
                flags,
            );
        }
    }
    return retval;
}
/* *
 * Decompress a JPEG image into separate Y, U (Cb), and V (Cr) image
 * planes.  This function performs JPEG decompression but leaves out the color
 * conversion step, so a planar YUV image is generated instead of an RGB image.
 *
 * @param handle a handle to a TurboJPEG decompressor or transformer instance
 *
 * @param jpegBuf pointer to a buffer containing the JPEG image to decompress
 *
 * @param jpegSize size of the JPEG image (in bytes)
 *
 * @param dstPlanes an array of pointers to Y, U (Cb), and V (Cr) image planes
 * (or just a Y plane, if decompressing a grayscale image) that will receive
 * the YUV image.  These planes can be contiguous or non-contiguous in memory.
 * Use #tjPlaneSizeYUV() to determine the appropriate size for each plane based
 * on the scaled image width, scaled image height, strides, and level of
 * chrominance subsampling.  Refer to @ref YUVnotes "YUV Image Format Notes"
 * for more details.
 *
 * @param width desired width (in pixels) of the YUV image.  If this is
 * different than the width of the JPEG image being decompressed, then
 * TurboJPEG will use scaling in the JPEG decompressor to generate the largest
 * possible image that will fit within the desired width.  If <tt>width</tt> is
 * set to 0, then only the height will be considered when determining the
 * scaled image size.  If the scaled width is not an even multiple of the MCU
 * block width (see #tjMCUWidth), then an intermediate buffer copy will be
 * performed within TurboJPEG.
 *
 * @param strides an array of integers, each specifying the number of bytes per
 * line in the corresponding plane of the output image.  Setting the stride for
 * any plane to 0 is the same as setting it to the scaled plane width (see
 * @ref YUVnotes "YUV Image Format Notes".)  If <tt>strides</tt> is NULL, then
 * the strides for all planes will be set to their respective scaled plane
 * widths.  You can adjust the strides in order to add an arbitrary amount of
 * line padding to each plane or to decompress the JPEG image into a subregion
 * of a larger YUV planar image.
 *
 * @param height desired height (in pixels) of the YUV image.  If this is
 * different than the height of the JPEG image being decompressed, then
 * TurboJPEG will use scaling in the JPEG decompressor to generate the largest
 * possible image that will fit within the desired height.  If <tt>height</tt>
 * is set to 0, then only the width will be considered when determining the
 * scaled image size.  If the scaled height is not an even multiple of the MCU
 * block height (see #tjMCUHeight), then an intermediate buffer copy will be
 * performed within TurboJPEG.
 *
 * @param flags the bitwise OR of one or more of the @ref TJFLAG_ACCURATEDCT
 * "flags"
 *
 * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2()
 * and #tjGetErrorCode().)
 */
#[no_mangle]

pub unsafe extern "C" fn tjDecompressToYUVPlanes(
    mut handle: crate::src::turbojpeg::tjhandle,
    mut jpegBuf: *const libc::c_uchar,
    mut jpegSize: libc::c_ulong,
    mut dstPlanes: *mut *mut libc::c_uchar,
    mut width: libc::c_int,
    mut strides: *mut libc::c_int,
    mut height: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut sfi: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut jpegwidth: libc::c_int = 0;
    let mut jpegheight: libc::c_int = 0;
    let mut jpegSubsamp: libc::c_int = 0;
    let mut scaledw: libc::c_int = 0;
    let mut scaledh: libc::c_int = 0;
    let mut pw: [libc::c_int; 10] = [0; 10];
    let mut ph: [libc::c_int; 10] = [0; 10];
    let mut iw: [libc::c_int; 10] = [0; 10];
    let mut tmpbufsize: libc::c_int = 0 as libc::c_int;
    let mut usetmpbuf: libc::c_int = 0 as libc::c_int;
    let mut th: [libc::c_int; 10] = [0; 10];
    let mut _tmpbuf: *mut crate::jmorecfg_h::JSAMPLE =
        crate::stddef_h::NULL as *mut crate::jmorecfg_h::JSAMPLE;
    let mut ptr: *mut crate::jmorecfg_h::JSAMPLE = 0 as *mut crate::jmorecfg_h::JSAMPLE;
    let mut outbuf: [*mut crate::jpeglib_h::JSAMPROW; 10] =
        [0 as *mut crate::jpeglib_h::JSAMPROW; 10];
    let mut tmpbuf: [*mut crate::jpeglib_h::JSAMPROW; 10] =
        [0 as *mut crate::jpeglib_h::JSAMPROW; 10];
    let mut dctsize: libc::c_int = 0;
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    let mut dinfo: crate::jpeglib_h::j_decompress_ptr =
        crate::stddef_h::NULL as crate::jpeglib_h::j_decompress_ptr;
    if this.is_null() {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"Invalid handle\x00" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    dinfo = &mut (*this).dinfo;
    (*this).jerr.warning = crate::jmorecfg_h::FALSE;
    (*this).isInstanceError = crate::jmorecfg_h::FALSE;
    (*this).jerr.stopOnWarning = if flags & crate::src::turbojpeg::TJFLAG_STOPONWARNING != 0 {
        crate::jmorecfg_h::TRUE
    } else {
        crate::jmorecfg_h::FALSE
    };
    i = 0 as libc::c_int;
    while i < crate::jmorecfg_h::MAX_COMPONENTS {
        tmpbuf[i as usize] = crate::stddef_h::NULL as *mut crate::jpeglib_h::JSAMPROW;
        outbuf[i as usize] = crate::stddef_h::NULL as *mut crate::jpeglib_h::JSAMPROW;
        i += 1
    }
    if (*this).init & DECOMPRESS as libc::c_int == 0 as libc::c_int {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecompressToYUVPlanes(): Instance has not been initialized for decompression\x00"
                as *const u8 as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecompressToYUVPlanes(): Instance has not been initialized for decompression\x00"
                as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else if jpegBuf.is_null()
        || jpegSize <= 0 as libc::c_int as libc::c_ulong
        || dstPlanes.is_null()
        || (*dstPlanes.offset(0 as libc::c_int as isize)).is_null()
        || width < 0 as libc::c_int
        || height < 0 as libc::c_int
    {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecompressToYUVPlanes(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecompressToYUVPlanes(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else {
        if flags & crate::src::turbojpeg::TJFLAG_FORCEMMX != 0 {
            crate::stdlib::putenv(
                b"JSIMD_FORCEMMX=1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if flags & crate::src::turbojpeg::TJFLAG_FORCESSE != 0 {
            crate::stdlib::putenv(
                b"JSIMD_FORCESSE=1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if flags & crate::src::turbojpeg::TJFLAG_FORCESSE2 != 0 {
            crate::stdlib::putenv(
                b"JSIMD_FORCESSE2=1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        if crate::stdlib::_setjmp((*this).jerr.setjmp_buffer.as_mut_ptr()) != 0 {
            /* If we get here, the JPEG code has signaled an error. */
            retval = -(1 as libc::c_int)
        } else {
            if (*this).headerRead == 0 {
                jpeg_mem_src_tj(dinfo, jpegBuf, jpegSize);
                crate::src::jdapimin::jpeg_read_header(dinfo, crate::jmorecfg_h::TRUE);
            }
            (*this).headerRead = 0 as libc::c_int;
            jpegSubsamp = getSubsamp(dinfo);
            if jpegSubsamp < 0 as libc::c_int {
                crate::stdlib::snprintf((*this).errStr.as_mut_ptr(),
                         crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                         b"%s\x00" as *const u8 as *const libc::c_char,
                         b"tjDecompressToYUVPlanes(): Could not determine subsampling type for JPEG image\x00"
                             as *const u8 as *const libc::c_char);
                (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                crate::stdlib::snprintf(errStr.as_mut_ptr(),
                         crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                         b"%s\x00" as *const u8 as *const libc::c_char,
                         b"tjDecompressToYUVPlanes(): Could not determine subsampling type for JPEG image\x00"
                             as *const u8 as *const libc::c_char);
                retval = -(1 as libc::c_int)
            } else if jpegSubsamp != crate::src::turbojpeg::TJSAMP_GRAY as libc::c_int
                && ((*dstPlanes.offset(1 as libc::c_int as isize)).is_null()
                    || (*dstPlanes.offset(2 as libc::c_int as isize)).is_null())
            {
                crate::stdlib::snprintf(
                    (*this).errStr.as_mut_ptr(),
                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    b"tjDecompressToYUVPlanes(): Invalid argument\x00" as *const u8
                        as *const libc::c_char,
                );
                (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                crate::stdlib::snprintf(
                    errStr.as_mut_ptr(),
                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    b"tjDecompressToYUVPlanes(): Invalid argument\x00" as *const u8
                        as *const libc::c_char,
                );
                retval = -(1 as libc::c_int)
            } else {
                jpegwidth = (*dinfo).image_width as libc::c_int;
                jpegheight = (*dinfo).image_height as libc::c_int;
                if width == 0 as libc::c_int {
                    width = jpegwidth
                }
                if height == 0 as libc::c_int {
                    height = jpegheight
                }
                i = 0 as libc::c_int;
                while i < NUMSF {
                    scaledw = (jpegwidth * sf[i as usize].num + sf[i as usize].denom
                        - 1 as libc::c_int)
                        / sf[i as usize].denom;
                    scaledh = (jpegheight * sf[i as usize].num + sf[i as usize].denom
                        - 1 as libc::c_int)
                        / sf[i as usize].denom;
                    if scaledw <= width && scaledh <= height {
                        break;
                    }
                    i += 1
                }
                if i >= NUMSF {
                    crate::stdlib::snprintf((*this).errStr.as_mut_ptr(),
                             crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                             b"%s\x00" as *const u8 as *const libc::c_char,
                             b"tjDecompressToYUVPlanes(): Could not scale down to desired image dimensions\x00"
                                 as *const u8 as *const libc::c_char);
                    (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                    crate::stdlib::snprintf(errStr.as_mut_ptr(),
                             crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                             b"%s\x00" as *const u8 as *const libc::c_char,
                             b"tjDecompressToYUVPlanes(): Could not scale down to desired image dimensions\x00"
                                 as *const u8 as *const libc::c_char);
                    retval = -(1 as libc::c_int)
                } else if (*dinfo).num_components > 3 as libc::c_int {
                    crate::stdlib::snprintf(
                        (*this).errStr.as_mut_ptr(),
                        crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        b"tjDecompressToYUVPlanes(): JPEG image must have 3 or fewer components\x00"
                            as *const u8 as *const libc::c_char,
                    );
                    (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                    crate::stdlib::snprintf(
                        errStr.as_mut_ptr(),
                        crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        b"tjDecompressToYUVPlanes(): JPEG image must have 3 or fewer components\x00"
                            as *const u8 as *const libc::c_char,
                    );
                    retval = -(1 as libc::c_int)
                } else {
                    width = scaledw;
                    height = scaledh;
                    (*dinfo).scale_num = sf[i as usize].num as libc::c_uint;
                    (*dinfo).scale_denom = sf[i as usize].denom as libc::c_uint;
                    sfi = i;
                    crate::src::jdmaster::jpeg_calc_output_dimensions(dinfo);
                    dctsize =
                        crate::jpeglib_h::DCTSIZE * sf[sfi as usize].num / sf[sfi as usize].denom;
                    i = 0 as libc::c_int;
                    loop {
                        if !(i < (*dinfo).num_components) {
                            current_block = 11064061988481400464;
                            break;
                        }
                        let mut compptr: *mut crate::jpeglib_h::jpeg_component_info =
                            &mut *(*dinfo).comp_info.offset(i as isize)
                                as *mut crate::jpeglib_h::jpeg_component_info;
                        let mut ih: libc::c_int = 0;
                        iw[i as usize] = (*compptr)
                            .width_in_blocks
                            .wrapping_mul(dctsize as libc::c_uint)
                            as libc::c_int;
                        ih = (*compptr)
                            .height_in_blocks
                            .wrapping_mul(dctsize as libc::c_uint)
                            as libc::c_int;
                        pw[i as usize] = ((*dinfo)
                            .output_width
                            .wrapping_add((*dinfo).max_h_samp_factor as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            & !((*dinfo).max_h_samp_factor - 1 as libc::c_int) as libc::c_uint)
                            .wrapping_mul((*compptr).h_samp_factor as libc::c_uint)
                            .wrapping_div((*dinfo).max_h_samp_factor as libc::c_uint)
                            as libc::c_int;
                        ph[i as usize] = ((*dinfo)
                            .output_height
                            .wrapping_add((*dinfo).max_v_samp_factor as libc::c_uint)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            & !((*dinfo).max_v_samp_factor - 1 as libc::c_int) as libc::c_uint)
                            .wrapping_mul((*compptr).v_samp_factor as libc::c_uint)
                            .wrapping_div((*dinfo).max_v_samp_factor as libc::c_uint)
                            as libc::c_int;
                        if iw[i as usize] != pw[i as usize] || ih != ph[i as usize] {
                            usetmpbuf = 1 as libc::c_int
                        }
                        th[i as usize] = (*compptr).v_samp_factor * dctsize;
                        tmpbufsize += iw[i as usize] * th[i as usize];
                        outbuf[i as usize] = crate::stdlib::malloc(
                            (::std::mem::size_of::<crate::jpeglib_h::JSAMPROW>() as libc::c_ulong)
                                .wrapping_mul(ph[i as usize] as libc::c_ulong),
                        )
                            as *mut crate::jpeglib_h::JSAMPROW;
                        if outbuf[i as usize].is_null() {
                            crate::stdlib::snprintf(
                                (*this).errStr.as_mut_ptr(),
                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                b"%s\x00" as *const u8 as *const libc::c_char,
                                b"tjDecompressToYUVPlanes(): Memory allocation failure\x00"
                                    as *const u8
                                    as *const libc::c_char,
                            );
                            (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                            crate::stdlib::snprintf(
                                errStr.as_mut_ptr(),
                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                b"%s\x00" as *const u8 as *const libc::c_char,
                                b"tjDecompressToYUVPlanes(): Memory allocation failure\x00"
                                    as *const u8
                                    as *const libc::c_char,
                            );
                            retval = -(1 as libc::c_int);
                            current_block = 12580004273960699447;
                            break;
                        } else {
                            ptr = *dstPlanes.offset(i as isize);
                            row = 0 as libc::c_int;
                            while row < ph[i as usize] {
                                let ref mut fresh17 = *outbuf[i as usize].offset(row as isize);
                                *fresh17 = ptr;
                                ptr = ptr.offset(if !strides.is_null()
                                    && *strides.offset(i as isize) != 0 as libc::c_int
                                {
                                    *strides.offset(i as isize)
                                } else {
                                    pw[i as usize]
                                } as isize);
                                row += 1
                            }
                            i += 1
                        }
                    }
                    match current_block {
                        12580004273960699447 => {}
                        _ => {
                            if usetmpbuf != 0 {
                                _tmpbuf = crate::stdlib::malloc(
                                    (::std::mem::size_of::<crate::jmorecfg_h::JSAMPLE>()
                                        as libc::c_ulong)
                                        .wrapping_mul(tmpbufsize as libc::c_ulong),
                                )
                                    as *mut crate::jmorecfg_h::JSAMPLE;
                                if _tmpbuf.is_null() {
                                    crate::stdlib::snprintf(
                                        (*this).errStr.as_mut_ptr(),
                                        crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                        b"%s\x00" as *const u8 as *const libc::c_char,
                                        b"tjDecompressToYUVPlanes(): Memory allocation failure\x00"
                                            as *const u8
                                            as *const libc::c_char,
                                    );
                                    (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                                    crate::stdlib::snprintf(
                                        errStr.as_mut_ptr(),
                                        crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                        b"%s\x00" as *const u8 as *const libc::c_char,
                                        b"tjDecompressToYUVPlanes(): Memory allocation failure\x00"
                                            as *const u8
                                            as *const libc::c_char,
                                    );
                                    retval = -(1 as libc::c_int);
                                    current_block = 12580004273960699447;
                                } else {
                                    ptr = _tmpbuf;
                                    i = 0 as libc::c_int;
                                    loop {
                                        if !(i < (*dinfo).num_components) {
                                            current_block = 1966075811433896587;
                                            break;
                                        }
                                        tmpbuf[i as usize] = crate::stdlib::malloc(
                                            (::std::mem::size_of::<crate::jpeglib_h::JSAMPROW>()
                                                as libc::c_ulong)
                                                .wrapping_mul(th[i as usize] as libc::c_ulong),
                                        )
                                            as *mut crate::jpeglib_h::JSAMPROW;
                                        if tmpbuf[i as usize].is_null() {
                                            crate::stdlib::snprintf((*this).errStr.as_mut_ptr(),
                                                     crate::jpeglib_h::JMSG_LENGTH_MAX as
                                                         libc::c_ulong,
                                                     b"%s\x00" as *const u8 as
                                                         *const libc::c_char,
                                                     b"tjDecompressToYUVPlanes(): Memory allocation failure\x00"
                                                         as *const u8 as
                                                         *const libc::c_char);
                                            (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                                            crate::stdlib::snprintf(errStr.as_mut_ptr(),
                                                     crate::jpeglib_h::JMSG_LENGTH_MAX as
                                                         libc::c_ulong,
                                                     b"%s\x00" as *const u8 as
                                                         *const libc::c_char,
                                                     b"tjDecompressToYUVPlanes(): Memory allocation failure\x00"
                                                         as *const u8 as
                                                         *const libc::c_char);
                                            retval = -(1 as libc::c_int);
                                            current_block = 12580004273960699447;
                                            break;
                                        } else {
                                            row = 0 as libc::c_int;
                                            while row < th[i as usize] {
                                                let ref mut fresh18 =
                                                    *tmpbuf[i as usize].offset(row as isize);
                                                *fresh18 = ptr;
                                                ptr = ptr.offset(iw[i as usize] as isize);
                                                row += 1
                                            }
                                            i += 1
                                        }
                                    }
                                }
                            } else {
                                current_block = 1966075811433896587;
                            }
                            match current_block {
                                12580004273960699447 => {}
                                _ => {
                                    if crate::stdlib::_setjmp(
                                        (*this).jerr.setjmp_buffer.as_mut_ptr(),
                                    ) != 0
                                    {
                                        /* If we get here, the JPEG code has signaled an error. */
                                        retval = -(1 as libc::c_int)
                                    } else {
                                        if flags & crate::src::turbojpeg::TJFLAG_FASTUPSAMPLE != 0 {
                                            (*dinfo).do_fancy_upsampling = crate::jmorecfg_h::FALSE
                                        }
                                        if flags & crate::src::turbojpeg::TJFLAG_FASTDCT != 0 {
                                            (*dinfo).dct_method = crate::jpeglib_h::JDCT_FASTEST
                                                as crate::jpeglib_h::J_DCT_METHOD
                                        }
                                        (*dinfo).raw_data_out = crate::jmorecfg_h::TRUE;
                                        crate::src::jdapistd::jpeg_start_decompress(dinfo);
                                        row = 0 as libc::c_int;
                                        while row < (*dinfo).output_height as libc::c_int {
                                            let mut yuvptr: [crate::jpeglib_h::JSAMPARRAY; 10] =
                                                [0 as *mut crate::jpeglib_h::JSAMPROW; 10];
                                            let mut crow: [libc::c_int; 10] = [0; 10];
                                            i = 0 as libc::c_int;
                                            while i < (*dinfo).num_components {
                                                let mut compptr_0:
                                                        *mut crate::jpeglib_h::jpeg_component_info =
                                                    &mut *(*dinfo).comp_info.offset(i
                                                                                        as
                                                                                        isize)
                                                        as
                                                        *mut crate::jpeglib_h::jpeg_component_info;
                                                if jpegSubsamp == crate::src::turbojpeg::TJ_420 {
                                                    /* When 4:2:0 subsampling is used with IDCT scaling, libjpeg will try
                                                    to be clever and use the IDCT to perform upsampling on the U and V
                                                    planes.  For instance, if the output image is to be scaled by 1/2
                                                    relative to the JPEG image, then the scaling factor and upsampling
                                                    effectively cancel each other, so a normal 8x8 IDCT can be used.
                                                    However, this is not desirable when using the decompress-to-YUV
                                                    functionality in TurboJPEG, since we want to output the U and V
                                                    planes in their subsampled form.  Thus, we have to override some
                                                    internal libjpeg parameters to force it to use the "scaled" IDCT
                                                    functions on the U and V planes. */
                                                    (*compptr_0).DCT_scaled_size = dctsize;
                                                    (*compptr_0).MCU_sample_width = tjMCUWidth
                                                        [jpegSubsamp as usize]
                                                        * sf[sfi as usize].num
                                                        / sf[sfi as usize].denom
                                                        * (*compptr_0).v_samp_factor
                                                        / (*dinfo).max_v_samp_factor;
                                                    (*(*dinfo).idct).inverse_DCT[i as usize] =
                                                        (*(*dinfo).idct).inverse_DCT
                                                            [0 as libc::c_int as usize]
                                                }
                                                crow[i as usize] = row * (*compptr_0).v_samp_factor
                                                    / (*dinfo).max_v_samp_factor;
                                                if usetmpbuf != 0 {
                                                    yuvptr[i as usize] = tmpbuf[i as usize]
                                                } else {
                                                    yuvptr[i as usize] = &mut *(*outbuf
                                                        .as_mut_ptr()
                                                        .offset(i as isize))
                                                    .offset(*crow.as_mut_ptr().offset(i as isize)
                                                        as isize)
                                                        as *mut crate::jpeglib_h::JSAMPROW
                                                }
                                                i += 1
                                            }
                                            crate::src::jdapistd::jpeg_read_raw_data(
                                                dinfo,
                                                yuvptr.as_mut_ptr(),
                                                ((*dinfo).max_v_samp_factor
                                                    * (*dinfo).min_DCT_scaled_size)
                                                    as crate::jmorecfg_h::JDIMENSION,
                                            );
                                            if usetmpbuf != 0 {
                                                let mut j: libc::c_int = 0;
                                                i = 0 as libc::c_int;
                                                while i < (*dinfo).num_components {
                                                    j = 0 as libc::c_int;
                                                    while j
                                                        < (if th[i as usize]
                                                            < ph[i as usize] - crow[i as usize]
                                                        {
                                                            th[i as usize]
                                                        } else {
                                                            (ph[i as usize]) - crow[i as usize]
                                                        })
                                                    {
                                                        crate::stdlib::memcpy(
                                                            *outbuf[i as usize].offset(
                                                                (crow[i as usize] + j) as isize,
                                                            )
                                                                as *mut libc::c_void,
                                                            *tmpbuf[i as usize].offset(j as isize)
                                                                as *const libc::c_void,
                                                            pw[i as usize] as libc::c_ulong,
                                                        );
                                                        j += 1
                                                    }
                                                    i += 1
                                                }
                                            }
                                            row += (*dinfo).max_v_samp_factor
                                                * (*dinfo).min_DCT_scaled_size
                                        }
                                        crate::src::jdapimin::jpeg_finish_decompress(dinfo);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if (*dinfo).global_state > crate::jpegint_h::DSTATE_START {
        crate::src::jdapimin::jpeg_abort_decompress(dinfo);
    }
    i = 0 as libc::c_int;
    while i < crate::jmorecfg_h::MAX_COMPONENTS {
        if !tmpbuf[i as usize].is_null() {
            crate::stdlib::free(tmpbuf[i as usize] as *mut libc::c_void);
        }
        if !outbuf[i as usize].is_null() {
            crate::stdlib::free(outbuf[i as usize] as *mut libc::c_void);
        }
        i += 1
    }
    if !_tmpbuf.is_null() {
        crate::stdlib::free(_tmpbuf as *mut libc::c_void);
    }
    if (*this).jerr.warning != 0 {
        retval = -(1 as libc::c_int)
    }
    (*this).jerr.stopOnWarning = crate::jmorecfg_h::FALSE;
    return retval;
}
/* *
 * Decompress a JPEG image to a YUV planar image.  This function performs JPEG
 * decompression but leaves out the color conversion step, so a planar YUV
 * image is generated instead of an RGB image.
 *
 * @param handle a handle to a TurboJPEG decompressor or transformer instance
 *
 * @param jpegBuf pointer to a buffer containing the JPEG image to decompress
 *
 * @param jpegSize size of the JPEG image (in bytes)
 *
 * @param dstBuf pointer to an image buffer that will receive the YUV image.
 * Use #tjBufSizeYUV2() to determine the appropriate size for this buffer based
 * on the image width, height, padding, and level of subsampling.  The Y,
 * U (Cb), and V (Cr) image planes will be stored sequentially in the buffer
 * (refer to @ref YUVnotes "YUV Image Format Notes".)
 *
 * @param width desired width (in pixels) of the YUV image.  If this is
 * different than the width of the JPEG image being decompressed, then
 * TurboJPEG will use scaling in the JPEG decompressor to generate the largest
 * possible image that will fit within the desired width.  If <tt>width</tt> is
 * set to 0, then only the height will be considered when determining the
 * scaled image size.  If the scaled width is not an even multiple of the MCU
 * block width (see #tjMCUWidth), then an intermediate buffer copy will be
 * performed within TurboJPEG.
 *
 * @param pad the width of each line in each plane of the YUV image will be
 * padded to the nearest multiple of this number of bytes (must be a power of
 * 2.)  To generate images suitable for X Video, <tt>pad</tt> should be set to
 * 4.
 *
 * @param height desired height (in pixels) of the YUV image.  If this is
 * different than the height of the JPEG image being decompressed, then
 * TurboJPEG will use scaling in the JPEG decompressor to generate the largest
 * possible image that will fit within the desired height.  If <tt>height</tt>
 * is set to 0, then only the width will be considered when determining the
 * scaled image size.  If the scaled height is not an even multiple of the MCU
 * block height (see #tjMCUHeight), then an intermediate buffer copy will be
 * performed within TurboJPEG.
 *
 * @param flags the bitwise OR of one or more of the @ref TJFLAG_ACCURATEDCT
 * "flags"
 *
 * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2()
 * and #tjGetErrorCode().)
 */
#[no_mangle]

pub unsafe extern "C" fn tjDecompressToYUV2(
    mut handle: crate::src::turbojpeg::tjhandle,
    mut jpegBuf: *const libc::c_uchar,
    mut jpegSize: libc::c_ulong,
    mut dstBuf: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut pad: libc::c_int,
    mut height: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut dstPlanes: [*mut libc::c_uchar; 3] = [0 as *mut libc::c_uchar; 3];
    let mut pw0: libc::c_int = 0;
    let mut ph0: libc::c_int = 0;
    let mut strides: [libc::c_int; 3] = [0; 3];
    let mut retval: libc::c_int = -(1 as libc::c_int);
    let mut jpegSubsamp: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0;
    let mut jpegwidth: libc::c_int = 0;
    let mut jpegheight: libc::c_int = 0;
    let mut scaledw: libc::c_int = 0;
    let mut scaledh: libc::c_int = 0;
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    let mut dinfo: crate::jpeglib_h::j_decompress_ptr =
        crate::stddef_h::NULL as crate::jpeglib_h::j_decompress_ptr;
    if this.is_null() {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"Invalid handle\x00" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    dinfo = &mut (*this).dinfo;
    (*this).jerr.warning = crate::jmorecfg_h::FALSE;
    (*this).isInstanceError = crate::jmorecfg_h::FALSE;
    (*this).jerr.stopOnWarning = if flags & crate::src::turbojpeg::TJFLAG_STOPONWARNING != 0 {
        crate::jmorecfg_h::TRUE
    } else {
        crate::jmorecfg_h::FALSE
    };
    if jpegBuf.is_null()
        || jpegSize <= 0 as libc::c_int as libc::c_ulong
        || dstBuf.is_null()
        || width < 0 as libc::c_int
        || pad < 1 as libc::c_int
        || !(pad & pad - 1 as libc::c_int == 0 as libc::c_int)
        || height < 0 as libc::c_int
    {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecompressToYUV2(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjDecompressToYUV2(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else {
        if crate::stdlib::_setjmp((*this).jerr.setjmp_buffer.as_mut_ptr()) != 0 {
            /* If we get here, the JPEG code has signaled an error. */
            return -(1 as libc::c_int);
        }
        jpeg_mem_src_tj(dinfo, jpegBuf, jpegSize);
        crate::src::jdapimin::jpeg_read_header(dinfo, crate::jmorecfg_h::TRUE);
        jpegSubsamp = getSubsamp(dinfo);
        if jpegSubsamp < 0 as libc::c_int {
            crate::stdlib::snprintf(
                (*this).errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                b"tjDecompressToYUV2(): Could not determine subsampling type for JPEG image\x00"
                    as *const u8 as *const libc::c_char,
            );
            (*this).isInstanceError = crate::jmorecfg_h::TRUE;
            crate::stdlib::snprintf(
                errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                b"tjDecompressToYUV2(): Could not determine subsampling type for JPEG image\x00"
                    as *const u8 as *const libc::c_char,
            );
            retval = -(1 as libc::c_int)
        } else {
            jpegwidth = (*dinfo).image_width as libc::c_int;
            jpegheight = (*dinfo).image_height as libc::c_int;
            if width == 0 as libc::c_int {
                width = jpegwidth
            }
            if height == 0 as libc::c_int {
                height = jpegheight
            }
            i = 0 as libc::c_int;
            while i < NUMSF {
                scaledw = (jpegwidth * sf[i as usize].num + sf[i as usize].denom
                    - 1 as libc::c_int)
                    / sf[i as usize].denom;
                scaledh = (jpegheight * sf[i as usize].num + sf[i as usize].denom
                    - 1 as libc::c_int)
                    / sf[i as usize].denom;
                if scaledw <= width && scaledh <= height {
                    break;
                }
                i += 1
            }
            if i >= NUMSF {
                crate::stdlib::snprintf(
                    (*this).errStr.as_mut_ptr(),
                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    b"tjDecompressToYUV2(): Could not scale down to desired image dimensions\x00"
                        as *const u8 as *const libc::c_char,
                );
                (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                crate::stdlib::snprintf(
                    errStr.as_mut_ptr(),
                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    b"tjDecompressToYUV2(): Could not scale down to desired image dimensions\x00"
                        as *const u8 as *const libc::c_char,
                );
                retval = -(1 as libc::c_int)
            } else {
                pw0 = tjPlaneWidth(0 as libc::c_int, width, jpegSubsamp);
                ph0 = tjPlaneHeight(0 as libc::c_int, height, jpegSubsamp);
                dstPlanes[0 as libc::c_int as usize] = dstBuf;
                strides[0 as libc::c_int as usize] =
                    pw0 + pad - 1 as libc::c_int & !(pad - 1 as libc::c_int);
                if jpegSubsamp == crate::src::turbojpeg::TJSAMP_GRAY as libc::c_int {
                    strides[2 as libc::c_int as usize] = 0 as libc::c_int;
                    strides[1 as libc::c_int as usize] = strides[2 as libc::c_int as usize];
                    dstPlanes[2 as libc::c_int as usize] =
                        crate::stddef_h::NULL as *mut libc::c_uchar;
                    dstPlanes[1 as libc::c_int as usize] = dstPlanes[2 as libc::c_int as usize]
                } else {
                    let mut pw1: libc::c_int = tjPlaneWidth(1 as libc::c_int, width, jpegSubsamp);
                    let mut ph1: libc::c_int = tjPlaneHeight(1 as libc::c_int, height, jpegSubsamp);
                    strides[2 as libc::c_int as usize] =
                        pw1 + pad - 1 as libc::c_int & !(pad - 1 as libc::c_int);
                    strides[1 as libc::c_int as usize] = strides[2 as libc::c_int as usize];
                    dstPlanes[1 as libc::c_int as usize] = dstPlanes[0 as libc::c_int as usize]
                        .offset((strides[0 as libc::c_int as usize] * ph0) as isize);
                    dstPlanes[2 as libc::c_int as usize] = dstPlanes[1 as libc::c_int as usize]
                        .offset((strides[1 as libc::c_int as usize] * ph1) as isize)
                }
                (*this).headerRead = 1 as libc::c_int;
                return tjDecompressToYUVPlanes(
                    handle,
                    jpegBuf,
                    jpegSize,
                    dstPlanes.as_mut_ptr(),
                    width,
                    strides.as_mut_ptr(),
                    height,
                    flags,
                );
            }
        }
    }
    (*this).jerr.stopOnWarning = crate::jmorecfg_h::FALSE;
    return retval;
}
#[no_mangle]

pub unsafe extern "C" fn tjDecompressToYUV(
    mut handle: crate::src::turbojpeg::tjhandle,
    mut jpegBuf: *mut libc::c_uchar,
    mut jpegSize: libc::c_ulong,
    mut dstBuf: *mut libc::c_uchar,
    mut flags: libc::c_int,
) -> libc::c_int {
    return tjDecompressToYUV2(
        handle,
        jpegBuf,
        jpegSize,
        dstBuf,
        0 as libc::c_int,
        4 as libc::c_int,
        0 as libc::c_int,
        flags,
    );
}
/* *
 * Create a new TurboJPEG transformer instance.
 *
 * @return a handle to the newly-created instance, or NULL if an error
 * occurred (see #tjGetErrorStr2().)
 */
/* Transformer */
#[no_mangle]

pub unsafe extern "C" fn tjInitTransform() -> crate::src::turbojpeg::tjhandle {
    let mut this: *mut tjinstance = crate::stddef_h::NULL as *mut tjinstance;
    let mut handle: crate::src::turbojpeg::tjhandle = crate::stddef_h::NULL as *mut libc::c_void;
    this = crate::stdlib::malloc(::std::mem::size_of::<tjinstance>() as libc::c_ulong)
        as *mut tjinstance;
    if this.is_null() {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"tjInitTransform(): Memory allocation failure\x00" as *const u8 as *const libc::c_char,
        );
        return crate::stddef_h::NULL as *mut libc::c_void;
    }
    crate::stdlib::memset(
        this as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<tjinstance>() as libc::c_ulong,
    );
    crate::stdlib::snprintf(
        (*this).errStr.as_mut_ptr(),
        crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
        b"No error\x00" as *const u8 as *const libc::c_char,
    );
    handle = _tjInitCompress(this);
    if handle.is_null() {
        return crate::stddef_h::NULL as *mut libc::c_void;
    }
    handle = _tjInitDecompress(this);
    return handle;
}
/* *
 * Losslessly transform a JPEG image into another JPEG image.  Lossless
 * transforms work by moving the raw DCT coefficients from one JPEG image
 * structure to another without altering the values of the coefficients.  While
 * this is typically faster than decompressing the image, transforming it, and
 * re-compressing it, lossless transforms are not free.  Each lossless
 * transform requires reading and performing Huffman decoding on all of the
 * coefficients in the source image, regardless of the size of the destination
 * image.  Thus, this function provides a means of generating multiple
 * transformed images from the same source or  applying multiple
 * transformations simultaneously, in order to eliminate the need to read the
 * source coefficients multiple times.
 *
 * @param handle a handle to a TurboJPEG transformer instance
 *
 * @param jpegBuf pointer to a buffer containing the JPEG source image to
 * transform
 *
 * @param jpegSize size of the JPEG source image (in bytes)
 *
 * @param n the number of transformed JPEG images to generate
 *
 * @param dstBufs pointer to an array of n image buffers.  <tt>dstBufs[i]</tt>
 * will receive a JPEG image that has been transformed using the parameters in
 * <tt>transforms[i]</tt>.  TurboJPEG has the ability to reallocate the JPEG
 * buffer to accommodate the size of the JPEG image.  Thus, you can choose to:
 * -# pre-allocate the JPEG buffer with an arbitrary size using #tjAlloc() and
 * let TurboJPEG grow the buffer as needed,
 * -# set <tt>dstBufs[i]</tt> to NULL to tell TurboJPEG to allocate the buffer
 * for you, or
 * -# pre-allocate the buffer to a "worst case" size determined by calling
 * #tjBufSize() with the transformed or cropped width and height.  Under normal
 * circumstances, this should ensure that the buffer never has to be
 * re-allocated (setting #TJFLAG_NOREALLOC guarantees that it won't be.)  Note,
 * however, that there are some rare cases (such as transforming images with a
 * large amount of embedded EXIF or ICC profile data) in which the output image
 * will be larger than the worst-case size, and #TJFLAG_NOREALLOC cannot be
 * used in those cases.
 * .
 * If you choose option 1, <tt>dstSizes[i]</tt> should be set to the size of
 * your pre-allocated buffer.  In any case, unless you have set
 * #TJFLAG_NOREALLOC, you should always check <tt>dstBufs[i]</tt> upon return
 * from this function, as it may have changed.
 *
 * @param dstSizes pointer to an array of n unsigned long variables that will
 * receive the actual sizes (in bytes) of each transformed JPEG image.  If
 * <tt>dstBufs[i]</tt> points to a pre-allocated buffer, then
 * <tt>dstSizes[i]</tt> should be set to the size of the buffer.  Upon return,
 * <tt>dstSizes[i]</tt> will contain the size of the JPEG image (in bytes.)
 *
 * @param transforms pointer to an array of n #tjtransform structures, each of
 * which specifies the transform parameters and/or cropping region for the
 * corresponding transformed output image.
 *
 * @param flags the bitwise OR of one or more of the @ref TJFLAG_ACCURATEDCT
 * "flags"
 *
 * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2()
 * and #tjGetErrorCode().)
 */
#[no_mangle]

pub unsafe extern "C" fn tjTransform(
    mut handle: crate::src::turbojpeg::tjhandle,
    mut jpegBuf: *const libc::c_uchar,
    mut jpegSize: libc::c_ulong,
    mut n: libc::c_int,
    mut dstBufs: *mut *mut libc::c_uchar,
    mut dstSizes: *mut libc::c_ulong,
    mut t: *mut crate::src::turbojpeg::tjtransform,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut xinfo: *mut crate::src::transupp::jpeg_transform_info =
        crate::stddef_h::NULL as *mut crate::src::transupp::jpeg_transform_info;
    let mut srccoefs: *mut crate::jpeglib_h::jvirt_barray_ptr =
        0 as *mut crate::jpeglib_h::jvirt_barray_ptr;
    let mut dstcoefs: *mut crate::jpeglib_h::jvirt_barray_ptr =
        0 as *mut crate::jpeglib_h::jvirt_barray_ptr;
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut jpegSubsamp: libc::c_int = 0;
    let mut saveMarkers: libc::c_int = 0 as libc::c_int;
    let mut this: *mut tjinstance = handle as *mut tjinstance;
    let mut cinfo: crate::jpeglib_h::j_compress_ptr =
        crate::stddef_h::NULL as crate::jpeglib_h::j_compress_ptr;
    let mut dinfo: crate::jpeglib_h::j_decompress_ptr =
        crate::stddef_h::NULL as crate::jpeglib_h::j_decompress_ptr;
    if this.is_null() {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"Invalid handle\x00" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    cinfo = &mut (*this).cinfo;
    dinfo = &mut (*this).dinfo;
    (*this).jerr.warning = crate::jmorecfg_h::FALSE;
    (*this).isInstanceError = crate::jmorecfg_h::FALSE;
    (*this).jerr.stopOnWarning = if flags & crate::src::turbojpeg::TJFLAG_STOPONWARNING != 0 {
        crate::jmorecfg_h::TRUE
    } else {
        crate::jmorecfg_h::FALSE
    };
    if (*this).init & COMPRESS as libc::c_int == 0 as libc::c_int
        || (*this).init & DECOMPRESS as libc::c_int == 0 as libc::c_int
    {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjTransform(): Instance has not been initialized for transformation\x00" as *const u8
                as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjTransform(): Instance has not been initialized for transformation\x00" as *const u8
                as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else if jpegBuf.is_null()
        || jpegSize <= 0 as libc::c_int as libc::c_ulong
        || n < 1 as libc::c_int
        || dstBufs.is_null()
        || dstSizes.is_null()
        || t.is_null()
        || flags < 0 as libc::c_int
    {
        crate::stdlib::snprintf(
            (*this).errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjTransform(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        (*this).isInstanceError = crate::jmorecfg_h::TRUE;
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjTransform(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else {
        if flags & crate::src::turbojpeg::TJFLAG_FORCEMMX != 0 {
            crate::stdlib::putenv(
                b"JSIMD_FORCEMMX=1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if flags & crate::src::turbojpeg::TJFLAG_FORCESSE != 0 {
            crate::stdlib::putenv(
                b"JSIMD_FORCESSE=1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if flags & crate::src::turbojpeg::TJFLAG_FORCESSE2 != 0 {
            crate::stdlib::putenv(
                b"JSIMD_FORCESSE2=1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        xinfo = crate::stdlib::malloc(
            (::std::mem::size_of::<crate::src::transupp::jpeg_transform_info>() as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong),
        ) as *mut crate::src::transupp::jpeg_transform_info;
        if xinfo.is_null() {
            crate::stdlib::snprintf(
                (*this).errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                b"tjTransform(): Memory allocation failure\x00" as *const u8 as *const libc::c_char,
            );
            (*this).isInstanceError = crate::jmorecfg_h::TRUE;
            crate::stdlib::snprintf(
                errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\x00" as *const u8 as *const libc::c_char,
                b"tjTransform(): Memory allocation failure\x00" as *const u8 as *const libc::c_char,
            );
            retval = -(1 as libc::c_int)
        } else {
            crate::stdlib::memset(
                xinfo as *mut libc::c_void,
                0 as libc::c_int,
                (::std::mem::size_of::<crate::src::transupp::jpeg_transform_info>()
                    as libc::c_ulong)
                    .wrapping_mul(n as libc::c_ulong),
            );
            if crate::stdlib::_setjmp((*this).jerr.setjmp_buffer.as_mut_ptr()) != 0 {
                /* If we get here, the JPEG code has signaled an error. */
                retval = -(1 as libc::c_int)
            } else {
                jpeg_mem_src_tj(dinfo, jpegBuf, jpegSize);
                i = 0 as libc::c_int;
                while i < n {
                    (*xinfo.offset(i as isize)).transform =
                        xformtypes[(*t.offset(i as isize)).op as usize];
                    (*xinfo.offset(i as isize)).perfect = if (*t.offset(i as isize)).options
                        & crate::src::turbojpeg::TJXOPT_PERFECT
                        != 0
                    {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                    (*xinfo.offset(i as isize)).trim = if (*t.offset(i as isize)).options
                        & crate::src::turbojpeg::TJXOPT_TRIM
                        != 0
                    {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                    (*xinfo.offset(i as isize)).force_grayscale = if (*t.offset(i as isize)).options
                        & crate::src::turbojpeg::TJXOPT_GRAY
                        != 0
                    {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                    (*xinfo.offset(i as isize)).crop = if (*t.offset(i as isize)).options
                        & crate::src::turbojpeg::TJXOPT_CROP
                        != 0
                    {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                    if n != 1 as libc::c_int
                        && (*t.offset(i as isize)).op
                            == crate::src::turbojpeg::TJXOP_HFLIP as libc::c_int
                    {
                        (*xinfo.offset(i as isize)).slow_hflip = 1 as libc::c_int
                    } else {
                        (*xinfo.offset(i as isize)).slow_hflip = 0 as libc::c_int
                    }
                    if (*xinfo.offset(i as isize)).crop != 0 {
                        (*xinfo.offset(i as isize)).crop_xoffset =
                            (*t.offset(i as isize)).r.x as crate::jmorecfg_h::JDIMENSION;
                        (*xinfo.offset(i as isize)).crop_xoffset_set =
                            crate::src::transupp::JCROP_POS;
                        (*xinfo.offset(i as isize)).crop_yoffset =
                            (*t.offset(i as isize)).r.y as crate::jmorecfg_h::JDIMENSION;
                        (*xinfo.offset(i as isize)).crop_yoffset_set =
                            crate::src::transupp::JCROP_POS;
                        if (*t.offset(i as isize)).r.w != 0 as libc::c_int {
                            (*xinfo.offset(i as isize)).crop_width =
                                (*t.offset(i as isize)).r.w as crate::jmorecfg_h::JDIMENSION;
                            (*xinfo.offset(i as isize)).crop_width_set =
                                crate::src::transupp::JCROP_POS
                        } else {
                            (*xinfo.offset(i as isize)).crop_width =
                                crate::src::transupp::JCROP_UNSET as libc::c_int
                                    as crate::jmorecfg_h::JDIMENSION
                        }
                        if (*t.offset(i as isize)).r.h != 0 as libc::c_int {
                            (*xinfo.offset(i as isize)).crop_height =
                                (*t.offset(i as isize)).r.h as crate::jmorecfg_h::JDIMENSION;
                            (*xinfo.offset(i as isize)).crop_height_set =
                                crate::src::transupp::JCROP_POS
                        } else {
                            (*xinfo.offset(i as isize)).crop_height =
                                crate::src::transupp::JCROP_UNSET as libc::c_int
                                    as crate::jmorecfg_h::JDIMENSION
                        }
                    }
                    if (*t.offset(i as isize)).options & crate::src::turbojpeg::TJXOPT_COPYNONE == 0
                    {
                        saveMarkers = 1 as libc::c_int
                    }
                    i += 1
                }
                crate::src::transupp::jcopy_markers_setup(
                    dinfo,
                    if saveMarkers != 0 {
                        crate::src::transupp::JCOPYOPT_ALL as libc::c_int
                    } else {
                        crate::src::transupp::JCOPYOPT_NONE as libc::c_int
                    } as crate::src::transupp::JCOPY_OPTION,
                );
                crate::src::jdapimin::jpeg_read_header(dinfo, crate::jmorecfg_h::TRUE);
                jpegSubsamp = getSubsamp(dinfo);
                if jpegSubsamp < 0 as libc::c_int {
                    crate::stdlib::snprintf(
                        (*this).errStr.as_mut_ptr(),
                        crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        b"tjTransform(): Could not determine subsampling type for JPEG image\x00"
                            as *const u8 as *const libc::c_char,
                    );
                    (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                    crate::stdlib::snprintf(
                        errStr.as_mut_ptr(),
                        crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        b"tjTransform(): Could not determine subsampling type for JPEG image\x00"
                            as *const u8 as *const libc::c_char,
                    );
                    retval = -(1 as libc::c_int)
                } else {
                    i = 0 as libc::c_int;
                    loop {
                        if !(i < n) {
                            current_block = 1852451392920375136;
                            break;
                        }
                        if crate::src::transupp::jtransform_request_workspace(
                            dinfo,
                            &mut *xinfo.offset(i as isize),
                        ) == 0
                        {
                            crate::stdlib::snprintf(
                                (*this).errStr.as_mut_ptr(),
                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                b"%s\x00" as *const u8 as *const libc::c_char,
                                b"tjTransform(): Transform is not perfect\x00" as *const u8
                                    as *const libc::c_char,
                            );
                            (*this).isInstanceError = crate::jmorecfg_h::TRUE;
                            crate::stdlib::snprintf(
                                errStr.as_mut_ptr(),
                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                b"%s\x00" as *const u8 as *const libc::c_char,
                                b"tjTransform(): Transform is not perfect\x00" as *const u8
                                    as *const libc::c_char,
                            );
                            retval = -(1 as libc::c_int);
                            current_block = 1538248363559331145;
                            break;
                        } else {
                            if (*xinfo.offset(i as isize)).crop != 0 {
                                if (*t.offset(i as isize)).r.x
                                    % (*xinfo.offset(i as isize)).iMCU_sample_width
                                    != 0 as libc::c_int
                                    || (*t.offset(i as isize)).r.y
                                        % (*xinfo.offset(i as isize)).iMCU_sample_height
                                        != 0 as libc::c_int
                                {
                                    crate::stdlib::snprintf(errStr.as_mut_ptr(),
                                             crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                             b"To crop this JPEG image, x must be a multiple of %d\nand y must be a multiple of %d.\n\x00"
                                                 as *const u8 as
                                                 *const libc::c_char,
                                             (*xinfo.offset(i as
                                                                isize)).iMCU_sample_width,
                                             (*xinfo.offset(i as
                                                                isize)).iMCU_sample_height);
                                    retval = -(1 as libc::c_int);
                                    current_block = 1538248363559331145;
                                    break;
                                }
                            }
                            i += 1
                        }
                    }
                    match current_block {
                        1538248363559331145 => {}
                        _ => {
                            srccoefs = crate::src::jdtrans::jpeg_read_coefficients(dinfo);
                            i = 0 as libc::c_int;
                            's_452: loop {
                                if !(i < n) {
                                    current_block = 7761909515536616543;
                                    break;
                                }
                                let mut w: libc::c_int = 0;
                                let mut h: libc::c_int = 0;
                                let mut alloc: libc::c_int = 1 as libc::c_int;
                                if (*xinfo.offset(i as isize)).crop == 0 {
                                    w = (*dinfo).image_width as libc::c_int;
                                    h = (*dinfo).image_height as libc::c_int
                                } else {
                                    w = (*xinfo.offset(i as isize)).crop_width as libc::c_int;
                                    h = (*xinfo.offset(i as isize)).crop_height as libc::c_int
                                }
                                if flags & crate::src::turbojpeg::TJFLAG_NOREALLOC != 0 {
                                    alloc = 0 as libc::c_int;
                                    *dstSizes.offset(i as isize) = tjBufSize(w, h, jpegSubsamp)
                                }
                                if (*t.offset(i as isize)).options
                                    & crate::src::turbojpeg::TJXOPT_NOOUTPUT
                                    == 0
                                {
                                    jpeg_mem_dest_tj(
                                        cinfo,
                                        &mut *dstBufs.offset(i as isize),
                                        &mut *dstSizes.offset(i as isize),
                                        alloc,
                                    );
                                }
                                crate::src::jctrans::jpeg_copy_critical_parameters(dinfo, cinfo);
                                dstcoefs = crate::src::transupp::jtransform_adjust_parameters(
                                    dinfo,
                                    cinfo,
                                    srccoefs,
                                    &mut *xinfo.offset(i as isize),
                                );
                                if flags & crate::src::turbojpeg::TJFLAG_PROGRESSIVE != 0
                                    || (*t.offset(i as isize)).options
                                        & crate::src::turbojpeg::TJXOPT_PROGRESSIVE
                                        != 0
                                {
                                    crate::src::jcparam::jpeg_simple_progression(cinfo);
                                }
                                if (*t.offset(i as isize)).options
                                    & crate::src::turbojpeg::TJXOPT_NOOUTPUT
                                    == 0
                                {
                                    crate::src::jctrans::jpeg_write_coefficients(cinfo, dstcoefs);
                                    crate::src::transupp::jcopy_markers_execute(
                                        dinfo,
                                        cinfo,
                                        if (*t.offset(i as isize)).options
                                            & crate::src::turbojpeg::TJXOPT_COPYNONE
                                            != 0
                                        {
                                            crate::src::transupp::JCOPYOPT_NONE as libc::c_int
                                        } else {
                                            crate::src::transupp::JCOPYOPT_ALL as libc::c_int
                                        }
                                            as crate::src::transupp::JCOPY_OPTION,
                                    );
                                } else {
                                    crate::src::jcmaster::jinit_c_master_control(
                                        cinfo,
                                        crate::jmorecfg_h::TRUE,
                                    );
                                }
                                crate::src::transupp::jtransform_execute_transform(
                                    dinfo,
                                    cinfo,
                                    srccoefs,
                                    &mut *xinfo.offset(i as isize),
                                );
                                if (*t.offset(i as isize)).customFilter.is_some() {
                                    let mut ci: libc::c_int = 0;
                                    let mut y: libc::c_int = 0;
                                    let mut by: crate::jmorecfg_h::JDIMENSION = 0;
                                    ci = 0 as libc::c_int;
                                    while ci < (*cinfo).num_components {
                                        let mut compptr:
                                                    *mut crate::jpeglib_h::jpeg_component_info =
                                                &mut *(*cinfo).comp_info.offset(ci
                                                                                    as
                                                                                    isize)
                                                    as
                                                    *mut crate::jpeglib_h::jpeg_component_info;
                                        let mut arrayRegion: crate::src::turbojpeg::tjregion = {
                                            let mut init = crate::src::turbojpeg::tjregion {
                                                x: 0 as libc::c_int,
                                                y: 0 as libc::c_int,
                                                w: (*compptr).width_in_blocks.wrapping_mul(
                                                    crate::jpeglib_h::DCTSIZE as libc::c_uint,
                                                )
                                                    as libc::c_int,
                                                h: crate::jpeglib_h::DCTSIZE,
                                            };
                                            init
                                        };
                                        let mut planeRegion: crate::src::turbojpeg::tjregion = {
                                            let mut init = crate::src::turbojpeg::tjregion {
                                                x: 0 as libc::c_int,
                                                y: 0 as libc::c_int,
                                                w: (*compptr).width_in_blocks.wrapping_mul(
                                                    crate::jpeglib_h::DCTSIZE as libc::c_uint,
                                                )
                                                    as libc::c_int,
                                                h: (*compptr).height_in_blocks.wrapping_mul(
                                                    crate::jpeglib_h::DCTSIZE as libc::c_uint,
                                                )
                                                    as libc::c_int,
                                            };
                                            init
                                        };
                                        by = 0 as libc::c_int as crate::jmorecfg_h::JDIMENSION;
                                        while by < (*compptr).height_in_blocks {
                                            let mut barray: crate::jpeglib_h::JBLOCKARRAY =
                                                (*(*dinfo).mem)
                                                    .access_virt_barray
                                                    .expect("non-null function pointer")(
                                                    dinfo as crate::jpeglib_h::j_common_ptr,
                                                    *dstcoefs.offset(ci as isize),
                                                    by,
                                                    (*compptr).v_samp_factor
                                                        as crate::jmorecfg_h::JDIMENSION,
                                                    crate::jmorecfg_h::TRUE,
                                                );
                                            y = 0 as libc::c_int;
                                            while y < (*compptr).v_samp_factor {
                                                if (*t.offset(i as isize))
                                                    .customFilter
                                                    .expect("non-null function pointer")(
                                                    (*(*barray.offset(y as isize))
                                                        .offset(0 as libc::c_int as isize))
                                                    .as_mut_ptr(),
                                                    arrayRegion,
                                                    planeRegion,
                                                    ci,
                                                    i,
                                                    &mut *t.offset(i as isize),
                                                ) == -(1 as libc::c_int)
                                                {
                                                    crate::stdlib::snprintf(
                                                        (*this).errStr.as_mut_ptr(),
                                                        crate::jpeglib_h::JMSG_LENGTH_MAX
                                                            as libc::c_ulong,
                                                        b"%s\x00" as *const u8
                                                            as *const libc::c_char,
                                                        b"tjTransform(): Error in custom filter\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                    (*this).isInstanceError =
                                                        crate::jmorecfg_h::TRUE;
                                                    crate::stdlib::snprintf(
                                                        errStr.as_mut_ptr(),
                                                        crate::jpeglib_h::JMSG_LENGTH_MAX
                                                            as libc::c_ulong,
                                                        b"%s\x00" as *const u8
                                                            as *const libc::c_char,
                                                        b"tjTransform(): Error in custom filter\x00"
                                                            as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                    retval = -(1 as libc::c_int);
                                                    current_block = 1538248363559331145;
                                                    break 's_452;
                                                } else {
                                                    arrayRegion.y += crate::jpeglib_h::DCTSIZE;
                                                    y += 1
                                                }
                                            }
                                            by = (by as libc::c_uint).wrapping_add(
                                                (*compptr).v_samp_factor as libc::c_uint,
                                            )
                                                as crate::jmorecfg_h::JDIMENSION
                                                as crate::jmorecfg_h::JDIMENSION
                                        }
                                        ci += 1
                                    }
                                }
                                if (*t.offset(i as isize)).options
                                    & crate::src::turbojpeg::TJXOPT_NOOUTPUT
                                    == 0
                                {
                                    crate::src::jcapimin::jpeg_finish_compress(cinfo);
                                }
                                i += 1
                            }
                            match current_block {
                                1538248363559331145 => {}
                                _ => {
                                    crate::src::jdapimin::jpeg_finish_decompress(dinfo);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if (*cinfo).global_state > crate::jpegint_h::CSTATE_START {
        crate::src::jcapimin::jpeg_abort_compress(cinfo);
    }
    if (*dinfo).global_state > crate::jpegint_h::DSTATE_START {
        crate::src::jdapimin::jpeg_abort_decompress(dinfo);
    }
    if !xinfo.is_null() {
        crate::stdlib::free(xinfo as *mut libc::c_void);
    }
    if (*this).jerr.warning != 0 {
        retval = -(1 as libc::c_int)
    }
    (*this).jerr.stopOnWarning = crate::jmorecfg_h::FALSE;
    return retval;
}
/* *
 * Load an uncompressed image from disk into memory.
 *
 * @param filename name of a file containing an uncompressed image in Windows
 * BMP or PBMPLUS (PPM/PGM) format
 *
 * @param width pointer to an integer variable that will receive the width (in
 * pixels) of the uncompressed image
 *
 * @param align row alignment of the image buffer to be returned (must be a
 * power of 2.)  For instance, setting this parameter to 4 will cause all rows
 * in the image buffer to be padded to the nearest 32-bit boundary, and setting
 * this parameter to 1 will cause all rows in the image buffer to be unpadded.
 *
 * @param height pointer to an integer variable that will receive the height
 * (in pixels) of the uncompressed image
 *
 * @param pixelFormat pointer to an integer variable that specifies or will
 * receive the pixel format of the uncompressed image buffer.  The behavior of
 * #tjLoadImage() will vary depending on the value of <tt>*pixelFormat</tt>
 * passed to the function:
 * - @ref TJPF_UNKNOWN : The uncompressed image buffer returned by the function
 * will use the most optimal pixel format for the file type, and
 * <tt>*pixelFormat</tt> will contain the ID of this pixel format upon
 * successful return from the function.
 * - @ref TJPF_GRAY : Only PGM files and 8-bit BMP files with a grayscale
 * colormap can be loaded.
 * - @ref TJPF_CMYK : The RGB or grayscale pixels stored in the file will be
 * converted using a quick & dirty algorithm that is suitable only for testing
 * purposes (proper conversion between CMYK and other formats requires a color
 * management system.)
 * - Other @ref TJPF "pixel formats" : The uncompressed image buffer will use
 * the specified pixel format, and pixel format conversion will be performed if
 * necessary.
 *
 * @param flags the bitwise OR of one or more of the @ref TJFLAG_BOTTOMUP
 * "flags".
 *
 * @return a pointer to a newly-allocated buffer containing the uncompressed
 * image, converted to the chosen pixel format and with the chosen row
 * alignment, or NULL if an error occurred (see #tjGetErrorStr2().)  This
 * buffer should be freed using #tjFree().
 */
#[no_mangle]

pub unsafe extern "C" fn tjLoadImage(
    mut filename: *const libc::c_char,
    mut width: *mut libc::c_int,
    mut align: libc::c_int,
    mut height: *mut libc::c_int,
    mut pixelFormat: *mut libc::c_int,
    mut flags: libc::c_int,
) -> *mut libc::c_uchar {
    let mut current_block: u64;
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut tempc: libc::c_int = 0;
    let mut pitch: libc::c_int = 0;
    let mut handle: crate::src::turbojpeg::tjhandle = crate::stddef_h::NULL as *mut libc::c_void;
    let mut this: *mut tjinstance = 0 as *mut tjinstance;
    let mut cinfo: crate::jpeglib_h::j_compress_ptr =
        crate::stddef_h::NULL as crate::jpeglib_h::j_compress_ptr;
    let mut src: crate::cdjpeg_h::cjpeg_source_ptr = 0 as *mut crate::cdjpeg_h::cjpeg_source_struct;
    let mut dstBuf: *mut libc::c_uchar = crate::stddef_h::NULL as *mut libc::c_uchar;
    let mut file: *mut crate::stdlib::FILE = crate::stddef_h::NULL as *mut crate::stdlib::FILE;
    let mut invert: crate::jmorecfg_h::boolean = 0;
    if filename.is_null()
        || width.is_null()
        || align < 1 as libc::c_int
        || height.is_null()
        || pixelFormat.is_null()
        || *pixelFormat < crate::src::turbojpeg::TJPF_UNKNOWN as libc::c_int
        || *pixelFormat >= crate::src::turbojpeg::TJ_NUMPF
    {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjLoadImage(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else if align & align - 1 as libc::c_int != 0 as libc::c_int {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjLoadImage(): Alignment must be a power of 2\x00" as *const u8
                as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else {
        handle = tjInitCompress();
        if handle.is_null() {
            return crate::stddef_h::NULL as *mut libc::c_uchar;
        }
        this = handle as *mut tjinstance;
        cinfo = &mut (*this).cinfo;
        file = crate::stdlib::fopen(filename, b"rb\x00" as *const u8 as *const libc::c_char);
        if file.is_null() {
            crate::stdlib::snprintf(
                errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\n%s\x00" as *const u8 as *const libc::c_char,
                b"tjLoadImage(): Cannot open input file\x00" as *const u8 as *const libc::c_char,
                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
            );
            retval = -(1 as libc::c_int)
        } else {
            tempc = crate::stdlib::getc(file);
            if tempc < 0 as libc::c_int || crate::stdlib::ungetc(tempc, file) == crate::stdlib::EOF
            {
                crate::stdlib::snprintf(
                    errStr.as_mut_ptr(),
                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                    b"%s\n%s\x00" as *const u8 as *const libc::c_char,
                    b"tjLoadImage(): Could not read input file\x00" as *const u8
                        as *const libc::c_char,
                    crate::stdlib::strerror(*crate::stdlib::__errno_location()),
                );
                retval = -(1 as libc::c_int)
            } else if tempc == crate::stdlib::EOF {
                crate::stdlib::snprintf(
                    errStr.as_mut_ptr(),
                    crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    b"tjLoadImage(): Input file contains no data\x00" as *const u8
                        as *const libc::c_char,
                );
                retval = -(1 as libc::c_int)
            } else if crate::stdlib::_setjmp((*this).jerr.setjmp_buffer.as_mut_ptr()) != 0 {
                /* If we get here, the JPEG code has signaled an error. */
                retval = -(1 as libc::c_int)
            } else {
                if *pixelFormat == crate::src::turbojpeg::TJPF_UNKNOWN as libc::c_int {
                    (*cinfo).in_color_space = crate::jpeglib_h::JCS_UNKNOWN
                } else {
                    (*cinfo).in_color_space = pf2cs[*pixelFormat as usize]
                }
                if tempc == 'B' as i32 {
                    src = crate::src::rdbmp::jinit_read_bmp(cinfo, crate::jmorecfg_h::FALSE);
                    if src.is_null() {
                        crate::stdlib::snprintf(
                            errStr.as_mut_ptr(),
                            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            b"tjLoadImage(): Could not initialize bitmap loader\x00" as *const u8
                                as *const libc::c_char,
                        );
                        retval = -(1 as libc::c_int);
                        current_block = 7323915231371106940;
                    } else {
                        invert = (flags & crate::src::turbojpeg::TJFLAG_BOTTOMUP
                            == 0 as libc::c_int) as libc::c_int;
                        current_block = 3689906465960840878;
                    }
                } else if tempc == 'P' as i32 {
                    src = crate::src::rdppm::jinit_read_ppm(cinfo);
                    if src.is_null() {
                        crate::stdlib::snprintf(
                            errStr.as_mut_ptr(),
                            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            b"tjLoadImage(): Could not initialize bitmap loader\x00" as *const u8
                                as *const libc::c_char,
                        );
                        retval = -(1 as libc::c_int);
                        current_block = 7323915231371106940;
                    } else {
                        invert = (flags & crate::src::turbojpeg::TJFLAG_BOTTOMUP
                            != 0 as libc::c_int) as libc::c_int;
                        current_block = 3689906465960840878;
                    }
                } else {
                    crate::stdlib::snprintf(
                        errStr.as_mut_ptr(),
                        crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        b"tjLoadImage(): Unsupported file type\x00" as *const u8
                            as *const libc::c_char,
                    );
                    retval = -(1 as libc::c_int);
                    current_block = 7323915231371106940;
                }
                match current_block {
                    7323915231371106940 => {}
                    _ => {
                        (*src).input_file = file;
                        Some((*src).start_input.expect("non-null function pointer"))
                            .expect("non-null function pointer")(cinfo, src);
                        Some(
                            (*(*cinfo).mem)
                                .realize_virt_arrays
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            cinfo as crate::jpeglib_h::j_common_ptr,
                        );
                        *width = (*cinfo).image_width as libc::c_int;
                        *height = (*cinfo).image_height as libc::c_int;
                        *pixelFormat = cs2pf[(*cinfo).in_color_space as usize];
                        pitch = *width * tjPixelSize[*pixelFormat as usize] + align
                            - 1 as libc::c_int
                            & !(align - 1 as libc::c_int);
                        dstBuf = crate::stdlib::malloc((pitch * *height) as libc::c_ulong)
                            as *mut libc::c_uchar;
                        if dstBuf.is_null() {
                            crate::stdlib::snprintf(
                                errStr.as_mut_ptr(),
                                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                                b"%s\x00" as *const u8 as *const libc::c_char,
                                b"tjLoadImage(): Memory allocation failure\x00" as *const u8
                                    as *const libc::c_char,
                            );
                            retval = -(1 as libc::c_int)
                        } else if crate::stdlib::_setjmp((*this).jerr.setjmp_buffer.as_mut_ptr())
                            != 0
                        {
                            /* If we get here, the JPEG code has signaled an error. */
                            retval = -(1 as libc::c_int)
                        } else {
                            while (*cinfo).next_scanline < (*cinfo).image_height {
                                let mut i: libc::c_int = 0;
                                let mut nlines: libc::c_int =
                                    Some((*src).get_pixel_rows.expect("non-null function pointer"))
                                        .expect("non-null function pointer")(
                                        cinfo, src
                                    ) as libc::c_int;
                                i = 0 as libc::c_int;
                                while i < nlines {
                                    let mut dstptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                                    let mut row: libc::c_int = 0;
                                    row = (*cinfo).next_scanline.wrapping_add(i as libc::c_uint)
                                        as libc::c_int;
                                    if invert != 0 {
                                        dstptr = &mut *dstBuf.offset(
                                            ((*height - row - 1 as libc::c_int) * pitch) as isize,
                                        )
                                            as *mut libc::c_uchar
                                    } else {
                                        dstptr = &mut *dstBuf.offset((row * pitch) as isize)
                                            as *mut libc::c_uchar
                                    }
                                    crate::stdlib::memcpy(
                                        dstptr as *mut libc::c_void,
                                        *(*src).buffer.offset(i as isize) as *const libc::c_void,
                                        (*width * tjPixelSize[*pixelFormat as usize])
                                            as libc::c_ulong,
                                    );
                                    i += 1
                                }
                                (*cinfo).next_scanline = ((*cinfo).next_scanline as libc::c_uint)
                                    .wrapping_add(nlines as libc::c_uint)
                                    as crate::jmorecfg_h::JDIMENSION
                                    as crate::jmorecfg_h::JDIMENSION
                            }
                            Some((*src).finish_input.expect("non-null function pointer"))
                                .expect("non-null function pointer")(
                                cinfo, src
                            );
                        }
                    }
                }
            }
        }
    }
    if !handle.is_null() {
        tjDestroy(handle);
    }
    if !file.is_null() {
        crate::stdlib::fclose(file);
    }
    if retval < 0 as libc::c_int && !dstBuf.is_null() {
        crate::stdlib::free(dstBuf as *mut libc::c_void);
        dstBuf = crate::stddef_h::NULL as *mut libc::c_uchar
    }
    return dstBuf;
}
/* *
 * Save an uncompressed image from memory to disk.
 *
 * @param filename name of a file to which to save the uncompressed image.
 * The image will be stored in Windows BMP or PBMPLUS (PPM/PGM) format,
 * depending on the file extension.
 *
 * @param buffer pointer to an image buffer containing RGB, grayscale, or
 * CMYK pixels to be saved
 *
 * @param width width (in pixels) of the uncompressed image
 *
 * @param pitch bytes per line in the image buffer.  Setting this parameter to
 * 0 is the equivalent of setting it to
 * <tt>width * #tjPixelSize[pixelFormat]</tt>.
 *
 * @param height height (in pixels) of the uncompressed image
 *
 * @param pixelFormat pixel format of the image buffer (see @ref TJPF
 * "Pixel formats".)  If this parameter is set to @ref TJPF_GRAY, then the
 * image will be stored in PGM or 8-bit (indexed color) BMP format.  Otherwise,
 * the image will be stored in PPM or 24-bit BMP format.  If this parameter
 * is set to @ref TJPF_CMYK, then the CMYK pixels will be converted to RGB
 * using a quick & dirty algorithm that is suitable only for testing (proper
 * conversion between CMYK and other formats requires a color management
 * system.)
 *
 * @param flags the bitwise OR of one or more of the @ref TJFLAG_BOTTOMUP
 * "flags".
 *
 * @return 0 if successful, or -1 if an error occurred (see #tjGetErrorStr2().)
 */
#[no_mangle]

pub unsafe extern "C" fn tjSaveImage(
    mut filename: *const libc::c_char,
    mut buffer: *mut libc::c_uchar,
    mut width: libc::c_int,
    mut pitch: libc::c_int,
    mut height: libc::c_int,
    mut pixelFormat: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut handle: crate::src::turbojpeg::tjhandle = crate::stddef_h::NULL as *mut libc::c_void;
    let mut this: *mut tjinstance = 0 as *mut tjinstance;
    let mut dinfo: crate::jpeglib_h::j_decompress_ptr =
        crate::stddef_h::NULL as crate::jpeglib_h::j_decompress_ptr;
    let mut dst: crate::cdjpeg_h::djpeg_dest_ptr = 0 as *mut crate::cdjpeg_h::djpeg_dest_struct;
    let mut file: *mut crate::stdlib::FILE = crate::stddef_h::NULL as *mut crate::stdlib::FILE;
    let mut ptr: *mut libc::c_char = crate::stddef_h::NULL as *mut libc::c_char;
    let mut invert: crate::jmorecfg_h::boolean = 0;
    if filename.is_null()
        || buffer.is_null()
        || width < 1 as libc::c_int
        || pitch < 0 as libc::c_int
        || height < 1 as libc::c_int
        || pixelFormat < 0 as libc::c_int
        || pixelFormat >= crate::src::turbojpeg::TJ_NUMPF
    {
        crate::stdlib::snprintf(
            errStr.as_mut_ptr(),
            crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"tjSaveImage(): Invalid argument\x00" as *const u8 as *const libc::c_char,
        );
        retval = -(1 as libc::c_int)
    } else {
        handle = tjInitDecompress();
        if handle.is_null() {
            return -(1 as libc::c_int);
        }
        this = handle as *mut tjinstance;
        dinfo = &mut (*this).dinfo;
        file = crate::stdlib::fopen(filename, b"wb\x00" as *const u8 as *const libc::c_char);
        if file.is_null() {
            crate::stdlib::snprintf(
                errStr.as_mut_ptr(),
                crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                b"%s\n%s\x00" as *const u8 as *const libc::c_char,
                b"tjSaveImage(): Cannot open output file\x00" as *const u8 as *const libc::c_char,
                crate::stdlib::strerror(*crate::stdlib::__errno_location()),
            );
            retval = -(1 as libc::c_int)
        } else if crate::stdlib::_setjmp((*this).jerr.setjmp_buffer.as_mut_ptr()) != 0 {
            /* If we get here, the JPEG code has signaled an error. */
            retval = -(1 as libc::c_int)
        } else {
            (*this).dinfo.out_color_space = pf2cs[pixelFormat as usize];
            (*dinfo).image_width = width as crate::jmorecfg_h::JDIMENSION;
            (*dinfo).image_height = height as crate::jmorecfg_h::JDIMENSION;
            (*dinfo).global_state = crate::jpegint_h::DSTATE_READY;
            (*dinfo).scale_denom = 1 as libc::c_int as libc::c_uint;
            (*dinfo).scale_num = (*dinfo).scale_denom;
            ptr = crate::stdlib::strrchr(filename, '.' as i32);
            if !ptr.is_null()
                && crate::stdlib::strcasecmp(ptr, b".bmp\x00" as *const u8 as *const libc::c_char)
                    == 0
            {
                dst = crate::src::wrbmp::jinit_write_bmp(
                    dinfo,
                    crate::jmorecfg_h::FALSE,
                    crate::jmorecfg_h::FALSE,
                );
                if dst.is_null() {
                    crate::stdlib::snprintf(
                        errStr.as_mut_ptr(),
                        crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        b"tjSaveImage(): Could not initialize bitmap writer\x00" as *const u8
                            as *const libc::c_char,
                    );
                    retval = -(1 as libc::c_int);
                    current_block = 4226535191933510688;
                } else {
                    invert = (flags & crate::src::turbojpeg::TJFLAG_BOTTOMUP == 0 as libc::c_int)
                        as libc::c_int;
                    current_block = 2604890879466389055;
                }
            } else {
                dst = crate::src::wrppm::jinit_write_ppm(dinfo);
                if dst.is_null() {
                    crate::stdlib::snprintf(
                        errStr.as_mut_ptr(),
                        crate::jpeglib_h::JMSG_LENGTH_MAX as libc::c_ulong,
                        b"%s\x00" as *const u8 as *const libc::c_char,
                        b"tjSaveImage(): Could not initialize PPM writer\x00" as *const u8
                            as *const libc::c_char,
                    );
                    retval = -(1 as libc::c_int);
                    current_block = 4226535191933510688;
                } else {
                    invert = (flags & crate::src::turbojpeg::TJFLAG_BOTTOMUP != 0 as libc::c_int)
                        as libc::c_int;
                    current_block = 2604890879466389055;
                }
            }
            match current_block {
                4226535191933510688 => {}
                _ => {
                    (*dst).output_file = file;
                    Some((*dst).start_output.expect("non-null function pointer"))
                        .expect("non-null function pointer")(dinfo, dst);
                    Some(
                        (*(*dinfo).mem)
                            .realize_virt_arrays
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        dinfo as crate::jpeglib_h::j_common_ptr
                    );
                    if pitch == 0 as libc::c_int {
                        pitch = width * tjPixelSize[pixelFormat as usize]
                    }
                    while (*dinfo).output_scanline < (*dinfo).output_height {
                        let mut rowptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                        if invert != 0 {
                            rowptr = &mut *buffer.offset(
                                (height as libc::c_uint)
                                    .wrapping_sub((*dinfo).output_scanline)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(pitch as libc::c_uint)
                                    as isize,
                            ) as *mut libc::c_uchar
                        } else {
                            rowptr = &mut *buffer.offset(
                                (*dinfo).output_scanline.wrapping_mul(pitch as libc::c_uint)
                                    as isize,
                            ) as *mut libc::c_uchar
                        }
                        crate::stdlib::memcpy(
                            *(*dst).buffer.offset(0 as libc::c_int as isize) as *mut libc::c_void,
                            rowptr as *const libc::c_void,
                            (width * tjPixelSize[pixelFormat as usize]) as libc::c_ulong,
                        );
                        Some((*dst).put_pixel_rows.expect("non-null function pointer"))
                            .expect("non-null function pointer")(
                            dinfo,
                            dst,
                            1 as libc::c_int as crate::jmorecfg_h::JDIMENSION,
                        );
                        (*dinfo).output_scanline = (*dinfo).output_scanline.wrapping_add(1)
                    }
                    Some((*dst).finish_output.expect("non-null function pointer"))
                        .expect("non-null function pointer")(dinfo, dst);
                }
            }
        }
    }
    if !handle.is_null() {
        tjDestroy(handle);
    }
    if !file.is_null() {
        crate::stdlib::fclose(file);
    }
    return retval;
}
