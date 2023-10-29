
fn is_leap_year(year: u32) -> bool {
    match year {
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use crate::leap_year::leap_year::is_leap_year;

    #[test]
    fn if_is_a_number_isnt_enough_to_be_a_leap_year() {
        let result = is_leap_year(1234);
        assert_eq!(result, false);
    }



}