use crate::creational::factory_method::VehicleTypes::{Car as CarType, Plane as PlaneType, Boat as BoatType};

pub trait Vehicle {
    fn translocate(&self) -> &str;
}

pub enum VehicleTypes {
    Car,
    Boat,
    Plane,
}

struct Car {}

impl Vehicle for Car {
    fn translocate(&self) -> &str {
        println!("Moved in land");
        "Moved in land"
    }
}


struct Boat {}

impl Vehicle for Boat {
    fn translocate(&self) -> &str {
        println!("Moved in water");
        "Moved in water"
    }
}

struct Plane {}

impl Vehicle for Plane {
    fn translocate(&self) -> &str {
        println!("Moved in air");
        "Moved in air"
    }
}

pub struct ShapeFactory;

impl ShapeFactory {
    pub fn new_vehicle(v: &VehicleTypes) -> Box<dyn Vehicle> {
        match v {
            VehicleTypes::Car => Box::new(Car {}),
            VehicleTypes::Boat => Box::new(Boat {}),
            VehicleTypes::Plane => Box::new(Plane {}),
        }
    }
}
