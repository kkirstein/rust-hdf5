# Makefile to build pcre binding library
#
# should be replaced by a proper rustpkg build later...
#

libsource = ./src/hdf5/lib.rs
binsource = ./src/work.rs
testsource = ./src/hdf5/test.rs
hdf5_sourcedir = ./src/pcre/C
hdf5_lib = libpcre.a

# compiler switches
rustc = rustc
outdirflag = --out-dir .
rclibflags = --lib -O -L$(clibdir)
rcbinflags = -O -L.

# define phony targets
.PHONY: all lib clean

# default target
all: lib exe

# lib target
lib: $(libsource)
	$(rustc) $(rclibflags) $(outdirflag) $<

# bin target
exe: $(binsource)
	$(rustc) $(rcbinflags) $(outdirflag) $(binsource)

# test target
test: $(testsource) lib
	$(rustc) --test $(rcbinflags) $(outdirflag) $<
	./test

clean:

realclean: clean
	$(RM) *.dll
	$(RM) *.exe


