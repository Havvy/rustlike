RUSTPATH = /home/havvy/code/rust/
LIBPATHS = -L $(RUSTPATH)rustlike/lib -L $(RUSTPATH)ncurses/lib
FLAGS = -g
OUTDIR = bin/
FILELIB = src/librusttower/rusttower.rs


all: librusttower ncurses

# lib build
librusttower:
	rustc $(FILELIB) $(LIBPATHS) $(FLAGS) --out-dir /home/havvy/code/rust/rustlike/lib

test: test-build test-run

test-build:
	RUST_LOG=std::rt::backtrace rustc $(FILELIB) $(LIBPATHS) $(FLAGS) -o $(OUTDIR)test --test -A dead-code

test-run:
	RUST_BACKTRACE=1 ./bin/test

# ncurses build
ncurses:
	rustc src/ncurses/main.rs $(LIBPATHS) $(FLAGS) -o $(OUTDIR)rusttower-nc

# just in case
clean:
	rm bin/*
	rm lib/*