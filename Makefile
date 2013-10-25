#todo: make this better. ugh

default: build

prep:
	mkdir -p bin

build: prep
	rustc src/librustymvc/lib.rs --out-dir=bin

test: prep
	rustc --test src/librustymvc/lib.rs -o bin/rustymvctest
	bin/rustymvctest

clean:
	rm -rf bin

