default: 
	rustc protomvc.rs

test:
	rustc protomvc.rs --test -o protomvc-test
	./protomvc-test

http: default
	thttpd -C thttpd.conf -D
