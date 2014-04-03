// raw.rs
//
// vim: ft=rust sw=4 ts=4
//
// Low-level (raw) bindings to HDF5 (hierarchical data format) library
// see docu at http://www.hdfgroup.org/ 
//
// Author: Kay-Uwe Kirstein
//

use std::libc::{c_char, c_uint, c_int, c_ulonglong};

// basic HDF5 types
type ID = c_int;
type Error = c_int;
type Size = c_ulonglong;

// file creation flags
pub enum H5Flags {
	H5F_ACC_RDONLY	= 0x0000u,	/*absence of rdwr => rd-only */
	H5F_ACC_RDWR	= 0x0001u,	/*open for read and write    */
	H5F_ACC_TRUNC	= 0x0002u,	/*overwrite existing files   */
	H5F_ACC_EXCL	= 0x0004u,	/*fail if file already exists*/
	H5F_ACC_DEBUG	= 0x0008u,	/*print debug info	     */
	H5F_ACC_CREAT	= 0x0010u,	/*create non-existing files  */
	H5F_ACC_DEFAULT = 0xffffu	/*ignore setting on lapl     */
}

// propertiy flags
pub enum H5PFlags {
	H5P_DEFAULT		= 0
}

// wrap unsafe function from HDF5 API
#[link(name = "hdf5dll")]
extern {

	// file namespace
	pub fn H5Fcreate(name: *c_char, flags: c_uint, fcpl_id: ID, fapl_id: ID) -> ID;
	pub fn H5Fclose(file_id: ID) -> Error;

	// dataspace namespace
	pub fn H5Screate_simple(rank: c_int, current_dims: *c_ulonglong, maximum_dims: *c_ulonglong) -> ID;
	pub fn H5Sclose(file_id: ID) -> Error;

	// dataset namespace
	pub fn H5Dcreate2(loc_id: ID, name: *c_char, dtype_id: ID, space_id: ID,
		lcpl_id: ID, dcpl_id: ID, dapl_id: ID) -> ID;
	pub fn H5Dclose(dataset_id: ID);
}

