pub fn one() {
    let n = 1000;
    let mut numeros: Vec<usize> = Vec::with_capacity(n);
    for x in 3..n {
        if x % 3 == 0 || x % 5 == 0 {
            numeros.push(x);
        }
    }
    println!("{}", numeros.iter().sum::<usize>());
}