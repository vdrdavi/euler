use num_bigint::BigInt;
pub fn fifty_six() {
    let mut max_sum = 0;
    for a in 1u32..100 {
        for b in 1u32..100 {
            let soma = calcula_soma(BigInt::from(a).pow(b).to_string());
            if soma > max_sum {
                max_sum = soma;
            }
        }
    }
    println!("{}",max_sum);
}

fn calcula_soma(numero: String) -> i32 {
    let mut soma = 0;
    for digito in numero.chars() {
        soma += digito.to_string().parse::<i32>().unwrap();
    }
    soma
}
