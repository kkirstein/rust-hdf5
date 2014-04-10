// work.rs
//
// vim: ft=rust sw=4 ts=4
//
// playground to develop Rust bindings for HDF5 (hierarchical data format) library
// see docu at http://www.hdfgroup.org/ 
//
// Author: Kay-Uwe Kirstein
//

// Examples taken from "Learn the basics tutorial" at
// http://www.hdfgroup.org/HDF5/examples/intro.html#c 

extern crate hdf5;

// Create a dataset
fn crtdat() {

	use hdf5::raw::{H5Fcreate, H5Fclose, H5Screate_simple, H5Sclose, H5Dcreate2, H5Dclose};
	use	hdf5::raw::{H5F_ACC_TRUNC, H5P_DEFAULT};
	use hdf5::raw::H5T_STD_I32BE;

	unsafe {

		// Create new file using default properties
		let file_name = "dset.h5";
		let file_id = file_name.with_c_str( |cstr| H5Fcreate(cstr, H5F_ACC_TRUNC as u32, H5P_DEFAULT as i32, H5P_DEFAULT as i32) );

		// Create the data space for the dataset
		let dims: ~[u64] = ~[4, 6];
		let dataspace_id = H5Screate_simple(2, dims.as_ptr(), ::std::ptr::null());

		// Create the dataset
		let dataset_name = "/dset";
		//let dataset_id = H5Dcreate2(file_id, "/dset", H5T_STD_I32BE, dataspace_id,
		//	H5P_DEFAULT as i32, H5P_DEFAULT as i32, H5P_DEFAULT as i32);
		let dataset_id = dataset_name.with_c_str( |cstr| H5Dcreate2(file_id, cstr, H5T_STD_I32BE.as_id(), dataspace_id,
			H5P_DEFAULT as i32, H5P_DEFAULT as i32, H5P_DEFAULT as i32) );

		// End access to the dataset and release resources used by it
		let status = H5Dclose(dataset_id);

   // Terminate access to the data space
   let status = H5Sclose(dataspace_id);

   // Close the file
   let status = H5Fclose(file_id);

	}
}

// main entry point
fn main() {
	println!("Hello HDF5!");

	crtdat();

	println!("Bye bye..");
}

