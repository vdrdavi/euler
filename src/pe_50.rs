pub fn fifty() {
    let n: i64 = 1000000;
    let mut primos: Vec<i64> = vec![2];
    let mut maior_sequencia = 0;
    let mut primo = 0;
    let mut soma: i64;
    lista_primos(n, &mut primos);

    for i in 0..primos.len() - 1 {
        for j in i + 1..primos.len() {
            soma = primos[i..j].iter().sum::<i64>();
            let tamanho = j - i;
            if soma > n {
                break;
            }
            if soma <= n && is_prime(soma) {
                if tamanho > maior_sequencia {
                    maior_sequencia = tamanho;
                    primo = soma;
                }
            }
        }
    }
    println!("{primo}");
}

fn is_prime(n: i64) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    for x in (3..=n.isqrt()).step_by(2) {
        if n % x == 0 {
            return false;
        }
    }
    true
}

fn lista_primos(n: i64, primos: &mut Vec<i64>) {
    for x in (3..n).step_by(2) {
        primos.push(x);
        for y in (3..=x.isqrt()).step_by(2) {
            if x % y == 0 {
                primos.pop();
                break;
            }
        }
    }
}
