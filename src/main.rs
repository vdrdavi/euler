fn main() {
    //zero();
    //one();
    //two();
    //three();
    //four();
    //five();
    //six();
    //seven();
    //eight();
    //nine();
    ten();
}

fn ten() {
    let n:i64 = 2000000;
    let mut primos: Vec<i64> = vec![2];
    for x in (3i64..n).step_by(2) {
        println!("{x}");
        primos.push(x);
        for y in (3..x.isqrt()+1).step_by(2) {
            if x % y == 0 {
                primos.pop();
                break;
            }
        }
    }
    println!("soma: {}", primos.iter().sum::<i64>())
}

#[allow(dead_code)]
fn nine() {
    for a in 1i32..1000 {
        for b in 1i32..1000 {
            for c in 1i32..1000 {
                if a.pow(2) + b.pow(2) == c.pow(2) && a < b && b < c && a + b + c == 1000 {
                    println!("{}", a * b * c);
                    return;
                }
            }
        }
    }
}
#[allow(dead_code)]
fn eight() {
    let numerozao = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450".to_string();
    let mut somas: Vec<u64> = Vec::new();
    let mut produto: u64;
    let n = 13;

    for x in n..numerozao.len() {
        produto = 1;
        for c in &numerozao[x - n..x].chars().collect::<Vec<char>>() {
            produto *= c.to_string().parse::<u64>().unwrap();
        }
        somas.push(produto);
    }
    println!("{}", somas.iter().max().unwrap())
}
#[allow(dead_code)]
fn seven() {
    let mut contador = 0;
    let mut numero = 2;
    while contador < 10001 {
        if numero % 2 != 0 {
            contador += 1;
            for x in 2..numero {
                if numero % x == 0 {
                    contador -= 1;
                    break;
                }
            }
        } else if numero == 2 {
            contador += 1
        }
        numero += 1
    }
    println!("{}", numero - 1)
}
#[allow(dead_code)]
fn six() {
    let mut soma_dos_quadrados: i32 = 0;
    let mut quadrado_da_soma: i32 = 0;

    for x in 1i32..=100 {
        soma_dos_quadrados += x.pow(2);
        quadrado_da_soma += x;
    }

    println!("{}", quadrado_da_soma.pow(2) - soma_dos_quadrados);
}
#[allow(dead_code)]
fn five() {
    let mut resolvido: bool = false;
    let mut numero = 20;
    while resolvido == false {
        resolvido = true;
        for x in 1..=20 {
            if numero % x != 0 {
                resolvido = false;
                break;
            }
        }
        numero += 1;
    }
    println!("{}", numero - 1)
}
#[allow(dead_code)]
fn four() {
    let n = 999;
    for x in (900..n + 1).rev() {
        for y in (900..n + 1).rev() {
            let w = x * y;
            if w.to_string().chars().eq(w.to_string().chars().rev()) {
                println!("{}", w);
                return;
            }
        }
    }
}
#[allow(dead_code)]
fn three() {
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
