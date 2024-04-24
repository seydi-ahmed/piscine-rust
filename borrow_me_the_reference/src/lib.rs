pub fn str_to_vec(s: String) -> Vec<String> {
    let mut tab_vec = Vec::new();

    for letter in s.chars() {
        tab_vec.push(letter.to_string());
    }

    tab_vec
}

pub fn delete_string_on_vec(tab_vec: Vec<String>, index: usize) -> Vec<String> {
    let mut tab_vec2 = Vec::new();
    let mut parcours: usize = 0;

    for letter in tab_vec {
        if parcours != index {
            tab_vec2.push(letter);
            parcours += 1;
        } else {
            parcours += 1;
        }
    }

    tab_vec2
}

pub fn delete_string_on_vec2(tab_vec: Vec<&String>, index: usize) -> Vec<&String> {
    let mut tab_vec2 = Vec::new();
    let mut parcours: usize = 0;

    for letter in tab_vec {
        if parcours != index {
            tab_vec2.push(letter);
            parcours += 1;
        } else {
            parcours += 1;
        }
    }

    tab_vec2
}

pub fn delete_and_backspace(s: &mut String) {
    let _s1 = String::from(s.to_string());
    let mut tab_vec: Vec<String> = str_to_vec(_s1);
    let mut parcours: usize = 0;
    let mut taille_vec: usize = tab_vec.len() - 1;

    while parcours < taille_vec {
        if tab_vec[parcours] == "-" {
            tab_vec = delete_string_on_vec(tab_vec, parcours);
            tab_vec = delete_string_on_vec(tab_vec, parcours - 1);
            taille_vec -= 2;
            parcours = 0;
        }else{
            parcours += 1;
        }
    }

    let mut tab_vec2 = tab_vec.iter().rev().collect::<Vec<&String>>();

    let mut parcours2: usize = 0;
    let mut taille_vec2: usize = tab_vec2.len() - 1;

    while parcours2 < taille_vec2 {
        if tab_vec2[parcours2] == "+" {
            tab_vec2 = delete_string_on_vec2(tab_vec2, parcours2);
            tab_vec2 = delete_string_on_vec2(tab_vec2, parcours2 - 1);
            taille_vec2 -= 2;
            parcours2 = 0;
        }else{
            parcours2 += 1;
        }
    }
    
    let tab_vec3 = tab_vec2.iter().rev().collect::<Vec<&&String>>();
    println!("{:?}", tab_vec3);


    s.clear();

    for letter in tab_vec3{
        s.push_str(&letter);
    }
}

// pub fn do_operations(v: &mut Vec<String>) {
//     let mut v1 : Vec<String> = Vec::new();

//     for letter in v{
//         v1.push(letter.to_string());
//     }

//     for letter in v1{
//         let tab_un_a_un : 
//     }

// }


// ***************************************************************************************

pub fn do_operations(v: &mut Vec<String>) {
    for equation in v.iter_mut() {
        let result: i32 = eval(equation); 
        *equation = result.to_string();
    }
}

fn eval(equation: &String) -> i32 {
    let mut result = 0;
    let mut operator = '+';
    let mut num = String::new();
    for c in equation.chars() {
        if c.is_numeric() {
            num.push(c);
        } else {
            if !num.is_empty() {
                
                let n: i32 = num.parse().unwrap();
                match operator {
                    '+' => result += n,
                    '-' => result -= n,
                    _ => (),
                }
                num.clear();
            }
            operator = c;
        }
    }
    
    let n: i32 = num.parse().unwrap();
    match operator {
        '+' => result += n,
        '-' => result -= n,
        _ => (),
    }
    result
}