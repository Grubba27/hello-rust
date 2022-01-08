use crate::fp::{calculate_big_salary, calculate_medium_salary, calculate_small_salary};

#[cfg(test)]
mod fp_test {
    use super::*;

    #[test]
    pub fn small_salary_test() {
        let result = calculate_small_salary();
        assert_eq!(result, 1.0, "Testing taxes, with small should return .3");
    }

    #[test]
    pub fn medium_salary_test() {
        let result = calculate_medium_salary();
        assert_eq!(result, 1.0, "Testing taxes, with small should return .8");
    }

    #[test]
    pub fn big_salary_test() {
        let result = calculate_big_salary();
        assert_eq!(result, 12.2, "Testing taxes, with small should return .62");
    }


}