#[derive(Debug, PartialEq, Clone)]
pub enum Plan {
    Free,
    Standard,
}

impl Plan {
    pub fn name(&self) -> String {
        match self {
            Plan::Free => "フリープラン".to_string(),
            Plan::Standard => "スタンダードプラン".to_string(),
        }
    }

    pub fn max_borrowing_count(&self) -> u32 {
        match self {
            Plan::Free => 2,
            Plan::Standard => 10,
        }
    }
}
