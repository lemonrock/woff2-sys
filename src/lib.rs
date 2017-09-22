// This file is part of woff2-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/woff2-sys/master/COPYRIGHT. No part of woff2-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of woff2-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/woff2-sys/master/COPYRIGHT.


#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


#[macro_use] extern crate cpp;


use ::std::mem::uninitialized;


cpp!
{{
	#include "woff2_enc.h"

	using std::string;
	using woff2::MaxWOFF2CompressedSize;
	using woff2::ConvertTTFToWOFF2;
	using woff2::WOFF2Params;
}}

#[link(name = "brotli")]
extern "C"
{
}

/// brotliQuality should normally be 11, allowTransforms should normally be true
pub fn convertTtfToWoff2(ttfFontBytes: &[u8], additionalExtendedMetadataBytes: &[u8], brotliQuality: u8, allowTransforms: bool) -> Result<Vec<u8>, ()>
{
	debug_assert!(brotliQuality < 12, "brotliQuality should be between 0 and 11 inclusive");
	
	let capacity = MaxWOFF2CompressedSize(ttfFontBytes.len(), additionalExtendedMetadataBytes.len());
	let mut woffFontBytes = Vec::with_capacity(capacity);
	let mut woffFontBytesLength = unsafe { uninitialized() };
	
	let success = ConvertTTFToWOFF2(ttfFontBytes.as_ptr(), ttfFontBytes.len(), woffFontBytes.as_mut_ptr(), &mut woffFontBytesLength, additionalExtendedMetadataBytes.as_ptr(), additionalExtendedMetadataBytes.len(), brotliQuality as i32, allowTransforms);
	if success
	{
		unsafe { woffFontBytes.set_len(woffFontBytesLength) };
		woffFontBytes.shrink_to_fit();
		Ok(woffFontBytes)
	}
	else
	{
		Err(())
	}
}

#[inline(always)]
fn MaxWOFF2CompressedSize(length: usize, extended_metadata_length: usize) -> usize
{
	length + 1024 + extended_metadata_length
	
	// cpp!([data as "uint8_t *", length as "size_t", extended_metadata as "const char *", extended_metadata_length as "size_t"] -> usize as "size_t"
	// {
	// 	string copyOfExtendedMetadata(extended_metadata, extended_metadata_length);
	//
	// 	return MaxWOFF2CompressedSize(data, length, &copyOfExtendedMetadata);
	// })
}

fn ConvertTTFToWOFF2(data: *const u8, length: usize, result: *mut u8, result_length: *mut usize, extended_metadata: *const u8, extended_metadata_length: usize, brotli_quality: i32, allow_transforms: bool) -> bool
{
	unsafe
	{
		cpp!([data as "const uint8_t *", length as "size_t", result as "uint8_t *", result_length as "size_t *", extended_metadata as "const char *", extended_metadata_length as "size_t", brotli_quality as "int", allow_transforms as "bool"] -> bool as "bool"
		{
			string copyOfExtendedMetadata(extended_metadata, extended_metadata_length);
		
			struct WOFF2Params params;
			params.extended_metadata = copyOfExtendedMetadata;
			params.brotli_quality = brotli_quality;
			params.allow_transforms = allow_transforms;
		
			return ConvertTTFToWOFF2(data, length, result, result_length, params);
		})
	}
}
