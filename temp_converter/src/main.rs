// Convert temp from F to C
// define which measurement unit (F or C)
// dependent on unit, convert from one to another
use std::io;

fn main(){
    let directory = ["f", "c"];
    let mut unit = String::new();
    let mut temp = String::new();

    println!("Please enter the unit (F or C)");
    io::stdin().read_line(&mut unit).expect("Failed to read line");
    unit.pop();
    println!("Please enter temperature");
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    let temp: f32 = match temp.trim().parse(){
            Ok(num) => num,
            Err(_) => 0.0,
        };
    if unit.trim().to_lowercase() == directory[0]{
        f_c(temp);
    } else {
        c_f(temp);
    }
}

fn c_f(cel: f32){
    let faren: f32;
    faren = cel * 1.8 + 32.0;
    println!("{} F", faren)

}

fn f_c(faren: f32){
    let cel: f32;
    cel = (faren - 32.0) * 5.0/9.0;
    println!("{} C", cel)
    
}