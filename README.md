# Pentominos in Rust

See
* https://en.wikipedia.org/wiki/Pentomino#Constructing_rectangular_dimensions
* https://www.migniot.com/Pentominos.html
* https://github.com/smigniot/Polyomino
* https://rustup.rs/

# Build instructions

* Follow the instructions at https://rustup.rs/ . This gets you `cargo` and `rustc` executables
* Clone this repository, `git clone https://github.com/smigniot/pento_rust.git`
* Inside the pento_rust directory, run `cargo run`

# Performance testing

* Follow the instructions at https://rustup.rs/ . This gets you `cargo` and `rustc` executables
* Clone this repository, `git clone https://github.com/smigniot/pento_rust.git`
* Inside the pento_rust directory, run `cargo build --release && time ./target/release/pento_rust`
* Send me a mail with the results :)

# One-liner

`rm -Rf /tmp/pento_rust && git -C /tmp/ clone https://github.com/smigniot/pento_rust.git && cd /tmp/pento_rust && cargo build --release && time ./target/release/pento_rust`

# Results - for the reference one-liner

* On a Macbook air 2014, MacOS Catalina 10.15.5

    ./target/release/pento_rust  99,19s user 0,61s system 98% cpu 1:40,99 total

* On a 2020 linux machine, debian 5.10.0-13-amd64, 2 cores G3240

    real	0m22.591s
 
* On a mobile phone, Linux 3.10, armv7l, 8 cores hi6250

   real 2m6.716s 

* On a TUF Gaming B450-Plus II

   execution scheduled - awaiting results - stay tuned :)

