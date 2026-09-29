pub fn seven() {
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