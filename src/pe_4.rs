pub fn four() {
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