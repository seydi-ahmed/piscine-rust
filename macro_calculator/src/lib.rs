pub use json::{self,object};

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for fd in foods {
        let cl = fd.calories[1].as_str().split("k").collect::<Vec<_>>()[0];
        let cl_num : f64 = cl.parse().unwrap_or(0.0);
        total_cals += cl_num * fd.nbr_of_portions;
        total_proteins += fd.proteins * fd.nbr_of_portions;
        total_fats += fd.fats * fd.nbr_of_portions;
        total_carbs += fd.carbs * fd.nbr_of_portions;
    }

    object! {
        "cals": (total_cals*100.0).round()/100.0,
        "carbs": (total_carbs*100.0).round()/100.0,
        "proteins": (total_proteins*100.0).round()/100.0,
        "fats": (total_fats*100.0).round()/100.0,
    }
}

    // "cals": 2777.39,
    // "carbs": 322.44,
    // "proteins": 122.06,
    // "fats": 106.93
