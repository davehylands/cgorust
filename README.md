# cgorust

This is a copy of https://github.com/timhughes/cgoexample which has been modified
to use rust rather than C.

# Building

You should be able to just do:
```
make
```
and this will build the rust library, run the rust tests, and build and run the go program.
You should see some output like the following:
```
$ make
cargo test --manifest-path=person/Cargo.toml
   Compiling person v0.1.0 (/Users/davehylands/go/cgorust/person)
    Finished test [unoptimized + debuginfo] target(s) in 1.43s
     Running person/target/debug/deps/person-daa0bee0ffc8f02f

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

cargo build --release --manifest-path=person/Cargo.toml
   Compiling person v0.1.0 (/Users/davehylands/go/cgorust/person)
    Finished release [optimized] target(s) in 0.21s
go build main.go
./main
Created APerson: name: "tim", long_name: "tim hughes"
Hello Go rust world: My name is tim, tim hughes.
Hello Go ruat world: My name is tim, tim hughes.
Dropping APerson: name: "tim", long_name: "tim hughes"
```
