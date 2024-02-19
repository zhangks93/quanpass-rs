use chrono::{Utc};

pub fn hours_ago_timestamp(hours: i64) -> i64 {
    (Utc::now().timestamp() - hours * 3600) * 1000
}

pub fn minutes_ago_timestamp(minutes: i64) -> i64 {
    (Utc::now().timestamp() - minutes * 60) * 1000
}

#[cfg(test)]
mod tests {
    use super::{hours_ago_timestamp};


    #[test]
    fn test_excute() {
        println!("{:?}", hours_ago_timestamp(1)) 
    }
}