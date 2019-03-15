use num_traits::{FromPrimitive, ToPrimitive};

pub trait NextEnumVariant {
    fn next_enum_variant(&mut self) -> Result<(), &str>
    where
        Self: FromPrimitive + ToPrimitive,
    {
        let mut incremented = match self.to_usize().and_then(|as_usize| FromPrimitive::from_usize(as_usize + 1)) {
            Some(n) => n,
            None => FromPrimitive::from_usize(0).ok_or("Can't construct enum from 0.")?,
        };
        std::mem::swap(self, &mut incremented);
        Ok(())
    }
}

impl<T> NextEnumVariant for T where T: FromPrimitive + ToPrimitive {}
