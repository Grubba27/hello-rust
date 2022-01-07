pub mod tax {
    ///
    /// Returns the value that was blocked in the source based on this test 1000 > $ -> $ *.10
    pub fn blocked_in_source_tax(val: f64) -> i32 {
        if val < 1000 as f64 {
            0
        } else {
            (val * 0.10)
        }
    }
}