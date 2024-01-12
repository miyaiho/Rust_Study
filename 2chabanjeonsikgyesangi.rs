use std::io;



fn main() {
    let mut input = String::new();

    println!("x^2의 계수는? : "); 
    io::stdin().read_line(&mut input).expect("에러");
    let a: i32 = input.trim().parse().expect("이상한 거");

    input.clear();

    println!("{}x^2", a);

    println!("x의 계수는? : ");
    io::stdin().read_line(&mut input).expect("에러");
    let b: i32 = input.trim().parse().expect("이상한 거");

    input.clear();

    println!("{}x^2 + {}x", a, b);

    println!("상수항은? : ");
    io::stdin().read_line(&mut input).expect("에러");
    let c: i32 = input.trim().parse().expect("이상한 거");
    
    input.clear();

    println!("{}x^2 + {}x + {}", a, b, c);

    let amt: i32 = a * 2;

    let panbyeolsik: i32 = b * b - 4 * a * c;
    let rpanbyeolsick: f64 = (panbyeolsik as f64).sqrt();

    let mab: f64 = -1.0 * (b as f64) - rpanbyeolsick;
    let pab: f64 = -1.0 * (b as f64) + rpanbyeolsick;

    let ma: f64 = mab / (amt as f64);
    let pa: f64 = pab / (amt as f64);

    println!("x = {} or x = {}", pa, ma);
}