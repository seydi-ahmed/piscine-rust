use serde_json::json;
use serde_json::Value;
use std::str::FromStr;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> Value {
    let mut total_calories = 0.0f64;
    let mut total_carbs = 0.0f64;
    let mut total_proteins = 0.0f64;
    let mut total_fats = 0.0f64;

    for food in foods {
        // Parse calorie values from strings to f64
        let calorie_kj = f64::from_str(&food.calories[0][..food.calories[0].len() - 2])
            .expect("Failed to parse calorie value");
        let calorie_kcal = f64::from_str(&food.calories[1][..food.calories[1].len() - 4])
            .expect("Failed to parse calorie value");
        
        // Calculate total calories considering portions
        let total_calories_per_portion = (calorie_kj + calorie_kcal * 4.184) * food.nbr_of_portions;
        total_calories += total_calories_per_portion;

        // Calculate total macros considering portions
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
    }

    // Round the totals to two decimal places
    let total_calories_rounded = (total_calories * 100.0).round() / 100.0;
    let total_proteins_rounded = (total_proteins * 100.0).round() / 100.0;
    let total_fats_rounded = (total_fats * 100.0).round() / 100.0;
    let total_carbs_rounded = (total_carbs * 100.0).round() / 100.0;

    // Build the JSON object
    json!({
        "cals": total_calories_rounded,
        "carbs": total_carbs_rounded,
        "proteins": total_proteins_rounded,
        "fats": total_fats_rounded,
    })
}




    // "cals": 2777.39,
    // "carbs": 322.44,
    // "proteins": 122.06,
    // "fats": 106.93

