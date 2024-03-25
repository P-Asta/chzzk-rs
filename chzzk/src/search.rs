pub trait SearchInfo {}
pub struct Search {
    pub search_type: SearchType,
    pub keyword: String,
    pub offset: usize,
    pub size: usize,
}

pub enum SearchType {
    Channel,
    Video,
    Live,
}

impl Default for Search {
    fn default() -> Self {
        Search {
            search_type: SearchType::Live,
            keyword: "5-23".to_string(),
            offset: 0,
            size: 50,
        }
    }
}
