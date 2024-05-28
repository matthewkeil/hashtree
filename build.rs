/*
MIT License

Copyright (c) 2021-2024 Prysmatic Labs

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
  let out_dir = env::var("OUT_DIR").unwrap_or_else(|_| {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    format!("{}/build", manifest_dir)
  });

  let lib_dir = PathBuf::from(&out_dir).join("lib");
  let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

  let status = Command::new("make")
    .current_dir(&src_dir)
    .env("OUT_DIR", &out_dir) // Pass OUT_DIR to makefile if needed
    .status()
    .expect("Failed to execute make command");

  if !status.success() {
    panic!("Failed to build the C library");
  }

  println!("cargo:warning=libdir: {}", lib_dir.display());
  match fs::read_dir(&lib_dir) {
    Ok(entries) => {
      for entry in entries {
        match entry {
          Ok(entry) => {
            println!("cargo:warning=libdir content: {}", entry.path().display())
          }
          Err(e) => println!("Error reading entry in libdir: {}", e),
        }
      }
    }
    Err(e) => println!("Error reading libdir: {}", e),
  }

  println!("cargo:rustc-link-search=native={}", lib_dir.display());

  let build_target = env::var("TARGET").unwrap();
  if build_target.contains("windows") {
    println!("cargo:rustc-link-lib=static=libhashtree");
  } else {
    println!("cargo:rustc-link-lib=static=hashtree");
  }
}
