use serde::{Deserialize, Deserializer};

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Wrapper<T> {
    data: T,
}

impl<T> Wrapper<T> {
    fn deserialize<'de, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        let wrapper = <Self as Deserialize>::deserialize(deserializer)?;
        Ok(wrapper.data)
    }
}
