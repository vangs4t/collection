use core::num;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/*Dasar2 aliran / control flow */
#[test]
fn fundamental() { 
    // variable and mutability
    /*Pembuatan variable di rust pada dasarnya variable tersebut menjadi 
    immutable, namun jika ingin membuat variable menjadi mutable maka harus ditambahkan
    kata kunci mut */
    let mut counter: u8 = 0; // mut
    counter += 5; 
    println!("counter {}", counter);
    
    // variable shadowing
    /* Jika kita mengimplementasikan nama variable yang sama berturut
    turut di dalam satu scope otomatis index atau isinya akan berubah seiring terahir
    kali data tersebut dirubah */
    let x = 5;
    println!("x = {}",x);
    let x = x +1;
    println!("x = {}", x);
    let x = "enam";
    println!("{x}");

    // condition
    /*di rust juga ada konsep if else dan cara penggunaan nya cukup sama
    karena konsep ini menjadi dasar dari semua pemograman */
    let num = 100;
    if num >= 100{ // tambahkan kondisi
        println!("Nomor besar"); // output jika kondisi terpenuhi
    } else {
        println!("Nomor kecil"); // output jika kondisi terpenuhi
    }

    // while loop
    /*while loop atau ketika loop 
    merupakan looping yang bisa ditambahkan kondisi  untuk menghentikan perulangan*/
    let mut a = 10;
    while a > 0 { // tambahkan kondisi setelah kata kunci while 
        a -= 1;
        println!("a = {a}");
    }

    // loop
    /*sedangkan loop biasa kita juga bisa menghentikan perulangan dengan kata kunci break */
    let mut nomor = 0;
    loop {
        println!("{nomor}");
        nomor += 1;
        if nomor > 5{
            break;
        }
    }
}

// function and data structure and enum

// function with return
 
fn hitung_luas(panjang: i32, lebar: i32) -> i32{
    panjang * lebar
}

// tuple return
fn get_data(nama: String, umur: u8, aktif: bool) -> (String, u8, bool) {
    let tuples = (nama, umur, aktif);
    tuples
}

#[test]
fn deconstructor() {
    let hasil = get_data("naufal".to_string(), 30, true);

    println!("Nama: {}", hasil.0);
    println!("umur: {}", hasil.1);
    println!("aktif: {}", hasil.2);
}

// simple struct

struct Buku{
    judul : String,
    halaman: u32
}

impl Buku {
    fn description(&self) -> String{
        format!("Buku ini berjudul {} dan berjumlah {} halaman", self.judul, self.halaman)
    }
}
#[test]
fn library() {
    let hasil = Buku{
        judul: "Cibalak".to_string(),
        halaman: 18
    };

    println!("Judul buku : {}", hasil.judul);
    println!("Halaman: {}", hasil.halaman);
    print!("Deskripsi buku {}", hasil.description())
}

// enumiration

enum Lampu {
    Merah,
    Kuning,
    Hijau
}

impl Lampu {
    fn fungsi(&self) {
        match self {
            Lampu::Merah => println!("Berhenti"),
            Lampu::Kuning => println!("Hati hati"),
            Lampu::Hijau => println!("Jalan")
        }
    }
}

#[test]
fn rambu() {
    let hasil = Lampu::Merah;

    hasil.fungsi();
}

/*OwnerShip borrowing and slices */
#[test]
fn owner() {
    let a = String::from("Halo");
    let b = &a; // reference
    let c = a.clone(); // 
    let mut hasil = "Kanjut terbang".to_string();
    let mut text = &mut hasil;
    let reference = "beberapa hari";
    let new_string = "Aku ";
    let new_str = "Anak ";
    println!("{}",literly(new_string, new_str));
    /*
    Takkan bisa di compile karena ownership a sudah dipindahkan ke 
    b, akan ada pengecualian jikalau kita menggunakan reference
    atau clone variable a */
    println!("{a}");
    get_len(b); // b bisa digunakan karena memang reference
    println!("{}",add_assign(text));
    println!("{}",potongan(reference));
}

// parameter borrowing
fn get_len(input :&String){
    println!("{input}");
}

fn get_leni(input :String){
    println!("{input}");
}
// mutable borrowing
fn add_assign(input: &mut String) -> &mut String{
    input.push('!');
    input
}

/*Sliceing */

fn potongan(input: &str) -> &str {
    let result: Vec<&str> = input.split(" ").collect(); 
    result[0]
}

struct User{
    nama: String
}

#[test]
fn firut() {
    let hasil = User{
        nama: "Kanjut".to_string()
    };
    // ownership hasil pindah karena gapake reference
    get_len(&hasil.nama);
    println!("{}", hasil.nama);
}

//Collections
#[test]
fn vektor() {
    let x = vec![10,20,30];
    x.iter().for_each(|a | println!("hasil = {}",a));
    // println!("{:?}",x.get(3)); // untuk method get hanya mengeluarkan option none
    // println!("{:?}", x[3]); // panic index out of bound, compiler langsung berhenti
}

fn literly(a: &str, b: &str) -> String{
    let mut hasil:String = String::from("");
    hasil.push_str(a);
    hasil.push_str(b);

    hasil
}

#[test]
fn maps() {
    let mut hasil:HashMap<String, u32> = HashMap::new();
    hasil.insert("Jajang".to_string(), 23);
    hasil.insert("Wiryo".to_string(), 114);
    hasil.insert("Santoso".to_string(), 3);

    println!("Pemain dengan skor {:? }, Bernama Jajang", hasil.get("Jajang"));
    hasil.iter().for_each(|x| println!("Nama pemain {}, Skor {}",x.0, x.1));
} 

/*
Multiple ownership and smart pointer */

#[test]
fn feature() {
    let hasil = Box::new(10);
    // box merupakan pointer yang unique karna bisa membuat data yang tersimpan di stak
    // berpindah ke dalam heap, sedangkan yang disimpan dalam stak hanya pointernya saja
    // yang otomatis bisa memiliki ownershipnya sendiri
    println!("{}", hasil);

    let a = Rc::new("Hasil".to_string());
    println!("{}",Rc::strong_count(&a));
    let b = a.clone();
    println!("{}",Rc::strong_count(&a));

    {
        let c = a.clone();
        println!("{}",Rc::strong_count(&a)); // jumlah reference akan berkurang karena scope nya 
        // sudah habis
    }
    println!("{}",Rc::strong_count(&a));

    let number: Rc<RefCell<i32>> = Rc::new(14.into());
    *number.borrow_mut() += 10;

    println!("{}", *number.borrow_mut());
}