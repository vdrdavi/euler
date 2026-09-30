pub fn twelve() {
    let mut n = 1;
    let mut triangular_number: u64 = 0;
    let mut qtd_divisores: u64 = 0;
    while qtd_divisores <= 500 {
        triangular_number = 0;
        qtd_divisores = 0;
        for x in 1..=n {
            triangular_number += x;
        }
        for y in 1..=triangular_number.isqrt() {
            if triangular_number % y == 0 {
                qtd_divisores += 1;
            }
        }
        qtd_divisores *= 2;
        
        n += 1;
    }
    println!(
            "{}",
            triangular_number
        );
}
