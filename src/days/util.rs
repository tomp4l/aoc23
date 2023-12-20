use std::mem::swap;

fn gcd(a: usize, b: usize) -> usize {
    let mut r = (a, b);

    while r.0 != 0 {
        let q = r.1 / r.0;
        swap(&mut r.0, &mut r.1);
        r.0 -= q * r.1;
    }

    r.1
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
