pub fn reverse_it(v: i32) -> String {
    let mut v1 : i32 = v.clone();
    let mut new_str : String = String::new();

    if v*(-1) >= 0{
        v1 = -v1;
        new_str.push_str(&"-".to_string());
    }

    let mut tab : Vec<i32> = Vec::new();

    while v1 > 0 {
        let q : i32 = v1/10;
        let r : i32 = v1%10;
        tab.push(r);
        v1 = q;
    }

    for nombre in tab {
        new_str.push_str(&nombre.to_string());
    }

    for nombre in tab.rev() {
        new_str.push_str(&nombre.to_string());
    }

    new_str
}