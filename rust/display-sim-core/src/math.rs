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
mod test_gcd {
    use super::gcd;
    #[test]
    fn test_gcd_gives_good_results() {
        assert_eq!(0u32, gcd(0u32, 0u32));
        assert_eq!(10u32, gcd(10u32, 0u32));
        assert_eq!(10u32, gcd(0u32, 10u32));
        assert_eq!(10u32, gcd(10u32, 20u32));
        assert_eq!(44u32, gcd(2024u32, 748u32));
    }
}

pub fn radians(grad: f32) -> f32 {
    let pi: f32 = glm::pi();
    grad * pi / 180.0
}

#[cfg(test)]
mod test_radians {
    use super::radians;
    #[test]
    fn test_radians_gives_good_results() {
        assert_eq!(2.0 * glm::pi::<f32>(), radians(360.0));
        assert_eq!(0.0, radians(0.0));
    }
}
