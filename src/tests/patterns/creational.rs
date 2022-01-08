use crate::creational::singleton;

#[cfg(test)]
mod creational_test {
    use super::*;

    #[test]
    fn get_singleton_test() {
        let singleton = singleton::get_singleton();
        let result = singleton.lock().unwrap();
        assert_eq!(result.global_const, "Global string".to_string(), "Created singleton should have the name global string");
    }

    #[test]
    fn update_singleton() {
        let singleton1 = singleton::get_singleton();
        {
            singleton::set_singleton("other", singleton1);
        }
        let updated_singleton = singleton::get_singleton();
        let result = updated_singleton.lock().unwrap();
        assert_eq!(result.global_const, "other", "Altered singleton should have the name other");
    }
}