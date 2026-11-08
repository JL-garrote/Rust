fn celsius_a_fahrenheit(temperatura: f64) -> f64 {
    let a = temperatura * 1.8 + 32.0;
    return a;
}

fn main() {
    println!("100 °C son {} °F", celsius_a_fahrenheit(100.0));
}