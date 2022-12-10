use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};
use crate::{
    crc32::crc32,
    huffman,
};

/// Decompresses a file in the specified format.
///
/// This function takes in a vector of bytes representing the file buffer,
/// which is the entire contents of the file in memory. It then iterates over
/// the file buffer to find the sector type, and returns an error if it is not found.
///
/// The function then reads the expected length and CRC value of the decompressed data
/// from the file buffer, and uses a Huffman decoding function to decrypt the Huffman set
/// in the file buffer. It checks that the decrypted length matches the expected length,
/// and returns an error if these values do not match.
///
/// The function then calculates the CRC value of the file buffer and the decrypted data,
/// and compares it to the expected value. If these values do not match, the function
/// returns an error. Finally, if all checks pass, the function returns the concatenation
/// of the file buffer and the decrypted data as a `Vec<u8>`.
///
/// # Arguments
///
/// * `file_buffer` - A `Vec<u8>` representing the file buffer.
///
/// # Returns
///
/// * If the sector type is not found, or if the decrypted length and expected length do not
///   match, or if the calculated CRC value and expected CRC value do not match, the function
///   returns an error as a `String`.
/// * Otherwise, the function returns the concatenation of the file buffer and the decrypted
///   data as a `Vec<u8>`.
pub fn decompress(file_buffer: Vec<u8>) -> Result<Vec<u8>, String> {
    // Create a new cursor that reads from the file buffer
    let mut cursor = Cursor::new(&file_buffer);

    // Initialize the offset and sector type variables
    let mut offset = 0;
    let mut sector_type = 0;

    // Iterate over the file buffer until the end of the file is reached,
    // or the maximum offset is reached, or the sector type is found
    while offset <= 1024 && offset+3 < file_buffer.len() {
        // Read a 4-byte word from the file buffer using the cursor
        let word = cursor.read_u32::<LittleEndian>().unwrap();

        // If the current word is 0x434f4d50, then set the sector type
        if word == 0x434f4d50 {
            sector_type = word;
            break;
        }

        // Increment the offset and move the cursor to the new position
        offset += 64;
        cursor.set_position(offset as u64);
    }

    // If the sector type was not found, return an error
    if sector_type == 0 {
        return Err("not a compressed file".to_owned())
    }

    // Move the cursor to the expected length and read it
    cursor.set_position(offset as u64 + 8);
    let expected_length = cursor.read_u32::<LittleEndian>().unwrap() as usize;

    // Read the expected CRC value
    let expected_crc = cursor.read_u32::<LittleEndian>().unwrap();

    // Get the starting position of the Huffman set
    let huff_set_start = cursor.position() as usize;

    // Check if the next byte is 0x28, and return an error if it is not
    if cursor.read_u8().unwrap() != 0x28 {
        return Err("file `{file_buf}`: unkown compresssion format".to_owned())
    }

    // Decrypt the Huffman set
    let decrypted = huffman::decode(&file_buffer[huff_set_start..]).unwrap();

    // Check if the decrypted length matches the expected length, and return an error if it does not
    if decrypted.len() != expected_length {
        return Err("unexpected length, something defeinitely went wrong".to_owned())
    }

    // Get the header from the file buffer
    let header = &file_buffer[..offset];

    // Calculate the CRC value of the header and the decrypted data
    let crc = !crc32(crc32(!0, header), &decrypted);

    // Check if the calculated CRC value matches the expected value, and return an error if it does not
    if crc != expected_crc {
        return Err("unexpected crc".to_owned())
    }

    // Return the concatenation of the header and decrypted data
    return Ok([header, &decrypted].concat());
}