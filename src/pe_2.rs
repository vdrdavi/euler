pub fn two() {
    let n = 4000000;
    let mut numeros: Vec<usize> = Vec::with_capacity(n);
    let mut x = 1;
    let mut i = 0;
    while x <= n {
        if x == 1 || x == 2 {
            numeros.push(x);
        } else if numeros[i] + numeros[i + 1] == x {
            numeros.push(x);
            i += 1
        }
        x += 1;
    }
    numeros.retain(|&x| x % 2 == 0);
    println!("{}", numeros.iter().sum::<usize>())
}