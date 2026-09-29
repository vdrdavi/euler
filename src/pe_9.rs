pub fn nine() {
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