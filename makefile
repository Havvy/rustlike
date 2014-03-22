LIBPATHS = /home/havvy/code/rust/ncurses/lib
FILE = src/main.rs
FLAGS = -g
OUTDIR = bin/


all:
	rustc -L $(LIBPATHS) $(FILE) $(FLAGS) -o $(OUTDIR)rustlike

test: test-build test-run

test-run:
	RUST_BACKTRACE=1 ./bin/test

test-build:
	RUST_LOG=std::rt::backtrace rustc -L $(LIBPATHS) $(FILE) $(FLAGS) --test -o $(OUTDIR)test -A dead-code

clean:
	rm bin/*