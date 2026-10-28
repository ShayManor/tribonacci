use num_bigint::BigUint;
use num_traits::One;

fn main() {
    let mut cur = BigUint::ONE;
    let mut mid = BigUint::ONE;
    let mut last = BigUint::ONE;
    println!("{}", last);
    println!("{}", mid);
    println!("{}", cur);
    loop {
        let cur_copy = cur.clone();
        cur = &cur + &mid + &last;
        last = mid;
        mid = cur_copy;
        println!("{}", cur);
    }

}