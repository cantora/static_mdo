SRC		= src/lib.rs

.PHONY: all
all: libstatic_mdo.rlib

libstatic_mdo.rlib: makefile $(SRC)
	rustc --crate-type rlib src/lib.rs

clean:
	rm -f *.rlib
