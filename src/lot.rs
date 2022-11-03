pub struct Lot {
    pub title: String,
    pub count: i32,
}

impl Lot {
    pub fn get_keywords(&self) -> Vec<String> {
        self.title
            .split(' ')
            .map(String::from)
            .filter(|x| {
                !x.to_lowercase().contains("артбук")
                    && !x.to_lowercase().contains("мир")
                    && !x.to_lowercase().contains("игр")
                    && !x.to_lowercase().contains("искусство")
            })
            .collect()
    }
}
