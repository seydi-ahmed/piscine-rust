use expected_variable::*;

// fn main() {
//     println!(
//         "{} close to it",
//         expected_variable("On_Point", "on_point").unwrap()
//     );
//     println!(
//         "{} close to it",
//         expected_variable("soClose", "so_close").unwrap()
//     );
//     println!(
//         "{:?}",
//         expected_variable("something", "something_completely_different")
//     );
//     println!(
//         "{} close to it",
//         expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch").unwrap()
//     );
// }

fn main() {
    match expected_variable("On_Point", "on_point") {
        Some(result) => println!("{}", result),
        None => println!("Strings are not similar enough"),
    }

    match expected_variable("soClose", "so_close") {
        Some(result) => println!("{}", result),
        None => println!("Strings are not similar enough"),
    }

    match expected_variable("something", "something_completely_different") {
        Some(result) => println!("{}", result),
        None => println!("Strings are not similar enough"),
    }

    match expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch") {
        Some(result) => println!("{}", result),
        None => println!("Strings are not similar enough"),
    }
}
