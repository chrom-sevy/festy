use std::fs;
use crate::file;

/// comapring the output to the javascript implementation
/// chatper0_dec is the decrypted version of chapter0
/// I'm leaving one of mine in here, it is just from one of my attack stance awakening save files
/// that you can watch on my channel https://www.youtube.com/channel/UCyyTLUvwR2cJqHv1g3dh7EA ;p
#[test]
fn decompress() {
    let original = fs::read("chapter0_dec").unwrap();
    let file = fs::read("chapter0").unwrap();
    let file = file::decompress(file).unwrap();
    assert_eq!(original, file);
}
