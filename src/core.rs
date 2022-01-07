pub mod core {
    ///
    /// Get 10 percent
    /// * `value_of_account` - does some
    pub fn get_ten_percent(value_of_account: f64) -> f64 {
        value_of_account * 0.10
    }

    /// pub fn print_type_of<T>(_: &T) {
//         println!("{}", std::any::type_name::<T>())
//     }


    ///```
    /// Delivery tax based on this table
    ///     |    test      |    result |
    ///     |--------------|-----------|
    ///     |80 =>   $     |   15      |
    ///     |80  < $ > 250 |   5       |
    ///     |250 <=   $    |   0       |
    ///
    ///
    ///
    ///```
    pub fn delivery_tax(money: u64) -> u64 {
        if money <= 80 {
            15
        } else if (80 < money) && (money < 250) {
            5
        } else {
            0
        }
    }
}