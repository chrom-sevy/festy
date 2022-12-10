/// Decodes a Huffman-encoded source buffer into a new buffer.
///
/// This function takes a reference to a source buffer containing Huffman-encoded data and decodes it into
/// a new buffer. The function returns a `Result` containing the decoded buffer or an error message if
/// something went wrong during the decoding process.
///
/// # Arguments
///
/// * `src_buf` - A reference to the source buffer containing Huffman-encoded data.
///
/// # Returns
///
/// * A `Result` containing the decoded buffer or an error message if something went wrong.
///
/// # Errors
///
/// This function can return the following error messages:
///
/// * "huf buf insuf": The source buffer is too small to contain a Huffman-encoded data.
/// * "not huffman mode 28": The source buffer does not contain Huffman-encoded data.
/// * "pos out of tree": The position of the cursor in the Huffman tree is out of bounds.
/// * "file had {n} trailing bytes": The source buffer has too many trailing bytes.
/// * "file ended with {n} bytes missing": The source buffer does not have enough bytes to decode.
pub fn decode(src_buf: &[u8]) -> Result<Vec<u8>, String> {
    // Guard clause to check if the source buffer is too small to contain Huffman-encoded data
    if src_buf.len() <= 6 {
        return Err("huf buf insuf".to_owned());
    }

    // Read the extra size from the source buffer
    let extra_size = src_buf.get(4).cloned().ok_or("failed reading byte")?;

    // Calculate the index of the magic number in the source buffer
    let mag_idx = 6 + 2 * extra_size as usize;

    // Guard clause to check if the source buffer does not contain Huffman-encoded data
    if src_buf.len() <= mag_idx {
        return Err("not huffman mode 28".to_owned());
    }

    // Calculate the length of the output buffer
    let out_length =
        src_buf[1] as usize | (src_buf[2] as usize) << 8usize | (src_buf[3] as usize) << 16usize;

    // Create the output buffer
    let mut out_buf = vec![0u8; out_length];

    // Get the Huffman tree from the source buffer
    let tree = &src_buf[4..mag_idx];

    // Set the initial entry and position in the Huffman tree
    let mut entry = tree[1];
    let mut pos = 0;

    // Set the initial position in the output buffer
    let mut dp = 0;

    // Iterate over the source buffer in chunks of 4 bytes
    for sp in (mag_idx..src_buf.len()).step_by(4) {
        // Iterate over the current chunk of the source buffer in reverse
        for sq in (sp..=sp + 3).rev() {
            // Set the initial bit mask
            let mut k = 128;

            // Iterate over the bits in the current byte of the source buffer
            while k > 0 {
                // Increment the position in the Huffman tree
                pos += (entry & 0x3F) + 1;

                // Check if the position is out of bounds, and return an error if it is
                if pos > tree[0] {
                    return Err("pos out of tree".to_owned());
                }

                // If the current bit is set, move to the right child of the current entry
                // and check if it is a leaf node
                let isleaf = if src_buf[sq] & k != 0 {
                    let old_entry = entry;
                    entry = tree[2 * pos as usize + 1];
                    old_entry & 0x40
                }
                // If the current bit is not set, move to the left child of the current entry
                // and check if it is a leaf node
                else {
                    let old_entry = entry;
                    entry = tree[2 * pos as usize];
                    old_entry & 0x80
                };

                // If the current entry is a leaf node, add its value to the output buffer
                // and check if the output buffer is complete
                if isleaf != 0 {
                    out_buf[dp] = entry;
                    dp += 1;
                    if dp >= out_length {
                        // If the source buffer has too many trailing bytes, return an error
                        if src_buf.len() - sp != 4 {
                            return Err(format!("file had {} trailing bytes", src_buf.len() - sp)
                                .to_owned());
                        }

                        // Return the output buffer if it is complete
                        return Ok(out_buf);
                    }

                    // Move back to the root of the Huffman tree and reset the position
                    entry = tree[1];
                    pos = 0;
                }

                // Shift the bit mask to the right
                k >>= 1;
            }
        }
    }

    // If the output buffer is not complete, return an error
    return Err(format!("file ended with {} bytes missing", out_length - dp).to_owned());
}
