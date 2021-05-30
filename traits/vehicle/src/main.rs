trait Vehicle {
    fn class(&self) -> String;
    fn repair(&mut self) {}
    fn honk(&self) -> String {
        "Tuuut!!".to_string()
    }
}

struct Car {
    broken: bool,
}
impl Vehicle for Car {
    fn wheels(&self) -> u8 {
        4
    }

    fn class(&self) -> String {
        String::from("Car")
    }

    fn repair(&mut self) {
        self.broken = false;
    }
}

enum BicycleType {
    Uni,
    Classic,
    Tri,
}

struct Bicycle {
    kind: BicycleType,
}
impl Vehicle for Bicycle {
    fn wheels(&self) -> u8 {
        match self.kind {
            BicycleType::Uni => 1,
            BicycleType::Classic => 2,
            BicycleType::Tri => 3,
        }
    }

    fn class(&self) -> String {
        match self.kind {
            BicycleType::Uni => String::from("Unicycle"),
            BicycleType::Classic => String::from("Bicycle"),
            BicycleType::Tri => String::from("Tricycle"),
        }
    }
}

fn how_many_wheels<T: Vehicle>(vehicle: &T) -> u8 {
    vehicle.wheels()
}

fn main() {
    let car = Car { broken: false };
    println!("{}: {} wheel(s)", car.class(), how_many_wheels(&car));

    let unicycle = Bicycle {
        kind: BicycleType::Uni,
    };
    println!(
        "{}: {} wheel(s)",
        unicycle.class(),
        how_many_wheels(&unicycle)
    );
}
