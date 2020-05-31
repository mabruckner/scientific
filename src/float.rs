/// return the f64 with the next largest absolute value
pub fn next_f64(val: f64) -> f64 {
    // convert to in-memory representation
    let mut bits: u64 = val.to_bits();
    // logically, the procedure is to increment the fractional part, and if it overflows, increment
    // the exponent. However the exponent is stored after the fractional part so we can just do
    // this.
    bits = bits + 1;
    f64::from_bits(bits)
}

pub fn inf_f64() -> f64 {
    f64::from_bits(0x7FFF000000000000)
}
