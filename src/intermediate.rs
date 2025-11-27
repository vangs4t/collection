
fn filter_bonus(gaji: Vec<i32>) -> Vec<i32> {
    gaji.iter()
    .filter(|&x|*x > 5000000)
    .map(|x|x /10 + x).collect()
}


#[test]
fn filter_gaji() {
    let gaji = vec![3_500_000, 7_200_000, 5_000_000, 12_000_000];
    println!("{:?}", filter_bonus(gaji));
}

#[test]
fn format_strings() {
    let names = vec!["Naufal", "Han", "Fei"];

    let result:Vec<String> = names.iter()
    .map(|x| format!("Halo, {}!", x)).collect();

    for x in result {
        println!("{}", x);
    }
}

fn total(harga: Vec<i32>) -> i32 { 
    harga.iter().fold(0, |acc, x| acc + x)
 }

#[test]
fn point_of_sales() {
    let harga = vec![12000, 18000, 5000, 3000];

    println!("{}", total(harga));
}

#[test]
fn sticy_finger() {
    let nama = vec!["Naufal", "Ayu", "Gilang"];
    let umur = vec![20, 19, 22];

    let result = nama.iter().zip(umur);

    for (a,b) in result {
        println!("{} berumur {}", a,b);
    }
}

#[test]
fn mergers() {
    let a = vec![1,2,3];
    let b = vec![4,5,6];
    let c = a.into_iter().chain(b);

    for x in c {
        println!("{}", x);
    }
}

