use core::panic;

use rand::{seq::SliceRandom, Rng};

use crate::primitives::{add, div, mult};

pub struct Polynomal {
    pub coefficients: Vec<u8>,
}

impl Polynomal {
    pub fn new(degree: u8, secret: u8) -> Self {
        let mut coefficients = vec![0; degree as usize + 1];
        coefficients[0] = secret;

        let mut rng = rand::thread_rng();
        rng.fill(&mut coefficients[1..]);

        Polynomal { coefficients }
    }

    pub fn evaluate(&self, x: u8) -> u8 {
        if x == 0 {
            return self.coefficients[0];
        }

        let degree = self.coefficients.len() - 1;
        let mut out = self.coefficients[degree];
        for i in (0..degree).rev() {
            let coeff = self.coefficients[i];
            out = add(mult(out, x), coeff);
        }

        return out;
    }
}

pub fn split(secret: Vec<u8>, shares: u8, threshold: u8) -> Vec<Vec<u8>> {
    if shares < threshold {
        panic!("shares has to be larger than threshold");
    }
    if threshold < 2 {
        panic!("threshold should be at least 2");
    }
    if secret.len() == 0 {
        panic!("secret needs to have at least 1 byte");
    }

    let mut rng = rand::thread_rng();
    let mut x_coordinates: Vec<u8> = (0..255).collect();
    x_coordinates.shuffle(&mut rng);

    let mut out: Vec<Vec<u8>> = vec![vec![0; secret.len() + 1]; shares.into()];

    for i in 0..out.len() {
        out[i][secret.len()] = x_coordinates[i] + 1;
    }

    for i in 0..secret.len() {
        let polynomial = Polynomal::new(threshold - 1, secret[i]);
        for j in 0..shares as usize {
            let x: u8 = x_coordinates[j] + 1;
            let y = polynomial.evaluate(x);
            out[j][i] = y;
        }
    }
    out
}

pub fn combine(shares: &Vec<Vec<u8>>) -> Vec<u8> {
    if shares.len() < 2 {
        panic!("need more than 2 shares to reconstruct the secret");
    }

    let mut secret: Vec<u8> = vec![0; shares[0].len() - 1];
    let mut x_shares = vec![0; shares.len()];
    let mut y_shares = vec![0; shares.len()];

    for i in 0..shares.len() {
        x_shares[i] = shares[i][shares[i].len() - 1];
    }

    for i in 0..secret.len() {
        for j in 0..shares.len() {
            y_shares[j] = shares[j][i];
        }

        let b = _lagrange_interpolation(&x_shares, &y_shares, 0);
        secret[i] = b;
    }
    secret
}

fn _lagrange_interpolation(x_shares: &Vec<u8>, y_shares: &Vec<u8>, x: u8) -> u8 {
    let k = x_shares.len();
    let mut result = 0;
    let mut basis;

    for i in 0..k {
        basis = 1;

        for j in 0..k {
            if i == j {
                continue;
            }

            let num = add(x, x_shares[j]);
            let denom = add(x_shares[i], x_shares[j]);
            let term = div(num, denom);
            basis = mult(basis, term);
        }

        let group = mult(y_shares[i], basis);
        result = add(result, group);
    }

    result
}
