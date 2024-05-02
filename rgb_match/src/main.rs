use rgb_match::*;

fn main() {
    let c = Color {
        r: 255,
        g: 200,
        b: 10,
        a: 30,
    };

    println!("{:?}", c.clone().swap(c.r, c.b));
    println!("{:?}", c.clone().swap(c.r, c.g));
    println!("{:?}", c.clone().swap(c.r, c.a));
    println!();
    println!("{:?}", c.clone().swap(c.g, c.r));
    println!("{:?}", c.clone().swap(c.g, c.b));
    println!("{:?}", c.clone().swap(c.g, c.a));
    println!();
    println!("{:?}", c.clone().swap(c.b, c.r));
    println!("{:?}", c.clone().swap(c.b, c.g));
    println!("{:?}", c.clone().swap(c.b, c.a));
    println!();
    println!("{:?}", c.clone().swap(c.a, c.r));
    println!("{:?}", c.clone().swap(c.a, c.b));
    println!("{:?}", c.clone().swap(c.a, c.g));
}
