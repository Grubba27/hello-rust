use crate::problems::problems::biggest_sum;
use crate::problems::problems::get_nested_values;

#[cfg(test)]
mod problems_test {
    use super::*;

    #[test]
    pub fn biggest_sum_test() {
        let v1 = vec![1, 2 ,3];
        let v2 = vec![5, 5 ,5];
        let v3 = vec![3, 1 ,4];
        let big_vec = vec![v1, v2, v3,];
        let result = biggest_sum(big_vec);

        assert_eq!(result, 15, "Testing sum, should return always 15");
    }

    #[test]
    pub fn get_nested_values_1_test() {
        let str = String::from("(1+(2*3)+((8)/4))+1");
        let result = get_nested_values(str);
        assert_eq!(result, 3, "Testing depth should always return 3");
    }

    #[test]
    pub fn get_nested_values_2_test() {
        let str = String::from("(((4+3)/ (3*3(9+(4/2))))+3)");
        let result = get_nested_values(str);
        assert_eq!(result, 5, "Testing depth should always return 5");
    }
}