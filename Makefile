PLAIN = 5
CRYPT = eZ
E = 3
D = 3763
N = 22879

generate: build
	./target/debug/rsa --generate

encode: build
	./target/debug/rsa -e $(E) -n $(N) -s $(PLAIN)

decode: build
	./target/debug/rsa -d $(D) -n $(N) -s $(CRYPT)

build:
	cargo build
