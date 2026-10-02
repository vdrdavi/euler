use num_bigint::BigInt;

pub fn twenty() {
    let n: i32 = 100;
    let mut fatorial: BigInt = BigInt::from(1);
    let mut soma: i32 = 0;
    for x in 2..=n {
        fatorial *= x;
    }
    for x in fatorial.to_string().chars() {
        soma += x.to_string().parse::<i32>().unwrap();
    }

    println!("{soma}");
}
