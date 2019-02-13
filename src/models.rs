use super::wrapper::Wrapper;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Continent {
    pub id: u64,
    pub name: String,
    #[serde(with = "Wrapper", default)]
    pub countries: Wrapper<Vec<Country>>,
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
    continent: Option<String>,
    sub_region: Option<String>,
    world_region: Option<String>,
    fifa: Option<String>,
    iso: Option<String>,
    longitude: Option<String>,
    latitude: Option<String>,
    flag: Option<String>,
}