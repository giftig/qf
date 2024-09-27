.PHONY: test

default: clean build/release

test:
	@cargo test

coverage:
	@cargo llvm-cov

clean:
	@rm -Rf target dist

build:
	cargo build

build/release:
	cargo build --release

install:
	sudo cp target/release/qf /usr/local/bin/qf
