
pub struct Score {
    pub value: f32
}

impl Score {
    pub fn from_float(value: f32) -> Self {
        Score { value }
    }

    pub fn from_int(value: i32) -> Self {
        Score { value: value as f32 }
    }

    pub fn from_usize(value: usize) -> Self {
        Score { value: value as f32 }
    }

    pub fn from_string(value: &str) -> Self {
        Score { value: value.parse::<f32>().unwrap() }
    }

    pub fn to_float(&self) -> f32 {
        self.value
    }

    pub fn to_int(&self) -> i32 {
        self.value as i32
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }

    pub fn to_usize(&self) -> usize {
        self.value as usize
    }
}

impl Clone for Score {
    fn clone(&self) -> Self {
        Score { value: self.value }
    }
}

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl std::fmt::Debug for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}