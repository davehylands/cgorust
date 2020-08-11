
all: main run

main: lib
	go build

lib:
	cargo build --manifest-path=person/Cargo.toml

test:
	cargo test --manifest-path=person/Cargo.toml

clean:
	go clean
	cargo clean --release --manifest-path=person/Cargo.toml
	cargo clean --manifest-path=person/Cargo.toml

run:
	./cgorust
