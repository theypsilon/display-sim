use enum_len_trait::EnumLen;
use num_traits::{FromPrimitive, ToPrimitive};

pub trait NextEnumVariant {
    fn next_enum_variant(&mut self) -> Result<(), &str>;
    fn previous_enum_variant(&mut self) -> Result<(), &str>;
}

impl<T> NextEnumVariant for T
where
    T: FromPrimitive + ToPrimitive + EnumLen,
{
    fn next_enum_variant(&mut self) -> Result<(), &str>
    where
        Self: FromPrimitive + ToPrimitive,
    {
        change_enum_variant(self, |u| u + 1)
    }

    fn previous_enum_variant(&mut self) -> Result<(), &str>
    where
        Self: FromPrimitive + ToPrimitive,
    {
        change_enum_variant(self, |u| if u == 0 { Self::len() - 1 } else { u - 1 })
    }
}

fn change_enum_variant<T: FromPrimitive + ToPrimitive + EnumLen>(instance: &mut T, action: impl Fn(usize) -> usize) -> Result<(), &str> {
    let mut changed = match instance.to_usize().and_then(|as_usize| FromPrimitive::from_usize(action(as_usize))) {
        Some(n) => n,
        None => FromPrimitive::from_usize(0).ok_or("Can't construct enum from 0.")?,
    };
    std::mem::swap(instance, &mut changed);
    Ok(())
}
