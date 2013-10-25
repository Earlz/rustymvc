#todo: make this better. ugh

default: build

prep:
	mkdir -p build

build: prep
	rustc src/librustymvc/lib.rs --out-dir=build
	
test: prep
	rustc --test src/librustymvc/lib.rs -o build/rustymvctest
	build/rustymvctest

clean:
	rm -rf build

