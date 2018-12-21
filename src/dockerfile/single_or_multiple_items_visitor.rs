use serde::de::{SeqAccess, Visitor};
use std::fmt::{self, Formatter};

pub struct SingleOrMultipleItemsVisitor;

impl<'de> Visitor<'de> for SingleOrMultipleItemsVisitor {
    type Value = Vec<String>;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "a string or a sequence of strings")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(vec![value.to_owned()])
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(vec![value.to_owned()])
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(vec![value])
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut elements = if let Some(size) = sequence.size_hint() {
            Vec::with_capacity(size)
        } else {
            Vec::new()
        };

        while let Some(element) = sequence.next_element()? {
            elements.push(element)
        }

        Ok(elements)
    }
}
