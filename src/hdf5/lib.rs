// lib.rs
//
// vim: ft=rust sw=4 ts=4
//
// Rust wrapper for HDF5 (hierarchical data format) library
// see docu at http://www.hdfgroup.org/ 
//
// Author: Kay-Uwe Kirstein
//

#[crate_id = "hdf5#0.1"];
#[crate_type = "dylib"];
#[desc = "Rust library to read and write HDF5 files (http://www.hdfgroup.org)"];
#[license = "BSD"];

// extern libraries
//extern mod extra;

// import symbols
//use std::libc::{c_void, c_char, c_int};
//use std::str::raw::from_c_str;
//use std::ptr::is_not_null;

// submodules
pub mod raw;
pub mod file;
pub mod dataspace;

// common definitions
// ==================

// general identifier for HDF5 entities
pub enum ID { }

// HDF5 error type
pub enum Error { }

