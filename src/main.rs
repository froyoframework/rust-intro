#[derive(Debug)]
struct Pemain {
    nama: String,
    umur: i32,
    gol: i32,
}

fn main() {
    println!("Hello, world!");
    let angka = 9;
    let angka_saya = calc(angka);
    println!("{}", angka_saya);
    let messi = tambah_pemain("Messi", 29, 500);
    println!("{:?}", messi);
    let pemain_keren = tambah_para_pemain();
    println!("{:?}", pemain_keren);

    // contoh ownership dan borrowing (reference)
    // pemain_bola adalah reference ke pemain_keren, sehingga pemain_keren tetap bisa diprint
    let pemain_bola = &pemain_keren;
    println!("pemain pertama adalah: {}", pemain_keren[0].nama);

    // b adalah mutable reference ke a
    // *b artinya mengakses isi dari reference, dalam hal ini, isi dari a
    // karena mutable reference, maka isi a bisa diganti
    let mut a = 90;
    {
        let b = &mut a;     // a dipinjam di sini
        *b += 9;            // isi a diakses di sini
    }                       // peminjaman a berakhir di sini
    println!("{}", a);
}

fn calc(x: i32) -> i32 {
    let y;
    match x {
        1...40 => y = 34,
        _ => y = 2,
    }
    y
}

fn tambah_pemain(nama_: &str, umur_: i32, gol_: i32) -> Pemain {
    let pemain_saya = Pemain {
        nama: nama_.to_string(),
        umur: umur_,
        gol: gol_,
    };
    pemain_saya
}

fn tambah_para_pemain() -> Vec<Pemain> {
    let ronaldo = tambah_pemain("Ronaldo", 31, 510);
    let bacca = tambah_pemain("Bacca", 31, 235);
    let payet = tambah_pemain("Payet", 28, 150);

    let mut pemain_favorit = Vec::new();
    pemain_favorit.push(ronaldo);
    pemain_favorit.push(bacca);
    pemain_favorit.push(payet);

    pemain_favorit
}
