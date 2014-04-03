# Makefile to build pcre binding library
#
# should be replaced by a proper rustpkg build later...
#

libsource = ./src/hdf5/lib.rs
binsource = ./src/work.rs
testsource = ./src/hdf5/test.rs
hdf5_sourcedir = 
hdf5_libdir = ./src/hdf5/C/bin
hdf5_lib = libhdf5.a

# compiler switches
rustc = rustc
outdirflag = --out-dir .
rclibflags = --lib -O -L$(hdf5_libdir)
#rclibflags = --lib -O
rcbinflags = -O -L. -L$(hdf5_libdir)

# define phony targets
.PHONY: all lib clean

# default target
#all: lib exe

# lib target
lib: $(libsource)
	$(rustc) $(rclibflags) $(outdirflag) $(libsource)

# bin target
exe: $(binsource) lib
	$(rustc) $(rcbinflags) $(outdirflag) $<

# test target
test: $(testsource) lib
	$(rustc) --test $(rcbinflags) $(outdirflag) $<
	./test

clean:

realclean: clean
	$(RM) *.dll
	$(RM) *.exe


