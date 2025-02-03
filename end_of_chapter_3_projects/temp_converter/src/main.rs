use std::io;
fn main() {
    loop {
        println!("What format is your temp in? C/F: ");

        let mut temp_format = String::new();

        io::stdin()
            .read_line(&mut temp_format)
            .expect("Invalid format");

        if temp_format.trim() == "C" {
            println!("Please input you temp: ");

            let mut temp = String::new();

            io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line");

            let temp: f32 = temp.trim().parse().expect("Invalid temp enter a number");

            let temp_f: f32 = temp * 1.8 + 32.0;
            println!("Your temp in Fahrenheit is: {temp_f}");
            break;
        } else if temp_format.trim() == "F" {
            println!("Please input you temp: ");

            let mut temp = String::new();

            io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line");

            let temp: f32 = temp.trim().parse().expect("Invalid temp enter a number");

            let temp_c: f32 = (temp - 32.0) / 1.8;
            println!("Your temp in Celsius is: {temp_c}");
            break;
        } else {
            println!("Invalid format");
        }
    }
}
