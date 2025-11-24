fn main() {
    println!("Hello, world!");
}

#[test]
fn test_formatter_dasar() {
    let a = vec![10,20,30];
    let mut i = 0;
    for value in a {
        println!("Angka ke-{} : {}",i, value );
        i+=1;
    }
}

#[test]
fn test_closure_fundamentals() {
    let vektor = vec![1,2,3,4];
    let kali_dua = |x: i32| -> i32{
        x * 2
    };
    
    // let mut result = vektor.iter().map(kali_dua());
}