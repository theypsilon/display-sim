use std::ops::Rem;

pub trait NonNegativeInteger: Copy + PartialOrd + Rem<Output = Self> + From<u8> {}

impl NonNegativeInteger for u8 {}
impl NonNegativeInteger for u16 {}
impl NonNegativeInteger for u32 {}
impl NonNegativeInteger for u64 {}
impl NonNegativeInteger for u128 {}
impl NonNegativeInteger for usize {}

pub fn gcd<T: NonNegativeInteger>(a: T, b: T) -> T {
    let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };
    while b != 0.into() {
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
        assert_eq!(0u32, gcd(0u32, 0u32));
        assert_eq!(10u32, gcd(10u32, 0u32));
        assert_eq!(10u32, gcd(0u32, 10u32));
        assert_eq!(10u32, gcd(10u32, 20u32));
        assert_eq!(44u32, gcd(2024u32, 748u32));
    }
}
