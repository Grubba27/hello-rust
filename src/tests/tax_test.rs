use crate::tax::tax::blocked_in_source_tax;

#[cfg(test)]
mod tax_text {
    use super::*;

    #[test]
    pub fn money_below_1000_test_max() {
        let result = blocked_in_source_tax(999.0);
        assert_eq!(result, 0.0, "Given an input lower than 1000 it should return 0");
    }

    #[test]
    pub fn money_below_1000_test_min() {
        let result = blocked_in_source_tax(1.1);
        assert_eq!(result, 0.0, "Given an input lower than 1000 it should return 0");
    }

    #[test]
    pub fn tax_above_exact_test_max() {
        let result = blocked_in_source_tax(1000.0);
        assert_eq!(result, 100.0, "Given an input bigger than 1000 it should return 10%");
    }

    #[test]
    pub fn tax_above_exact_test_min() {
        let result = blocked_in_source_tax(2100.0);
        assert_eq!(result, 210.0, "Given an input bigger than 1000 it should return 10%");
    }
}
