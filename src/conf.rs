#[derive(Deserialize, Default)]
pub struct Config {
    pub include: Includes,
}

#[derive(Deserialize, Default)]
pub struct Includes {
    pub js: Vec<String>,
    pub css: Vec<String>,
}
