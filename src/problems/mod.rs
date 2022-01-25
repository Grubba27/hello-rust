pub mod problems {
    /// given a list of lists which of the list is the biggest sum
    /// as the result of the table!
    ///
    /// _Table_
    /// ```
    ///
    /// | Values |   Sum     | Result|  Winner|
    /// |--------|-----------|-------|--------|
    /// | 1 2 3  | 1 + 2 + 3 |  6    |        |
    /// | 5 5 5  | 5 + 5 + 5 |  15   |   X    |
    /// | 3 1 4  | 3 + 1 + 4 |  8    |        |
    /// ```
    pub fn biggest_sum(nums: Vec<Vec<i32>>) -> i32 {
        nums.iter()
            .map(|x| x.iter().sum())
            .max()
            .unwrap()
    }

    /// Given a nest of values  in a string give me the biggest depth
    /// ```
    ///   (1+(2*3)+((8)/4))+1 -> return 3
    ///   (((4+3)/ (3*3(9+(4/2))))+3)
    /// ```
    ///
    pub fn get_nested_values(strs: String) -> i32 {
        strs.chars()
            .filter(|&char| "()".contains(char))
            .map(|char| if char == '(' { 1 } else { -1 })
            .scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            })
            .max()
            .unwrap_or(0)
    }


    pub fn find_digits(nums: Vec<i32>) -> i32 {
        nums.iter()
            .map(|item| item.to_string().len())
            .filter(|item| item % 2 == 0)
            .count() as i32
    }
}
