default: clean build/release

test:
	@cargo test

clean:
	@rm -Rf target dist

build:
	cargo build

build/release:
	cargo build --release

install:
	sudo cp target/release/qf /usr/local/bin/qf
