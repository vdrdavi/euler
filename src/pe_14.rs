pub fn fourteen() {
    let mut n: i64;
    let mut resposta = (1, 1);
    let mut chain: Vec<i64> = Vec::new();
    for x in 1..1000000 {
        n = x;
        chain.clear();
        while n != 1 {
            chain.push(n);
            if n % 2 == 0 {
                n = n / 2;
            } else {
                n = 3 * n + 1;
            }
        }
        chain.push(1);
        if chain.len() > resposta.1 {
            resposta = (chain[0], chain.len());
        }
    }
    println!("{}", resposta.0);
}
