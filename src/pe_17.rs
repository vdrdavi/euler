pub fn seventeen() {
    let um_a_nove = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let dez_a_dezenove = vec![
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let prefixos = [
        "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let mut soma = 0;

    for n in [um_a_nove.clone(), dez_a_dezenove.clone()].concat() {
        soma += n.len();
    }
    for prefixo in prefixos {
        soma += prefixo.len();
        for sulfixo in &um_a_nove {
            soma += (prefixo.to_string() + sulfixo).len();
        }
    }
    for prefixo in um_a_nove.clone() {
            soma += (prefixo.to_string() + "hundred").len();
        for sulfixo in [um_a_nove.clone(), dez_a_dezenove.clone()].concat() {
            soma += (prefixo.to_string() + "hundredand" + sulfixo).len();
        }
        for infixo in prefixos {
            soma += (prefixo.to_string() + "hundredand" + infixo).len();
            for sulfixo in &um_a_nove {
                soma += (prefixo.to_string() + "hundredand" + infixo + sulfixo).len();
            }
        }
    }
    soma += "onethousand".len();
    println!("{soma}");
}
