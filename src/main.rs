use std::collections::HashMap;
use std::io::{self, Write};

// Struct untuk menyimpan data santri
#[derive(Debug)]
struct Santri {
    nama: String,
    usia: u8,
    kelas: String,
}

fn main() {
    let mut database: HashMap<String, Santri> = HashMap::new();

    loop {
        println!("Menu:");
        println!("1. Tambah data santri");
        println!("2. Lihat data santri");
        println!("3. Edit data santri");
        println!("4. Hapus data santri");
        println!("5. Keluar");

        print!("Pilih menu: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Gagal membaca input");
        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Masukan angka yang valid!");
                continue;
            }
        };

        match choice {
            1 => tambah_data(&mut database),
            2 => lihat_data(&database),
            3 => edit_data(&mut database),
            4 => hapus_data(&mut database),
            5 => {
                println!("Keluar dari program.");
                break;
            }
            _ => println!("Pilihan tidak valid!"),
        }
    }
}

fn tambah_data(database: &mut HashMap<String, Santri>) {
    println!("Masukkan nama santri:");
    let mut nama = String::new();
    io::stdin().read_line(&mut nama).expect("Gagal membaca input");

    println!("Masukkan usia:");
    let mut usia = String::new();
    io::stdin().read_line(&mut usia).expect("Gagal membaca input");
    let usia: u8 = match usia.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Masukan angka yang valid!");
            return;
        }
    };

    println!("Masukkan kelas:");
    let mut kelas = String::new();
    io::stdin().read_line(&mut kelas).expect("Gagal membaca input");

    let santri = Santri {
        nama: nama.trim().to_string(),
        usia,
        kelas: kelas.trim().to_string(),
    };

    database.insert(nama.trim().to_string(), santri);
    println!("Data santri berhasil ditambahkan!");
}

fn lihat_data(database: &HashMap<String, Santri>) {
    println!("Data Santri:");
    for (nama, santri) in database {
        println!("Nama: {}", nama);
        println!("Usia: {}", santri.usia);
        println!("Kelas: {}", santri.kelas);
        println!("==============================");
    }
}

fn edit_data(database: &mut HashMap<String, Santri>) {
    println!("Masukkan nama santri yang akan diedit:");
    let mut nama = String::new();
    io::stdin().read_line(&mut nama).expect("Gagal membaca input");
    let nama = nama.trim().to_string();

    if let Some(santri) = database.get_mut(&nama) {
        println!("Masukkan usia baru:");
        let mut usia = String::new();
        io::stdin().read_line(&mut usia).expect("Gagal membaca input");
        let usia: u8 = match usia.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Masukan angka yang valid!");
                return;
            }
        };

        println!("Masukkan kelas baru:");
        let mut kelas = String::new();
        io::stdin().read_line(&mut kelas).expect("Gagal membaca input");

        santri.usia = usia;
        santri.kelas = kelas.trim().to_string();

        println!("Data santri berhasil diubah!");
    } else {
        println!("Santri dengan nama tersebut tidak ditemukan!");
    }
}

fn hapus_data(database: &mut HashMap<String, Santri>) {
    println!("Masukkan nama santri yang akan dihapus:");
    let mut nama = String::new();
    io::stdin().read_line(&mut nama).expect("Gagal membaca input");
    let nama = nama.trim().to_string();

    if let Some(_) = database.remove(&nama) {
        println!("Data santri berhasil dihapus!");
    } else {
        println!("Santri dengan nama tersebut tidak ditemukan!");
    }
}
