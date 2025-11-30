use std::fmt::Debug;

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
