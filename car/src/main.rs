use car::car_factory;

mod car;

fn main() {
    let mut car = car_factory(String::from("Blue"), car::Transmission::Automatic, true);

    println!(
        "{} car is convertible {} has {} miles while driving a {} transmition",
        car.color, car.convertible, car.mileage, car.transmission
    );
    car = car_factory(String::from("Red"), car::Transmission::SemiAuto, false);
    println!(
        "{} car is convertible {} has {} miles while driving a {} transmition",
        car.color, car.convertible, car.mileage, car.transmission
    );
    car = car_factory(String::from("Yellow"), car::Transmission::SemiAuto, false);
    println!(
        "{} car is convertible {} has {} miles while driving a {} transmition",
        car.color, car.convertible, car.mileage, car.transmission
    );

    car = car_factory(String::from("Silver"), car::Transmission::Manual, false);
    println!(
        "{} car is convertible {} has {} miles while driving a {} transmition",
        car.color, car.convertible, car.mileage, car.transmission
    );
}
