mod area;
mod celsius_to_fahrenheit;
mod number_analyser;

fn main(){
  let width = 12.4;
  let height = 21.34;
  let temp: f64 = 25.0;
  let number: i32 = 25;

  let fah = celsius_to_fahrenheit::celsius_to_fahrenheit(temp);
  let area = area::calculate_area(height, width);

  println!("Width: {}", width);
  println!("Height: {}", height);
  println!("Area: {}", area);
  println!("Celsius to Fahrenheit: {}°C = {}°F", temp, fah);
  println!("Number: {}", number);
  number_analyser::number_analyser(number);
}