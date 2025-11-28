
fn hitung_pajak(gaji: Vec<u32>) -> Result<Vec<u32>, String>{
    match gaji.len() {
        0 => Err("Gaji karyawan tidak ditemukan".to_string()),
        _ => Ok(gaji.iter().map(|x| x - (x / 10) ).collect())
    }
}

fn parse_angka(data: Vec<&str>) -> Result<Vec<i32>, String>{
    let mut hasil = vec![];

    for x in data {
        match x.parse::<i32>() {
            Ok(angka) => hasil.push(angka),
            Err(_) => return Err(format!("Bukan angka {}",x))
        }
    }
    Ok(hasil)
}


#[test]
fn gitung() {
    let gaji:Vec<u32> = vec![3_500_000, 7_200_000, 5_000_000, 12_000_000];
    let angka= vec!["8","23","44"];
    println!("{:?}", hitung_pajak(gaji));
    println!("{:?}", parse_angka(angka));
}

fn database_connection(host :Option<String>) -> Result<String, String> {
    match host {
        Some(msg) => Ok("connected to database".to_string()),
        None => Err("databse not found".to_string())
    }
}

fn emails(host :Option<String>) -> Result<String, String> {
    match host {
        Some(msg) => Ok("Login..".to_string()),
        None => panic!("TErhack")
    }
}

#[test]
fn connected() {
    let host = None;
    let email = None;
    println!("{:?}", database_connection(host));
    println!("{:?}", emails(email));
}