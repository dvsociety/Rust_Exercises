fn main() {
    let f = 86.0;
    println!("{}ºF son {}ºC", f, fahrenheit_to_celsius(f));
}

fn fahrenheit_to_celsius(fahrenheit_temperature: f32) -> f32 {
    let celsius = (fahrenheit_temperature - 32.0) / 1.8;
    celsius
}