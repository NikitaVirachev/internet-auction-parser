#[derive(Debug)]
pub struct Lot {
    pub id: String,
    pub title: String,
    pub url: String,
    pub count: i32,
}

impl Lot {
    pub fn get_keywords(&self) -> Vec<String> {
        self.title
            .split(|c| {
                c == ':' || c == ' ' || c == '-' || c == '—' || c == '/' || c == '.' || c == ','
            })
            .map(|word| String::from(word).to_lowercase())
            .filter(|x| {
                !(x.contains("артбук")
                    || x.contains("мир")
                    || x.contains("игр")
                    || x.contains("искусство")
                    || x.contains("artbook")
                    || x.contains("of"))
                    && x.len() > 0
                    && x != "и"
                    && x != "art"
            })
            .collect()
    }
}
