// use super::wrapper::Wrapper;
use serde::{Deserialize, Deserializer};

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Wrapper<T> {
    pub data: T,
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

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Continent {
    pub id: u64,
    pub name: String,
    #[serde(with = "Wrapper", default)]
    pub countries: Option<Vec<Country>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Country {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub extra: Option<CountryExtra>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountryExtra {
    pub continent: Option<String>,
    pub sub_region: Option<String>,
    pub world_region: Option<String>,
    pub fifa: Option<String>,
    pub iso: Option<String>,
    pub longitude: Option<String>,
    pub latitude: Option<String>,
    pub flag: Option<String>,
}
