pub fn ten() {
    let n: i64 = 2000000;
    let mut primos: Vec<i64> = vec![2];
    for x in (3i64..n).step_by(2) {
        println!("{x}");
        primos.push(x);
        for y in (3..x.isqrt() + 1).step_by(2) {
            if x % y == 0 {
                primos.pop();
                break;
            }
        }
    }
    println!("{}", primos.iter().sum::<i64>())
}