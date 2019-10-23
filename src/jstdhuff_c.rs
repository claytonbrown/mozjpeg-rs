pub unsafe extern "C" fn add_huff_table(
    mut cinfo: crate::jpeglib_h::j_common_ptr,
    mut htblptr: *mut *mut crate::jpeglib_h::JHUFF_TBL,
    mut bits: *const crate::jmorecfg_h::UINT8,
    mut val: *const crate::jmorecfg_h::UINT8,
) {
    
      
    if (*htblptr).is_null() {
        *htblptr = crate::jpeglib_h::jpeg_alloc_huff_table(cinfo)
    } else {
        return;
    }
    /* Copy the number-of-symbols-of-each-code-length counts */
    crate::stdlib::memcpy(
        (**htblptr).bits.as_mut_ptr() as *mut libc::c_void,
        bits as *const libc::c_void,
        ::std::mem::size_of::<[crate::jmorecfg_h::UINT8; 17]>() as libc::c_ulong,
    );
    /* Validate the counts.  We do this here mainly so we can copy the right
     * number of symbols from the val[] array, without risking marching off
     * the end of memory.  jchuff.c will do a more thorough test later.
     */
    
     let mut nsymbols:   libc::c_int =  0i32; let mut len:   libc::c_int =  1i32;
    while len <= 16i32 {
        nsymbols += *bits.offset(len as isize) as libc::c_int;
        len += 1
    }
    if nsymbols < 1i32 || nsymbols > 256i32 {
        (*(*cinfo).err).msg_code = crate::src::jerror::JERR_BAD_HUFF_TABLE as libc::c_int;
        Some(
            (*(*cinfo).err)
                .error_exit
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(cinfo);
    }
    crate::stdlib::memcpy(
        (**htblptr).huffval.as_mut_ptr() as *mut libc::c_void,
        val as *const libc::c_void,
        nsymbols as libc::c_ulong *
    ::std::mem::size_of::<crate::jmorecfg_h::UINT8>() as libc::c_ulong,
    );
    crate::stdlib::memset(
        &mut *(**htblptr).huffval.as_mut_ptr().offset(nsymbols as isize)
            as *mut crate::jmorecfg_h::UINT8 as *mut libc::c_void,
        0i32,
        (256i32 - nsymbols) as libc::c_ulong *
    ::std::mem::size_of::<crate::jmorecfg_h::UINT8>() as libc::c_ulong,
    );
    /* Initialize sent_table FALSE so table will be written to JPEG file. */
    (**htblptr).sent_table = crate::jmorecfg_h::FALSE;
}
pub unsafe extern "C" fn std_huff_tables(mut cinfo: crate::jpeglib_h::j_common_ptr) {
    
     let mut dc_huff_tbl_ptrs:  *mut *mut crate::jpeglib_h::JHUFF_TBL =
    
        ::std::ptr::null_mut::< *mut crate::jpeglib_h::JHUFF_TBL>(); let mut ac_huff_tbl_ptrs:  *mut *mut crate::jpeglib_h::JHUFF_TBL =
    
        ::std::ptr::null_mut::< *mut crate::jpeglib_h::JHUFF_TBL>();
    pub static mut bits_dc_luminance: [crate::jmorecfg_h::UINT8; 17] = [
        0u8,
        0u8,
        1u8,
        5u8,
        1u8,
        1u8,
        1u8,
        1u8,
        1u8,
        1u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
    ];
    pub static mut val_dc_luminance: [crate::jmorecfg_h::UINT8; 12] = [
        0u8,
        1u8,
        2u8,
        3u8,
        4u8,
        5u8,
        6u8,
        7u8,
        8u8,
        9u8,
        10u8,
        11u8,
    ];
    pub static mut bits_dc_chrominance: [crate::jmorecfg_h::UINT8; 17] = [
        0u8,
        0u8,
        3u8,
        1u8,
        1u8,
        1u8,
        1u8,
        1u8,
        1u8,
        1u8,
        1u8,
        1u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
    ];
    pub static mut val_dc_chrominance: [crate::jmorecfg_h::UINT8; 12] = [
        0u8,
        1u8,
        2u8,
        3u8,
        4u8,
        5u8,
        6u8,
        7u8,
        8u8,
        9u8,
        10u8,
        11u8,
    ];
    pub static mut bits_ac_luminance: [crate::jmorecfg_h::UINT8; 17] = [
        0u8,
        0u8,
        2u8,
        1u8,
        3u8,
        3u8,
        2u8,
        4u8,
        3u8,
        5u8,
        5u8,
        4u8,
        4u8,
        0u8,
        0u8,
        1u8,
        0x7du8,
    ];
    pub static mut val_ac_luminance: [crate::jmorecfg_h::UINT8; 162] = [
        0x1u8,
        0x2u8,
        0x3u8,
        0u8,
        0x4u8,
        0x11u8,
        0x5u8,
        0x12u8,
        0x21u8,
        0x31u8,
        0x41u8,
        0x6u8,
        0x13u8,
        0x51u8,
        0x61u8,
        0x7u8,
        0x22u8,
        0x71u8,
        0x14u8,
        0x32u8,
        0x81u8,
        0x91u8,
        0xa1u8,
        0x8u8,
        0x23u8,
        0x42u8,
        0xb1u8,
        0xc1u8,
        0x15u8,
        0x52u8,
        0xd1u8,
        0xf0u8,
        0x24u8,
        0x33u8,
        0x62u8,
        0x72u8,
        0x82u8,
        0x9u8,
        0xau8,
        0x16u8,
        0x17u8,
        0x18u8,
        0x19u8,
        0x1au8,
        0x25u8,
        0x26u8,
        0x27u8,
        0x28u8,
        0x29u8,
        0x2au8,
        0x34u8,
        0x35u8,
        0x36u8,
        0x37u8,
        0x38u8,
        0x39u8,
        0x3au8,
        0x43u8,
        0x44u8,
        0x45u8,
        0x46u8,
        0x47u8,
        0x48u8,
        0x49u8,
        0x4au8,
        0x53u8,
        0x54u8,
        0x55u8,
        0x56u8,
        0x57u8,
        0x58u8,
        0x59u8,
        0x5au8,
        0x63u8,
        0x64u8,
        0x65u8,
        0x66u8,
        0x67u8,
        0x68u8,
        0x69u8,
        0x6au8,
        0x73u8,
        0x74u8,
        0x75u8,
        0x76u8,
        0x77u8,
        0x78u8,
        0x79u8,
        0x7au8,
        0x83u8,
        0x84u8,
        0x85u8,
        0x86u8,
        0x87u8,
        0x88u8,
        0x89u8,
        0x8au8,
        0x92u8,
        0x93u8,
        0x94u8,
        0x95u8,
        0x96u8,
        0x97u8,
        0x98u8,
        0x99u8,
        0x9au8,
        0xa2u8,
        0xa3u8,
        0xa4u8,
        0xa5u8,
        0xa6u8,
        0xa7u8,
        0xa8u8,
        0xa9u8,
        0xaau8,
        0xb2u8,
        0xb3u8,
        0xb4u8,
        0xb5u8,
        0xb6u8,
        0xb7u8,
        0xb8u8,
        0xb9u8,
        0xbau8,
        0xc2u8,
        0xc3u8,
        0xc4u8,
        0xc5u8,
        0xc6u8,
        0xc7u8,
        0xc8u8,
        0xc9u8,
        0xcau8,
        0xd2u8,
        0xd3u8,
        0xd4u8,
        0xd5u8,
        0xd6u8,
        0xd7u8,
        0xd8u8,
        0xd9u8,
        0xdau8,
        0xe1u8,
        0xe2u8,
        0xe3u8,
        0xe4u8,
        0xe5u8,
        0xe6u8,
        0xe7u8,
        0xe8u8,
        0xe9u8,
        0xeau8,
        0xf1u8,
        0xf2u8,
        0xf3u8,
        0xf4u8,
        0xf5u8,
        0xf6u8,
        0xf7u8,
        0xf8u8,
        0xf9u8,
        0xfau8,
    ];
    pub static mut bits_ac_chrominance: [crate::jmorecfg_h::UINT8; 17] = [
        0u8,
        0u8,
        2u8,
        1u8,
        2u8,
        4u8,
        4u8,
        3u8,
        4u8,
        7u8,
        5u8,
        4u8,
        4u8,
        0u8,
        1u8,
        2u8,
        0x77u8,
    ];
    pub static mut val_ac_chrominance: [crate::jmorecfg_h::UINT8; 162] = [
        0u8,
        0x1u8,
        0x2u8,
        0x3u8,
        0x11u8,
        0x4u8,
        0x5u8,
        0x21u8,
        0x31u8,
        0x6u8,
        0x12u8,
        0x41u8,
        0x51u8,
        0x7u8,
        0x61u8,
        0x71u8,
        0x13u8,
        0x22u8,
        0x32u8,
        0x81u8,
        0x8u8,
        0x14u8,
        0x42u8,
        0x91u8,
        0xa1u8,
        0xb1u8,
        0xc1u8,
        0x9u8,
        0x23u8,
        0x33u8,
        0x52u8,
        0xf0u8,
        0x15u8,
        0x62u8,
        0x72u8,
        0xd1u8,
        0xau8,
        0x16u8,
        0x24u8,
        0x34u8,
        0xe1u8,
        0x25u8,
        0xf1u8,
        0x17u8,
        0x18u8,
        0x19u8,
        0x1au8,
        0x26u8,
        0x27u8,
        0x28u8,
        0x29u8,
        0x2au8,
        0x35u8,
        0x36u8,
        0x37u8,
        0x38u8,
        0x39u8,
        0x3au8,
        0x43u8,
        0x44u8,
        0x45u8,
        0x46u8,
        0x47u8,
        0x48u8,
        0x49u8,
        0x4au8,
        0x53u8,
        0x54u8,
        0x55u8,
        0x56u8,
        0x57u8,
        0x58u8,
        0x59u8,
        0x5au8,
        0x63u8,
        0x64u8,
        0x65u8,
        0x66u8,
        0x67u8,
        0x68u8,
        0x69u8,
        0x6au8,
        0x73u8,
        0x74u8,
        0x75u8,
        0x76u8,
        0x77u8,
        0x78u8,
        0x79u8,
        0x7au8,
        0x82u8,
        0x83u8,
        0x84u8,
        0x85u8,
        0x86u8,
        0x87u8,
        0x88u8,
        0x89u8,
        0x8au8,
        0x92u8,
        0x93u8,
        0x94u8,
        0x95u8,
        0x96u8,
        0x97u8,
        0x98u8,
        0x99u8,
        0x9au8,
        0xa2u8,
        0xa3u8,
        0xa4u8,
        0xa5u8,
        0xa6u8,
        0xa7u8,
        0xa8u8,
        0xa9u8,
        0xaau8,
        0xb2u8,
        0xb3u8,
        0xb4u8,
        0xb5u8,
        0xb6u8,
        0xb7u8,
        0xb8u8,
        0xb9u8,
        0xbau8,
        0xc2u8,
        0xc3u8,
        0xc4u8,
        0xc5u8,
        0xc6u8,
        0xc7u8,
        0xc8u8,
        0xc9u8,
        0xcau8,
        0xd2u8,
        0xd3u8,
        0xd4u8,
        0xd5u8,
        0xd6u8,
        0xd7u8,
        0xd8u8,
        0xd9u8,
        0xdau8,
        0xe2u8,
        0xe3u8,
        0xe4u8,
        0xe5u8,
        0xe6u8,
        0xe7u8,
        0xe8u8,
        0xe9u8,
        0xeau8,
        0xf2u8,
        0xf3u8,
        0xf4u8,
        0xf5u8,
        0xf6u8,
        0xf7u8,
        0xf8u8,
        0xf9u8,
        0xfau8,
    ];
    if (*cinfo).is_decompressor != 0 {
        dc_huff_tbl_ptrs = (*(cinfo as crate::jpeglib_h::j_decompress_ptr))
            .dc_huff_tbl_ptrs
            .as_mut_ptr();
        ac_huff_tbl_ptrs = (*(cinfo as crate::jpeglib_h::j_decompress_ptr))
            .ac_huff_tbl_ptrs
            .as_mut_ptr()
    } else {
        dc_huff_tbl_ptrs = (*(cinfo as crate::jpeglib_h::j_compress_ptr))
            .dc_huff_tbl_ptrs
            .as_mut_ptr();
        ac_huff_tbl_ptrs = (*(cinfo as crate::jpeglib_h::j_compress_ptr))
            .ac_huff_tbl_ptrs
            .as_mut_ptr()
    }
    crate::jstdhuff_c::add_huff_table(
        cinfo,
        &mut *dc_huff_tbl_ptrs.offset(0),
        bits_dc_luminance.as_ptr(),
        val_dc_luminance.as_ptr(),
    );
    crate::jstdhuff_c::add_huff_table(
        cinfo,
        &mut *ac_huff_tbl_ptrs.offset(0),
        bits_ac_luminance.as_ptr(),
        val_ac_luminance.as_ptr(),
    );
    crate::jstdhuff_c::add_huff_table(
        cinfo,
        &mut *dc_huff_tbl_ptrs.offset(1),
        bits_dc_chrominance.as_ptr(),
        val_dc_chrominance.as_ptr(),
    );
    crate::jstdhuff_c::add_huff_table(
        cinfo,
        &mut *ac_huff_tbl_ptrs.offset(1),
        bits_ac_chrominance.as_ptr(),
        val_ac_chrominance.as_ptr(),
    );
}
use crate::jmorecfg_h::boolean;
use crate::jmorecfg_h::FALSE;
use crate::jmorecfg_h::UINT8;
use crate::jpeglib_h::j_common_ptr;
use crate::jpeglib_h::j_compress_ptr;
use crate::jpeglib_h::j_decompress_ptr;
use crate::jpeglib_h::jpeg_alloc_huff_table;
use crate::jpeglib_h::jpeg_common_struct;
use crate::jpeglib_h::JHUFF_TBL;
use crate::src::jerror::JERR_BAD_HUFF_TABLE;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
