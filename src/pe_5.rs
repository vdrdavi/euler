pub fn five() {
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