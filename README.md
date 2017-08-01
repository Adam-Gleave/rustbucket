# RustBucket
Tiny experimental kernel written in Rust and x86 assembly language. Purely for educational purposes. Inspiration take from [OSDev](http://wiki.osdev.org/Main_Page), [Philipp Oppermann](https://os.phil-opp.com/set-up-rust/), and [Bare Metal Rust](http://www.randomhacks.net/bare-metal-rust/).

### Build
RustBucket is written using Rust, and therefore needs rust (and Cargo) to be installed in order to build properly. To do so, use the script [here](http://rustup.rs/), if on Linux or UNIX. For Windows, there are further instructions on the website. Since the kernel makes use of the [no_std] attribute (since the standard library cannot be implemented, we must recreate one), the __nightly__ toolkit must be used. Xargo (a cross-compiler extension to cargo), must also be installed, so that rlib (a Rust implementation of libc) can be built for the system. QEMU must also be installed if you wish to run the resultant .iso image in an emulator.

Once these have been installed and set up correctly, run __make run__, to boot into the kernel with QEMU.
