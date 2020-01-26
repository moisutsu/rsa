CONTEXT = Hello
KEY = 100

generate: build
	./target/debug/rsa --generate

encode: build
	./target/debug/rsa --encode $(CONTEXT) --key $(KEY)

decode: build
	./target/debug/rsa --decode $(CONTEXT) --key $(KEY)

build:
	cargo build
