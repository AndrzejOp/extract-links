fn funkcja(napis: &String){
    println!("{}", napis);
}

fn count_char(napis: String, szukany_znak: char) -> u32 {
    let mut out = 0;
    for i in napis.chars(){
        if i == szukany_znak{
            out += 1;
        }
    }

    out

}

fn zad3(imie: &String, nazwisko: &String) -> String {

    let mut out = String::new();
    let upper_imie = imie.to_uppercase();
    let pierwsza_litera = upper_imie.chars().nth(0).unwrap();
    out.push_str(pierwsza_litera.to_string().as_str());
    out.push_str(". ");
    let upper_nazwisko = nazwisko.to_uppercase();
    let pierwsza_nazwiska = upper_nazwisko.chars().nth(0).unwrap();
    out.push_str(pierwsza_nazwiska.to_string().as_str());
    let lower_nazwisko = nazwisko.to_lowercase();

    let mut i = 1;

    while i < lower_nazwisko.len() {
        let litera = lower_nazwisko.chars().nth(i).unwrap();
        out.push_str(litera.to_string().as_str());
        i += 1;
    }

    out
}

fn zad4(napis: &String) -> String {
    let iterator = napis.chars().step_by(2);
    let mut out = String::new();
    for i in iterator {

        out.push_str(i.to_string().as_str());
    }
    // let mut i = 0;
    //
    // while i < napis.len() {
    //    let litera = napis.chars().nth(i).unwrap();
    //     out.push_str(litera.to_string().as_str());
    //     i += 2;
    // }

    out
}

fn zad5(napis: &String) -> String {
    let out = napis.chars().rev().collect();

    out
}

fn szyfruj(napis: &String, klucz: usize) -> String{
    let mut out = String::new();
    let it = napis.chars();

    let mut offset: usize = 0;

    loop {
        if offset * klucz > napis.len() {
            break;
        }
        out += it.
            clone().
            skip(offset*klucz).
            take(klucz).
            collect::<String>().
            chars().
            rev().
            collect::<String>().
            as_str();
        offset+=1;
    }
    out
}

fn main() {
    let lepszy_napis: String = String::from("keoszy okoks");
    funkcja(&lepszy_napis);

    let napis: String = String::from("mleko banany igły lód");
    let count_o = count_char(napis, 'l');
    println!("{}", count_o);

    let imie  = String::from("janEk");
    let nazwisko = String::from("KOWQAL");
    let zlozenie = zad3(&imie,&nazwisko);

    println!("{}",zlozenie);

    let napis = String::from("Tojestnapisiwezcodrugalitere");
    let co_druga_litera = zad4(&napis);
    println!("{}",co_druga_litera);

    let napis = String::from("Rust");
    let rev_napis = zad5(&napis);
    println!("{}",rev_napis);

    let napis = String::from("Rust to fajny fakultet");
    let rev_napis = szyfruj(&napis,2);
    println!("{}",rev_napis);

}

