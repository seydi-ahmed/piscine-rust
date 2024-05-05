pub use json::{self,object};

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64
}


pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut total_cals : f64 = 0.0;
    let mut total_fats : f64 = 0.0;
    let mut total_carbs : f64 = 0.0;
    let mut total_proteins : f64 = 0.0;

    for fd in foods {
        let cl = fd.calories[1].as_str().split("k").collect::<Vec<_>>()[0];
        let cl_num : f64 = cl.parse().unwrap_or(0.0);

        total_cals += cl_num * fd.nbr_of_portions;
        total_fats += fd.fats * fd.nbr_of_portions;
        total_carbs += fd.carbs * fd.nbr_of_portions;
        total_proteins += fd.proteins * fd.nbr_of_portions;
    }

    object! {
        "cals": (total_cals * 100.0).round()/100.0,
        "carbs": (total_carbs * 100.0).round()/100.0,
        "proteins": (total_proteins * 100.0).round()/100.0,
        "fats": (total_fats * 100.0).round()/100.0
    }
}


fn main(){
    let a = vec![
        Food {
            name: String::from("big mac"),
            calories: ["2133.84kJ".to_string(), "510kcal".to_string()],
            proteins: 27.0,
            fats: 26.0,
            carbs: 41.0,
            nbr_of_portions: 2.0,
        },
        Food {
            name: "pizza margherita".to_string(),
            calories: ["1500.59kJ".to_string(), "358.65kcal".to_string()],
            proteins: 13.89,
            fats: 11.21,
            carbs: 49.07,
            nbr_of_portions: 4.9,
        },
    ];

    println!("{:#}", calculate_macros(a));
}