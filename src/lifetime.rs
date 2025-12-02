use core::str;
use std::{fmt::Debug, ops::{AddAssign, Deref}, rc::{self, Rc}, result};
use std::cell::RefCell;

/*
Lifetime digunakan sebagai penanda untuk value reference */
fn reference<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
#[test]
fn lifetims() {
    let x = "kanjt";
    let y = "kintikls";

    reference(x, y);
}
#[derive(PartialEq, PartialOrd)]
struct Capture<'a> {
    name: &'a str,
    camera: &'a str
}

impl<'a> Capture<'a> {
    fn view_merek(&self, names: &'a str) -> &'a str{
        names
    }
}

impl<'a> Debug for Capture<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Capture").field("name", &self.name).field("camera", &self.camera).finish()
    }
    // fn view_merek(&self, names: &'a str) -> &'a str{
    //     names
    // }
}

#[test]
fn struct_lifetime() {
    let result = Capture{
        name: "Naufal",
        camera: "Nikon"
    };

    println!("{:?}", result.view_merek(result.camera));
}

#[derive(Debug, Clone)]
struct User{
    name: String,
    umur: u32,
}

#[test]
fn user_featr() {
    let result = User{
        name: "Naufal".to_string(),
        umur: 23
    };

    let hasil = result.clone();

    println!("{:?}", result);
}

#[deprecated]
fn old_function() {
    println!("Ini fungsi lama");
}

#[test]
fn deprecated_test() {
    old_function();
    let data = vec!["12","44","54"];
    println!("{:?}",parse_angka(data));
}

fn parse_angka(data: Vec<&str>) -> Result<Vec<i32>, String> {
    let mut hasil = Vec::new();

    for item in data {
        match item.parse::<i32>() {
            Ok(angka) => hasil.push(angka),
            Err(_) => return Err(format!("'{}' bukan angka valid", item)),
        }
    }

    Ok(hasil)
}

enum LoginError{
    UsernameKosong,
    PasswordKosong,
}

fn login(user: &str, pass: &str) -> Result<String, LoginError>{
    if user.is_empty() {
        return Err(LoginError::UsernameKosong);
    } else if pass.is_empty() {
        return Err(LoginError::PasswordKosong);
    };
    Ok("Selamat datang".to_string())

}

#[derive(Debug)]
enum List{
    Node(i32, Box<List>),
    End,
}

impl List {
    fn chains(){
        let hasil = List::Node(1, Box::new(List::Node(2, Box::new(List::Node(3, Box::new(List::End))))));
        println!("{:?}", hasil);
    }
}

#[test]
fn rantai() {
    List::chains();
}


/* Multiple ownership menggunakan struct file
serta menggunakan smartpointer Rc */
struct File{
    name: String,
    number: u32,
}

#[test]
fn count_files() {
    //implementasikan struct
    let one = File{
        name : "count awal 1".to_string(),
        number : 2
    };
    let hasil = Rc::new(one); // masukan ke dalam smartpointer
    println!("Hsil = {}", Rc::strong_count(&hasil)); // strong_count digunakan untuk melihat jumlah ownership
    let hasil2 = Rc::clone(&hasil); // cloning ownership 
    println!("Hsil = {}", Rc::strong_count(&hasil2)); // tampilkan jumlah ownership

    {
        let hasil3 = Rc::clone(&hasil);

        println!("Hsil = {}", Rc::strong_count(&hasil3));
        // owner juga akan otomatis dihapus saat keluar scope
    }

    println!("Hsil = {}", Rc::strong_count(&hasil2));


}

/*Interior mutability bisa digunakan untuk mengubah data di dalam 
variable immutable */
#[derive(Debug)]
struct Counter{
    nilai :u32
}

#[test]
fn count_refcell() {
    let b = Counter{
        nilai :10
    }; // variable b immutable

    let a = RefCell::new(b); // penggunaan refcell untuk mengubah data dalam variable

    println!("{:?}", a); // cek terlebih dahulu datanya
    
    let mut result = a.borrow_mut(); 
    result.nilai +=10; // pengubahan data dalam struct

    println!("{:?}", result); // hasil
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x > y {
        x
    } else {
        y
    }
}

struct Pesan<'a> {
    teks: &'a str,
}

fn buat_pesan<'a>(input: &'a String) -> Pesan<'a> {
    let hasil = Pesan{
        teks: &input
    };
    hasil
}


trait Task {
    fn run(&self) -> ();
}

struct PrintTask{
    msg: String,
}

struct AddTask{
    a: i32,
    b: i32
}

impl Task for PrintTask{
    fn run(&self) {
        println!("{}", self.msg)
    }
}

// impl Task for AddTask {
//     fn run(&self) -> Vec<Rc<RefCell<dyn Task>>>{
//         vec![]
//     }
// }

/*Dereferences yaitu kebalikan dari reference, dereference berfungsi untuk mengambil value
induk */

struct Buah<T>{
    nama: T
}


impl<T> Deref for Buah<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.nama
    }
}

struct Database{
    host: String,
}

impl Drop for Database {
    fn drop(&mut self) {
        println!("Pemutusan database {}", self.host);
    }
}

#[test]
fn sayuran() {
    let buah = Buah{
        nama: "pisang".to_string()
    };

    // bisa memanggil tanpa penyebutan nama field dengan dereferences
    // gunakan symbol *
    println!("Nama buah: {}", *buah);
}

/*Cleanup berguna untuk memberitahu data apa saja sebelum di drop 
atau di hapus dari memori saat selesai dicompile */

#[test]
fn test_cleanup() {
    
    let result = Database{
        host: String::from("Connect to database 1")
    };

    {
        let result1 = Database{ host: String::from("Connect to database 2")};
        // jika scope berahir maka trait drop akan dipanggil
    }

    println!("Menyambungkan ");
    // drop akan dipanggil sebelum selesai ngompile
}

/*Macro bisa membuat program sendiri saat dipanggil
dan cara pengimplementasianya juga sedikit kompleks yaitu dengan pattern */

macro_rules! hi {
    () => { // macro dengan parameter kosong
        println!("halo")
    };

    /* macro dengan parameter, gunakan tanda dolar untuk penaamaan parameter
    jangan lupa tambah kata kunci expr jika mengembalikan nilai*/
    ($name: expr) => {
        println!("Halo {}", $name); // selalu gunakan tanda dolar
    }
}

#[test]
fn test_macro() {
    hi!();
    hi!("Kanjut");
}