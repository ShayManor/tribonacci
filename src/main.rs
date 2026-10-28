use rug::Integer;
use rug::integer::IsPrime;


fn main() {
    let mut cur = Integer::from(1);
    let mut mid = Integer::from(1);
    let mut last = Integer::from(1);
    let mut i: u128 = 0;
    println!("{}", last);
    println!("{}", mid);
    println!("{}", cur);
    loop {
        let cur_copy = cur.clone();
        let mut next = Integer::from(&cur + &mid);
        next += last;
        cur = next;
        last = mid;
        mid = cur_copy;
        println!("{}", cur);
//         if cur.is_probably_prime(30) != IsPrime::No {
//             println!("Index {i} has probable prime {cur}");
//         }
        i += 1;
    }

}