# Makefile to build pcre binding library
#
# should be replaced by a proper rustpkg build later...
#

libsource = ./src/hdf5/lib.rs
binsource = ./src/work.rs
testsource = ./src/hdf5/test.rs
hdf5_sourcedir = 
hdf5_libdir = ./src/hdf5/C/bin/
hdf5_lib = hdf5dll.dll zlib.dll szip.dll

# compiler switches
rustc = rustc
outdirflag = --out-dir .
rclibflags = -O -L$(hdf5_libdir)
rcbinflags = -O -L. -L$(hdf5_libdir)

# tools
CP = cp

# define phony targets
.PHONY: all lib clean

# default target
#all: lib exe

# lib target
lib: $(libsource) $(hdf5_lib)
	$(rustc) $(rclibflags) $(outdirflag) $<

$(hdf5_lib):
	$(CP) $(addprefix $(hdf5_libdir), $(hdf5_lib)) .

# bin target
exe: $(binsource) lib
	$(rustc) $(rcbinflags) $(outdirflag) $<

# test target
test: $(libsource)
	$(rustc) --test $(rclibflags) $(outdirflag) $<
	./hdf5

clean:

realclean: clean
	$(RM) *.dll
	$(RM) *.exe


