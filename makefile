SRC		= src/lib.rs

.PHONY: all
all: libstatic_mdo.rlib

libstatic_mdo.rlib: makefile $(SRC)
	rm -f libstatic_mdo*.rlib
	rustc --crate-type rlib src/lib.rs
	find . -iname 'libstatic_mdo*.rlib' | head -1 | xargs -I% ln -s % $@

clean:
	rm -f *.rlib
