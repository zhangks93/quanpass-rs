use rust_decimal::Decimal;

pub fn get_precision(num: f64) -> u32 {
    let temp = num.to_string();
    if temp.contains(".") {
        let splited:Vec<&str> = temp.split(".").collect();
        splited[1].len().try_into().unwrap()
    }else {
        0
    }

}

pub fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i32.pow(decimals) as f64;
    (x * y).round() / y
}

#[cfg(test)]
mod tests {
    use crate::util::number_util::{get_precision, round};

    #[test]
    fn test_get_precision() {
        assert_eq!(get_precision(3.3), 1);
    }

    #[test]
    fn test_get_precision_2() {
        assert_eq!(get_precision(3.1415), 4);
    }

    #[test]
    fn test_get_precision_3() {
        assert_eq!(get_precision(14.0), 0);
    }

    

    #[test]
    fn test_round() {
        assert_eq!(round(3.1415, 2), 3.14);
        assert_eq!(round(3.1415, 0), 3.0);
    }
}
