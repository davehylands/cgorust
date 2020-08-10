
all: test main run

main: lib
	go build

lib:
	cargo build --release --manifest-path=person/Cargo.toml

test:
	cargo test --manifest-path=person/Cargo.toml

clean:
	go clean
	cargo clean --manifest-path=person/Cargo.toml

run:
	./cgorust
