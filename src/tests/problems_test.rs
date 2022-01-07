use crate::problems::problems::biggest_sum;

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


}