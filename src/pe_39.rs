pub fn thirty_nine() {
    let mut maior = 0;
    let mut contador;
    let mut x = 0;
    for p in 3..=1000 {
        contador = 0;
        for b in 1i32..p/2 {
            for c in b..p/2 {
                let a = p - b - c;
                if a.pow(2) == b.pow(2) + c.pow(2) {
                    contador += 1;
                }
            }
        }
        if contador > maior {
            x = p;
            maior = contador
        }
    }
    println!("{x}");
}
