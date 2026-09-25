use core::num;

fn main() {
    //zero();
    //one();
    //two();
}

#[allow(dead_code)]
fn two() {
    let n = 4000000;    
    let mut numeros: Vec<usize> = Vec::with_capacity(n);
    let mut x = 1;
    let mut i = 0;
    while x <= n{
        if x == 1 || x == 2{
            numeros.push(x);
        }
        else if numeros[i] + numeros[i+1] == x{
            numeros.push(x);
            i+=1
        }
        x+=1;
    }
    numeros.retain(|&x| x % 2 == 0);
    println!("{}",numeros.iter().sum::<usize>())
}
#[allow(dead_code)]
fn one() {
    let n = 1000;
    let mut r: Vec<usize> = Vec::with_capacity(n);
    for x in 3..n {
        if x % 3 == 0 || x % 5 == 0 {
            r.push(x);
        }
    }
    println!("{}", r.iter().sum::<usize>());
}
#[allow(dead_code)]
fn zero() {
    let n = 730000;
    let mut r: Vec<i64> = Vec::with_capacity(n);
    let mut x = 2;

    while r.len() < n - 1 {
        r.push(x * x);
        x += 1;
    }

    r.retain(|&x| x % 2 != 0);

    println!("{}", r.iter().sum::<i64>() + 1);
}
