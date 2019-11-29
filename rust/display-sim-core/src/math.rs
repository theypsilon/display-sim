use std::ops::Rem;
use num_traits::Zero;

pub fn gcd<T>(a: T, b: T) -> T
where T: Zero + Copy + PartialOrd + Rem<Output = T> {
    let (mut a, mut b) = if a > b {
        (a, b)
    } else {
        (b, a)
    };
    while !b.is_zero() {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

#[cfg(test)]
mod test {
    use super::gcd;
    #[test]
    fn test_gcd() {
        assert_eq!(0, gcd(0, 0));
        assert_eq!(10, gcd(10, 0));
        assert_eq!(10, gcd(0, 10));
        assert_eq!(10, gcd(10, 20));
        assert_eq!(44, gcd(2024, 748));
    }
}
