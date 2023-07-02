## FIGHTING WITH MAKE
TARGETS = x86_64-unknown-linux-gnu x86_64-pc-windows-msvc x86_64-apple-darwin aarch64-unknown-linux-gnu aarch64-apple-darwin

build: clean
	cargo build --release;

%:
	cross build --release --target $*

.PHONY: clean
clean:
	-rm -rf target/release;
