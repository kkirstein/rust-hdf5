// test.rs
//
// vim: ft=rust sw=4 ts=4
//
// Test case for HDF5 Rust bindings
// see docu at http://www.hdfgroup.org/ 
//
// Author: Kay-Uwe Kirstein
//

//extern crate hdf5;

// tests for predefined constants
// ==============================
#[test]
fn test_properties() {

	use raw::H5P_DEFAULT;
	
	assert_eq!(0, H5P_DEFAULT as i32);
}

#[test]
fn test_file_access_flags() {

	use raw::{H5F_ACC_RDONLY, H5F_ACC_RDWR, H5F_ACC_TRUNC, H5F_ACC_EXCL, H5F_ACC_DEBUG, H5F_ACC_CREAT, H5F_ACC_DEFAULT};

	assert_eq!(0, H5F_ACC_RDONLY as uint);
	assert_eq!(1, H5F_ACC_RDWR as uint);
	assert_eq!(2, H5F_ACC_TRUNC as uint);
	assert_eq!(4, H5F_ACC_EXCL as uint);
	assert_eq!(8, H5F_ACC_DEBUG as uint);
	assert_eq!(16, H5F_ACC_CREAT as uint);
	assert_eq!(65535, H5F_ACC_DEFAULT as uint);
}

