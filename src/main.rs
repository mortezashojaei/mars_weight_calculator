use std::io;


fn main() {
    println!("Enter your weight in KG:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let weight:f32 = input.trim().parse().unwrap();
    let mars_weight = canculate_weight_on_mars(weight) ;
    println!("result:{} {}",mars_weight ,"KG")
}

fn canculate_weight_on_mars(weight: f32)->f32{
    weight*3.711/9.81
}
