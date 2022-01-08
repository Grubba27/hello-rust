use crate::fp::{calculate_big_salary, calculate_medium_salary, calculate_small_salary, curry_function};

#[cfg(test)]
mod fp_test {
    use super::*;

    #[test]
    pub fn small_salary_test() {
        let result = calculate_small_salary();
        assert_eq!(result, 1.0, "Testing taxes, with small should return 1.0");
    }

    #[test]
    pub fn medium_salary_test() {
        let result = calculate_medium_salary();
        assert_eq!(result, 1.0, "Testing taxes, with small should return 1.0");
    }

    #[test]
    pub fn big_salary_test() {
        let result = calculate_big_salary();
        assert_eq!(result, 12.2, "Testing taxes, with small should return 12.2");
    }
    #[test]
    pub fn curring_test() {
        let sum_fn = curry_function(10);
        let result = sum_fn(90);
        assert_eq!(result, 100 );
    }

}