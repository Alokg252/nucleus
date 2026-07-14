pub fn set(bitmap: &mut [u8], bit: usize) {
    let byte_index = bit / 8;
    let bit_index = bit % 8;

    bitmap[byte_index] |= 1 << bit_index;
}

pub fn clear(bitmap: &mut [u8], bit: usize) {
    let byte_index = bit / 8;
    let bit_index = bit % 8;

    bitmap[byte_index] &= !(1 << bit_index);
}

pub fn is_set(bitmap: &[u8], bit: usize) -> bool {
    let byte_index = bit / 8;
    let bit_index = bit % 8;

    (bitmap[byte_index] & (1 << bit_index)) != 0
}