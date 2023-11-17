// Galois Fields GF(2^8) operations
pub fn add(a: u8, b: u8) -> u8 {
    a ^ b
}

pub fn mult(mut a: u8, mut b: u8) -> u8 {
    let mut r: u8 = 0;
    for _ in 0..8 {
        if b & 1 != 0 {
            r ^= a;
        }

        if a & 0x80 != 0 {
            a = (a << 1) ^ 0x1B;
        } else {
            a <<= 1;
        }

        b >>= 1;
    }
    r
}

pub fn div(a: u8, b: u8) -> u8 {
    if b == 0 {
        panic!("Division by zero!");
    }
    return mult(a, _invert(b));
}

fn _invert(b: u8) -> u8 {
    let mut z = b;
    for _ in 0..6 {
        z = mult(z, z);
        z = mult(z, b);
    }
    return mult(z, z);
}
