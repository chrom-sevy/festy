/// Calculates the CRC32 value of a byte array.
///
/// This function takes in a register value and a byte array, and calculates
/// the CRC32 value of the array by iterating over it and updating the register
/// value using the current byte and the current register value. The specific
/// polynomial used for the calculation is 0xedb88320, which is a common value
/// used in the CRC32 algorithm.
///
/// At the end, the function returns the final register value as the CRC32 value.
///
/// # Arguments
///
/// * `reg` - The initial register value for the CRC32 calculation.
/// * `s` - A slice of bytes representing the input data for which to calculate the CRC32 value.
///
/// # Returns
///
/// * The CRC32 value of the input data as a `u32`.
pub fn crc32(mut reg: u32, s: &[u8]) -> u32 {
    // Iterate over the input data
    for j in 0..s.len() {
        // Calculate the intermediate value using the current byte and the current register value
        let mut x = (s[j] as u32 ^ reg) & 255;

        // Iterate 8 times to calculate the CRC32 value
        for _ in 0..8 {
            // Update the intermediate value using the current value and the polynomial
            x = (x>>1) ^ (x&1)*0xedb88320;
        }

        // Update the register value using the intermediate value and the current register value
        reg = x ^ (reg>>8);
    }

    // Return the final register value as the CRC32 value
    return reg
}