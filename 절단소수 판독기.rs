use std::io;

fn is_sosu(num: u32) -> bool {
    (2..=(num as f64).sqrt() as u32).all(|i| num % i != 0)
}

fn is_jeoldan_sosu(num: u32) -> bool {
    let num_str = num.to_string();
    let num_len = num_str.len();

    (1..num_len)
        .all(|i| is_sosu(num_str[i..].parse().unwrap()) && is_sosu(num_str[..num_len - i].parse().unwrap()))
}

fn main() {
    for num in 1..=u32::MAX {
        if is_jeoldan_sosu(num) && !(num >= 0 && num <= 9) {
            println!("절단소수: {}", num);
        }
    }
}
