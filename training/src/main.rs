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
    let mut total_cals :f64 = 0.0;
    let mut total_carbs :f64 = 0.0;
    let mut total_proteins :f64 = 0.0;
    let mut total_fats :f64 = 0.0;

    for food in foods {
        let cal = food.calories[1].as_str().split('k').collect::<Vec<_>>()[0];
        let cal_num : f64 = cal.parse().unwrap();
        total_cals += food.nbr_of_portions * cal_num;
        total_carbs += food.nbr_of_portions * food.carbs;
        total_proteins += food.nbr_of_portions * food.proteins;
        total_fats += food.nbr_of_portions * food.fats;
    }


    object! {
        "cals": (total_cals*100.0).round() / 100.0,
        "carbs": (total_carbs*100.0).round() / 100.0,
        "proteins": (total_proteins*100.0).round() / 100.0,
        "fats": (total_fats*100.0).round() / 100.0
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