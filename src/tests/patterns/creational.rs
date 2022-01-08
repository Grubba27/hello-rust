use crate::creational::{singleton, factory_method, builder};

#[cfg(test)]
mod creational_test {
    use crate::creational::builder::use_case_one_part;
    use crate::creational::builder::use_case_many_parts;
    use crate::creational::factory_method::VehicleTypes;
    use super::*;

    #[test]
    fn get_singleton_test() {
        let singleton = singleton::get_singleton();
        let result = singleton.lock().unwrap();
        assert_eq!(result.global_const, "Global string".to_string(), "Created singleton should have the name global string");
    }

    #[test]
    fn update_singleton_test() {
        let singleton1 = singleton::get_singleton();
        {
            singleton::set_singleton("other", singleton1);
        }
        let updated_singleton = singleton::get_singleton();
        let result = updated_singleton.lock().unwrap();
        assert_eq!(result.global_const, "other", "Altered singleton should have the name other");
    }


    #[test]
    fn abstract_factory_create_car() {
        let car = factory_method::ShapeFactory::new_vehicle(&VehicleTypes::Car);
        let result = car.translocate();
        assert_eq!(result, "Moved in land", "Creating a car with a type only ");
    }

    #[test]
    fn abstract_factory_create_boat() {
        let boat = factory_method::ShapeFactory::new_vehicle(&VehicleTypes::Boat);
        let result = boat.translocate();
        assert_eq!(result, "Moved in water", "Creating a boat with a type only ");
    }

    #[test]
    fn abstract_factory_create_plane() {
        let plane = factory_method::ShapeFactory::new_vehicle(&VehicleTypes::Plane);
        let result = plane.translocate();
        assert_eq!(result, "Moved in air", "Creating a plane with a type only ");
    }

    #[test]
    fn builder_create_one_part_test(){
        let result = use_case_one_part();
        let compare = vec![String::from("parte a")];
        assert_eq!(result, compare , "Comparing builder");
    }

    #[test]
    fn builder_create_many_parts_test(){
        let result = use_case_many_parts();
        let compare = vec![String::from("parte a"), String::from("parte 2"), String::from("parte Z") ];
        assert_eq!(result, compare , "Comparing builder");
    }
}