# FESTY - A Rusty alternative for [FEST](https://github.com/RainThunder/FEST)

3DS compression written in rust for the sake of accessibility. FEST requires the dotnet Framework runtime to be compiled and run. By using rust crate, it should be trivial to port 3DS savefile decompression functionality anywhere.

As of now, the compression is not implemented as it doesn't appear to be a 100% match of the original encrypted file. Don't worry, both citra and the 3ds can read unencrypted files just fine. According to muhmuhten there this edge case across localizations where the header is smaller in japanese so they decided to encrypt it -- don't quote me on this I don't even know if I remember it correctly.

Just make sure to not have too many files in your save directory, as the game will almost certainly crash if it is too full.

# Examples

## Rust

```rust
use festy;

fn main() {
    // read an encrypted/compressed file into a buffer
    let file_encrypted = std::fs::read("chapter0").unwrap();

    // decompress the file_buffer and return the decompressed buffer
    let file_buffer = festy::file::decompres(file_encrypted).unwrap();

    // write the buffer to a file
    std::fs::write_to_file("chapter0_dec", &file_buffer).unwrap();
}

```

# Credits

most of the work goes to muhmuhten, who is the one responsible for the logic.
