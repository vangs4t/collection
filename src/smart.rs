/*Jadi smart pointer digunakan untuk menyimpan data ke dalam heap 
sekaligus menyimpan alamat atau pointernya di stack */
#[test]
fn feature() {
    let result: Box<i32> = Box::new(5);

    // disini tipe data i32 otomatis disimpan di dalam heap
    let hail = result.clone();

    println!("{}",result);
}