mod intermediate;
mod exception;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_formatter_dasar() {
    let a = vec![10,20,30];
    let mut i = 0;
    for (i, value) in a.iter().enumerate() {
    println!("Angka ke-{i} : {value}");
}

}

#[test]
fn test_closure_fundamentals() {
    let vektor = vec![1,2,3,4];
    let kali_dua = |x: &i32| -> i32{
        2 * x
    };
    let result = vektor.iter().map(kali_dua);

    let vektir: Vec<_> = vektor.iter().map(|x| x * 2).collect();
    println!("{:?}", vektir);

}

#[test]
fn filter_odd_number() {
    let vektor = vec![1,5,7,3,2];
    let otpuy: Vec<_> = vektor
    .iter()
    .filter(|x| *x % 2 != 0)
    .map(|x| x * 3)
    .collect();

    println!("{:?}", otpuy);
}   

#[test]
fn format_nama() {
    let names = vec!["Xin".to_string(), "Fei".to_string(), "Han".to_string()];
    let format_nama = |x: &String| -> String{
        format!("Halo {x}!, selamat berperang")
    };

    let iters = names.iter().map(format_nama);

    for name in names.iter().map(|x| format!("Halo {x}! Selamat berperang")) {
    println!("{}", name);
}

}

#[test]
fn sequence_without_vector() {
    (1..=10)
    .filter(|x| x % 2 == 0)
    .for_each(|x| println!("Genap {x}"));

}

#[test]
fn closure_capture() {
    let  num = 5;
    let mut cektor = vec![];
    let mut capture = || for x in 1..4 {
        cektor.push(x * num);
    };
    capture();
    println!("{:?}", cektor);
}


