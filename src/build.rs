// This file is part of woff2-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/woff2-sys/master/COPYRIGHT. No part of woff2-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of woff2-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/woff2-sys/master/COPYRIGHT.


extern crate cpp_build;
extern crate cc;


use ::bindgen::Builder;
use ::gcc::Build;
use ::std::env::var;


fn main()
{
	compileCPlusPlusGlueCode();
	
	
}

fn compileCPlusPlusGlueCode()
{
	cpp_build::build("src/lib.rs");
}

fn compileWoff2Library()
{
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
	
	.file("lib/woff2/font.c")
	.file("lib/woff2/glyph.c")
	.file("lib/woff2/normalize.c")
	.file("lib/woff2/table_tags.c")
	.file("lib/woff2/transform.c")
	.file("lib/woff2/woff2_dec.c")
	.file("lib/woff2/woff2_enc.c")
	.file("lib/woff2/woff2_common.c")
	.file("lib/woff2/woff2_out.c")
	.file("lib/woff2/variable_length.c")
	
	// everything in brotli
	
    .compile("woff2");
}

/*
BROTLI = brotli
BROTLIOBJ = $(BROTLI)/bin/obj
ENCOBJ = $(BROTLIOBJ)/enc/*.o
DECOBJ = $(BROTLIOBJ)/dec/*.o
COMMONOBJ = $(BROTLIOBJ)/common/*.o
*/
