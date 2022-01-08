///
/// big hard fucntion
pub fn big_hard_fn() -> f32 {
    0.30
}

pub fn other_big_hard_fn() -> f32 {
    0.50
}

fn http_request() -> f32 {
    return other_big_hard_fn();
}

pub fn very_big_hard_fn(function_calulate: f32, other_fn: f32) -> f32 {
    return 10.70 + function_calulate + other_fn;
}

pub fn big_hard_takes_arguments_fn(person_type: &str) -> f32 {
    0.70
}

pub fn calculate_salary_imposts_plus_transactions(salary: f64, person_type: &str, cost_per_transactions: f32) -> f32 {
    return if salary < 1000.0 {
        let person_trasaction = big_hard_takes_arguments_fn(person_type);
        let result = person_trasaction + big_hard_fn();
        result
    } else if salary > 1000.0 && salary < 2000.0 {
        let person_trasaction = big_hard_takes_arguments_fn(person_type);
        let result = person_trasaction + big_hard_fn();
        result
    } else {
        let person_trasaction = big_hard_takes_arguments_fn(person_type);
        let result = person_trasaction + cost_per_transactions;
        result
    };
}

pub fn calculate_small_salary() -> f32 {
    calculate_salary_imposts_plus_transactions(900.32, "small", big_hard_fn())
}

pub fn calculate_medium_salary() -> f32 {
    calculate_salary_imposts_plus_transactions(1200.32, "medium", other_big_hard_fn())
}

pub fn calculate_big_salary() -> f32 {
    calculate_salary_imposts_plus_transactions(5000.32, "big", very_big_hard_fn(big_hard_fn(), http_request()))
}

pub fn curry_function(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}
