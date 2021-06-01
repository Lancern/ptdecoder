# ptdecoder

> Command line utility for decoding Intel PT packets from binary data stream.

## Build

`ptdecoder` is written in Rust. You need a working Rust toolchain to build and run `ptdecoder`. To install a working Rust toolchain, you can refer to [rustup](https://rustup.rs/).

Clone this repository to local:

```shell
git clone https://github.com/Lancern/ptdecoder.git
cd ptdecoder
```

Then build it with `cargo`:

```shell
cargo build
```

Then you are done. The executable file can be found under `target/debug`.

## Usage

```text
$ ptdecoder
USAGE:
    ptdecoder <INPUT>

For more information try --help
```

The only command line argument `INPUT` is the path to the file that holds binary encoded Intel PT packets data.

## License

This program is open-sourced under [MIT License](./LICENSE).

## Acknowledgements

This program depends on the amazing library [`libipt-rs`](https://github.com/sum-catnip/libipt-rs), which is a Rust-friendly wrapper around [`libipt`](https://github.com/intel/libipt).
