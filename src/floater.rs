pub fn f32tof24(f: f32) -> u32 {
    let i: u32 = f.to_bits();
    let mut mantissa: u32 = (i << 9) >>  9;
	let mut exponent: i32 = ((i << 1) >> 24) as i32;
	let     sign:     u32 = (i << 0) >> 31;

	// Truncate mantissa
	mantissa >>= 7;

	// Re-bias exponent
	exponent = exponent - 127 + 63;
	if (exponent < 0)
	{
		// Underflow: flush to zero
		return sign << 23;
	}
	else if (exponent > 0x7F)
	{
		// Overflow: saturate to infinity
		return sign << 23 | 0x7F << 16;
	}

	sign << 23 | (exponent as u32) << 16 | mantissa
}

///Data expected in XYZW order
pub fn f32x4tof24x4(f: [f32;4]) -> [u32;3] {
    let [x,y,z,w] = f.map(f32tof24);
    [
        ((z >> 16) & 0xFF) | (w << 8),
        ((y >> 8) & 0xFFFF) | ((z & 0xFFFF) << 16),
        x | ((y & 0xFF) << 24)
    ]
}
