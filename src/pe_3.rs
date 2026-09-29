pub fn three() {
    let n: i64 = 600851475143;
    let mut numeros = Vec::new();
    for x in (3..n.isqrt() + 1).step_by(2) {
        if n % x == 0 {
            println!("verificando...");
            numeros.push(x);
            for y in (3..x.isqrt() + 1).step_by(2) {
                if x % y == 0 {
                    numeros.pop();
                    println!("{x} não é primo, é divisivel por {y}");
                    break;
                }
            }
        }
    }
    println!("resposta:{}", numeros.iter().max().unwrap());
}