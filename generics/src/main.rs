use num_traits::ToPrimitive;

fn solve(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

fn main() {
    let a:f32 = 3.0;
    // let a_64 = a as f64;
    let a_64 = a.to_f64().unwrap();
    let b = 4.0;

    println!("{}", solve(a_64, b));
}
