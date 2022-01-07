pub mod problems {
    ///
    /// given a list of lists which of the list is the biggest sum
    /// !the result of the table!
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

    pub fn biggest_sum(val: Vec<Vec<i32>>) -> i32 {
        val.iter()
            .map(|x| x.iter().sum())
            .max()
            .unwrap()
    }
}