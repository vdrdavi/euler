pub fn forty_one() {
    for n in (12..=7654321).rev().step_by(2) {
        if is_pandigital(n, n.to_string().len()) && is_prime(n) {
            println!("{n}");
            break;
        }
    }
}
fn is_pandigital(number: u64, n: usize) -> bool {
    let numero: String = number.to_string();
    for x in 1..=9 {
        let c: char = char::from_digit(x, 10).unwrap();
        if (x > n.try_into().unwrap() && numero.contains(c))
            || (x <= n.try_into().unwrap() && !numero.contains(c))
        {
            return false;
        }
    }
    true
}
fn is_prime(number: u64) -> bool {
    if number == 2 {
        return true;
    }

    for n in (3..=number.isqrt()).step_by(2) {
        if number % n == 0 {
            return false;
        }
    }
    true
}
