fn main() {
    //zero();
    //one();
    //two();
    three();
}

fn three() {
    let n: i64 = 600851475143;
    let mut vet = Vec::new();
    for x in (3..n.isqrt()+1).step_by(2) {
        if n % x == 0 {
            println!("verificando...");
            vet.push(x);
            for y in (3..x.isqrt()+1).step_by(2) {
                if x % y == 0 {
                    vet.pop();
                    println!("{x} não é primo, é divisivel por {y}");
                    break;
                }
            }
        }
    }
    println!("resposta: {:?}", vet);
}
#[allow(dead_code)]
fn two() {
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
#[allow(dead_code)]
fn one() {
    let n = 1000;
    let mut numeros: Vec<usize> = Vec::with_capacity(n);
    for x in 3..n {
        if x % 3 == 0 || x % 5 == 0 {
            numeros.push(x);
        }
    }
    println!("{}", numeros.iter().sum::<usize>());
}
#[allow(dead_code)]
fn zero() {
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
