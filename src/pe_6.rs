pub fn six() {
    let mut soma_dos_quadrados: i32 = 0;
    let mut quadrado_da_soma: i32 = 0;

    for x in 1i32..=100 {
        soma_dos_quadrados += x.pow(2);
        quadrado_da_soma += x;
    }

    println!("{}", quadrado_da_soma.pow(2) - soma_dos_quadrados);
}