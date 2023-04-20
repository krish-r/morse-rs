# morse-rs

![morse_rs][morse_rs_png]

A simple program to encode to/decode from morse code.

## Note

-   I'm exploring the Rust Book & rustlings, so wanted to build something while learning.
-   Supports A-Z, 0-9 (Chart/Mapping referred from on [Wikipedia][wikipedia]).
-   `|` is used for representing spaces in the encoded string
-   Consecutive spaces are treated as a single space

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
    # `|` should be used as delimiter instead of space (` `)
    morse-rs -m decode ".... . .-.. .-.. --- | .-- --- .-. .-.. -.."
    ```

[wikipedia]: https://en.wikipedia.org/wiki/Morse_code
[morse_rs_png]: https://user-images.githubusercontent.com/54745129/233487314-a9f4e2d4-5bd8-4d0c-9e29-07f141cbec9d.png
