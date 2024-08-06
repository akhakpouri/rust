use core::fmt;

pub struct Car {
    pub color: String,
    pub transmission: Transmission,
    pub convertible: bool,
    pub mileage: u32,
}

#[derive(PartialEq, Debug)]
pub enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

impl fmt::Display for Transmission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Transmission::Manual => write!(f, "manual"),
            Transmission::SemiAuto => write!(f, "semi-auto"),
            Transmission::Automatic => write!(f, "auto"),
        }
    }
}

pub fn car_factory(color: String, transmission: Transmission, is_convirtible: bool) -> Car {
    Car {
        color: color,
        convertible: is_convirtible,
        mileage: 5000,
        transmission: transmission,
    }
}
