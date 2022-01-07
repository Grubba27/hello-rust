use crate::core::core::delivery_tax;

#[cfg(test)]
mod core_test {
    use super::*;

    #[test]
    pub fn tax_below_80_with_1_test() {
        let result = delivery_tax(1);
        assert_eq!(result, 15, "Testing delivery taxes, should return 15");
    }

    #[test]
    pub fn tax_below_80_with_80_test() {
        let result = delivery_tax(80);
        assert_eq!(result, 15, "Testing delivery taxes, should return 15");
    }

    #[test]
    pub fn tax_between_normal_test() {
        let result = delivery_tax(85);
        assert_eq!(result, 5, "Testing delivery taxes, should return 5");
    }

    #[test]
    pub fn tax_between_exact_test() {
        let result = delivery_tax(249);
        assert_eq!(result, 5, "Testing delivery taxes, should return 5");
    }


    #[test]
    pub fn tax_bigger_normal_test() {
        let result = delivery_tax(250);
        assert_eq!(result, 0, "Testing delivery taxes, should return 0");
    }

    #[test]
    pub fn tax_bigger_exact_test() {
        let result = delivery_tax(300);
        assert_eq!(result, 0, "Testing delivery taxes, should return 0");
    }

}