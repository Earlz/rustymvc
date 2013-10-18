default: 
	rustc protomvc.rs

test:
	RUST_TEST_TASKS=1 && rustc protomvc.rs --test -o protomvc-test
	./protomvc-test

http: default
	thttpd -C thttpd.conf -D
