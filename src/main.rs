use std::io;

const C: f32 = 32.0;

fn main() {
    println!("F to C: 0");
    println!("C to F: 1");

    let (type_convert, tempirature) = input_values();

    if type_convert > 1 {
        panic!("Use only 0 || 1")
    }

    if type_convert == 0 {
        let converted_tempirature = f_to_c(tempirature);
        println!(
            "Selected: {}, tempirature F: {}, tempirature C: {}",
            type_convert, tempirature, converted_tempirature
        )
    }

    if type_convert == 1 {
        let converted_tempirature = c_to_f(tempirature);
        println!(
            "Selected: {}, tempirature C: {}, tempirature F: {}",
            type_convert, tempirature, converted_tempirature
        )
    }
}

fn input_values() -> (u8, f32) {
    let mut convert_type = String::new();
    let mut tempirature = String::new();

    io::stdin().read_line(&mut convert_type).unwrap();
    let convert_type: u8 = convert_type.trim().parse::<u8>().expect("Use numbers");

    io::stdin().read_line(&mut tempirature).unwrap();
    let tempirature: f32 = tempirature.trim().parse::<f32>().expect("Use numbers");

    return (convert_type, tempirature);
}

fn c_to_f(value: f32) -> f32 {
    value * (9.0 / 5.0) + C
}

fn f_to_c(value: f32) -> f32 {
    (value - C) * (5.0 / 9.0)
}
