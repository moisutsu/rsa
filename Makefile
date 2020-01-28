CONTEXT = Hello
E = 5
D = 3307189001887676048
N = 14882350516239957559

generate: build
	./target/debug/rsa --generate

encode: build
	./target/debug/rsa -e $(E) -n $(N) -s $(CONTEXT)

decode: build
	./target/debug/rsa -d $(D) -n $(N) -s $(CONTEXT)

build:
	cargo build
