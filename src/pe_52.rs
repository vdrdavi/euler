use std::collections::HashSet;

pub fn fifty_two(){
    let mut n = 1;
    while !(mesmos_digitos(n, n*2) && mesmos_digitos(n, n*3) && mesmos_digitos(n, n*4) && mesmos_digitos(n, n*5) && mesmos_digitos(n, n*6)){
        n+=1;
    }
    println!("{n}");
}

fn mesmos_digitos(a:u64, b:u64) -> bool {
    if a == b{
        return true;
    }
    let a = a.to_string();
    let b = b.to_string();
    let mut numeros: HashSet<u64> = HashSet::new();
    if a.len() != b.len(){
        return false;
    }
    for x in a.chars(){
        numeros.insert(x.into());
    }
    for y in b.chars(){
        if !numeros.contains(&u64::from(y)){
            return false;
        }
        numeros.remove(&u64::from(y));
    }
    numeros.is_empty()
}