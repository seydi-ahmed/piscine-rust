use border_cross::{Car, Truck, Vehicle, all_models};

fn main() {
    let vehicles: Vec<Box<dyn Vehicle>> = vec![
        Box::new(Car {
            plate_nbr: "A3D5C7",
            model: "Model 3",
            horse_power: 325,
            year: 2010,
        }),
        Box::new(Truck {
            plate_nbr: "V3D5CT",
            model: "Ranger",
            horse_power: 325,
            year: 2010,
            load_tons: 40,
        }),
    ];
    println!("{:?}", all_models(vehicles));
}
