use num_bigint::BigInt;

pub fn sixteen() {
    let dois:BigInt = BigInt::from(2);
    let x = dois.pow(1000u32);
    let mut soma = 0;
    for y in x.to_string().chars() {
        soma += y.to_string().parse::<i32>().unwrap();
    }
    println!("{}", soma);
}
