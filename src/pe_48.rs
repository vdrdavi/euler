use num_bigint::BigInt;
use num_traits::ToPrimitive;

pub fn forty_eight(){
    let mut soma:BigInt = BigInt::from(0);
    let mut numero:BigInt = BigInt::from(1);
    while numero <= BigInt::from(1000) {
        soma+= numero.pow(numero.to_u32().unwrap());
        numero+=1;
    }
    println!("{}",&soma.to_string()[soma.to_string().len()-10..]);
}