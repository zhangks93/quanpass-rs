
pub fn get_precision(num: f32) -> i32 {
    let mut decimal = num - num.floor();
    print!("{:?}", decimal);
    if decimal == 0.0 {
        return 0;
    }
    (decimal.to_string().len() - 2).try_into().unwrap()

}

#[cfg(test)]
mod tests {
    use crate::util::number_util::get_precision;

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
}
