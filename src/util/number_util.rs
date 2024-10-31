use rand::{thread_rng, Rng};

pub fn get_precision(num: f64) -> u32 {
    let temp = num.to_string();
    if temp.contains(".") {
        let splited: Vec<&str> = temp.split(".").collect();
        splited[1].len().try_into().unwrap()
    } else {
        0
    }
}

pub fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i32.pow(decimals) as f64;
    (x * y).round() / y
}

pub fn get_random_number() -> u32 {
    thread_rng().gen_range(1..=1_000_000)
}

pub fn smart_quantity(order_size: f64, price: f64) -> f64 {
    let mut precision = 0;
    if price >= 800.0 {
        precision = 3;
    } else if price >= 100.0 {
        precision = 2;
    } else if price >= 20.0 {
        precision = 1;
    } else {
        precision = 0;
    }
    round(order_size / price, precision)
}

#[cfg(test)]
mod tests {
    use crate::util::number_util::{get_precision, get_random_number, round};

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

    #[test]
    fn test_rand() {
        println!("{}", (get_random_number() as f64 * 5.0 + 2.0) / 100.0);
    }
}
