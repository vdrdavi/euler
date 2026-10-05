pub fn thirty_six() {
    let mut soma = 0;
    for numero in 0..1000000 {
        let numero_str: String = numero.to_string();
        let numero_str_rev: String = numero_str.chars().rev().collect();
        let binario: String = gerar_binario(numero);
        let binario_rev: String = binario.chars().rev().collect();
        if binario == binario_rev && numero_str == numero_str_rev{
            soma+=numero;
        }
    }
    println!("{soma}");
}

fn gerar_binario(numero: i64) -> String {
    let mut resposta: String = String::from("");

    let mut x = numero;
    while x > 0 {
        let resto = x % 2;
        if resto == 1 {
            resposta = "1".to_string() + &resposta;
        } else {
            resposta = "0".to_string() + &resposta;
        }
        x /= 2;
    }

    resposta
}
