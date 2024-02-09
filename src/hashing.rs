use secp::Point;
use sha2::Digest as _;

pub fn sha256(input: &[u8]) -> [u8; 32] {
    sha2::Sha256::new().chain_update(input).finalize().into()
}

/// Recursively increments a slice of bytes as if it were a big-endian integer.
fn inc_slice_be(slice: &mut [u8]) {
    if slice.len() == 0 {
        return;
    }
    let last = slice.len() - 1;
    if slice[last] == 0xFF {
        slice[last] = 0;
        inc_slice_be(&mut slice[..last]);
    } else {
        slice[last] += 1;
    }
}

/// Implements a secure hash-to-curve function **in non-constant time.**
/// The output [`Point`] has no known discrete log relative to [`G`][secp::G],
/// and its Y coordinate always has even-parity.
pub fn hash_to_point(input: &[u8]) -> Point {
    let mut h = sha256(input);

    loop {
        if let Ok(point) = Point::lift_x(&h) {
            return point;
        }
        inc_slice_be(&mut h);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inc_slice_be() {
        let fixtures = [
            (vec![], vec![]),
            (vec![0], vec![1]),
            (vec![0, 0, 0xFE], vec![0, 0, 0xFF]),
            (vec![0, 0, 0xFF], vec![0, 1, 0]),
            (vec![0xFF, 0xFF, 0xFF], vec![0, 0, 0]),
            (vec![0xFF, 0xFF, 1], vec![0xFF, 0xFF, 2]),
        ];

        for (input, output) in fixtures {
            let mut s = input.clone();
            inc_slice_be(&mut s);
            assert_eq!(&s, &output);
        }
    }
}
