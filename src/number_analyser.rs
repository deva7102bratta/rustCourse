pub fn number_analyser(number: i32){
  if number<0{
    println!("Negative");
  }else{
    println!("Positive");
  }
  if number%2==0{
    println!("Even");
  }else{
    println!("Odd")
  }
  for i in 0..=number{
    println!("{}", i)
  }
}