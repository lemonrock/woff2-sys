// This file is part of woff2-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/woff2-sys/master/COPYRIGHT. No part of woff2-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of woff2-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/woff2-sys/master/COPYRIGHT.


#![allow(non_snake_case)]


extern crate cpp_build;
extern crate cc;


use ::cc::Build;
use ::cpp_build::Config;
use ::std::env::var;


fn main()
{
	compileCPlusPlusGlueCode();
	compileWoff2Library();
}

fn compileCPlusPlusGlueCode()
{
	Config::new().include("lib/woff2/src").build("src/lib.rs");
}

fn compileWoff2Library()
{
	let brotliIncludeFolderPath = var("DEP_BROTLI_INCLUDE").unwrap();
	
	// eg x86_64-apple-darwin15
	let target = var("TARGET").unwrap();
	let compilingForMacOsX = target.rfind("-darwin").is_some();
	
	let mut cc = Build::new();
	cc
	.cpp(true)
	.shared_flag(false)
	.static_flag(true)
	.warnings(false)
	.flag("-fno-omit-frame-pointer")
	.flag("-no-canonical-prefixes")
	.flag("-std=c++11")
	.define(" __STDC_FORMAT_MACROS", None);
	if compilingForMacOsX
	{
		cc.define("OS_MACOSX", None);
	}
	else
	{
		cc.flag("-fno-tree-vrp");
	}
	cc
	.include(brotliIncludeFolderPath)
	.file("lib/woff2/src/font.cc")
	.file("lib/woff2/src/glyph.cc")
	.file("lib/woff2/src/normalize.cc")
	.file("lib/woff2/src/table_tags.cc")
	.file("lib/woff2/src/transform.cc")
	.file("lib/woff2/src/woff2_dec.cc")
	.file("lib/woff2/src/woff2_enc.cc")
	.file("lib/woff2/src/woff2_common.cc")
	.file("lib/woff2/src/woff2_out.cc")
	.file("lib/woff2/src/variable_length.cc")
    .compile("woff2");
}
