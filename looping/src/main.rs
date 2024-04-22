use std::io;

fn main() -> io::Result<()> {
    let _riddle : &str = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let mut number_of_trials: u32 = 1;
    
    loop {
        let mut response = String::new();
        println!("{}", _riddle);
        io::stdin().read_line(&mut response)?;
        if (response).trim() == "The letter e" {
            println!("Number of trials: {}", number_of_trials);
            break;
        }
        number_of_trials += 1;
    }

    Ok(())
}
