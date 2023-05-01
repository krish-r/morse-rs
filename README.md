# morse-rs

![morse_rs][morse_rs_png]

A simple program to encode to/decode from morse code.

## Note

-   I'm exploring the Rust Book & rustlings, so wanted to build something while learning.
-   Supports A-Z, 0-9 (Chart/Mapping referred from on [Wikipedia][wikipedia]).
-   `|` is used for representing spaces in the encoded string
-   Consecutive spaces are treated as a single space

## TOC

-   [Install Instructions](#install-instructions)
-   [Uninstall Instructions](#uninstall-instructions)
-   [Usage](#usage)
-   [Examples](#examples)

## Install Instructions

-   Option 1 - using Cargo

    ```sh
    # Clone the repository
    git clone https://github.com/krish-r/morse-rs.git

    # Switch to the cloned directory
    cd morse-rs

    # Try it without installing
    cargo run -- [OPTIONS] [Word/Sentence]

    # or

    # To install (the cargo bin directory `~/.cargo/bin` should be in your `$PATH`)
    cargo install --path .

    ```

-   Option 2 - using the binary from the release page

    ```sh
    # Download the binary from release
    curl -LO https://github.com/krish-r/morse-rs/releases/download/0.1.0/morse-rs_0.1.0.tar.gz

    # Extract the archive
    tar xvzf ./morse-rs_0.1.0.tar.gz && rm -ir ./morse-rs_0.1.0.tar.gz

    # add executable permission to user
    chmod u+x ./morse-rs

    # Move the file somewhere in your `$PATH` (for ex. `~/.local/bin`)
    mv ./morse-rs ~/.local/bin/morse-rs
    ```

## Uninstall instructions

```sh
rm -i $(which morse-rs)
```

## Usage

```sh
morse-rs --help
```

```txt
Usage: morse-rs [OPTIONS] [Word/Sentence]

Arguments:
  [Word/Sentence]


Options:
  -m, --mode <MODE>
          mode

          [default: encode]

          Possible values:
          - encode: Encode to Dots and Dash
          - decode: Decode to String

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Examples

-   Encoding

    ```sh
    morse-rs "Hello World"

    # or

    morse-rs -m encode "Hello World"
    ```

-   Decoding

    ```sh
    # '|' should be used as delimiter instead of space (' ')
    # Note the '--' between decode and input string
    morse-rs -m decode -- ".... . .-.. .-.. --- | .-- --- .-. .-.. -.."
    ```

[wikipedia]: https://en.wikipedia.org/wiki/Morse_code
[morse_rs_png]: https://user-images.githubusercontent.com/54745129/233487314-a9f4e2d4-5bd8-4d0c-9e29-07f141cbec9d.png
