# FESTY - A Rusty alternative for [FEST](https://github.com/RainThunder/FEST)

3DS compression written in rust for the sake of accessibility. FEST requires the dotnet Framework runtime to be compiled and run. By using rust crate, it should be trivial to port 3DS savefile decompression functionality anywhere.

As of now, the compression is not implemented as it doesn't appear to be a 100% match of the original encrypted file. Don't worry, both citra and the 3ds can read unencrypted files just fine. According to muhmuhten there this edge case across localizations where the header is smaller in japanese so they decided to encrypt it -- don't quote me on this I don't even know if I remember it correctly.

Just make sure to not have too many files in your save directory, as the game will almost certainly crash if it is too full.

# Exampels

```rust

```

# Credits

most of the work goes to muhmuhten, who is the one responsible for the logic.
