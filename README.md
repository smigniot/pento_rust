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

| Machine | Execution time |
| ------- | -------------- |
| Mobile phone, Linux 3.10, armv7l, 8 cores hi6250 | 126,7s |
| Macbook air 2014, MacOS Catalina 10.15.5 | 100,99s |
| Macbook Air M1 2021 | 38,47s |
| 2020 debian 5 machine, 2 cores G3240 | 22,59s |

