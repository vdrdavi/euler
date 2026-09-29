pub fn zero() {
    let n = 730000;
    let mut numeros: Vec<i64> = Vec::with_capacity(n);
    let mut x = 2;

    while numeros.len() < n - 1 {
        numeros.push(x * x);
        x += 1;
    }

    numeros.retain(|&x| x % 2 != 0);

    println!("{}", numeros.iter().sum::<i64>() + 1);
}