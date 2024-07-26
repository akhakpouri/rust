pub struct Car {
    pub color: String,
    pub transmission: Transmission,
    pub convertible: bool,
    pub mileage: u32,
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}
