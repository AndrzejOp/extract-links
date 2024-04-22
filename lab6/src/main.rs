//Napisz funkcję o nagłówku fn dodaj_pisemnie(a: ..., b: ...) -> ...
//która doda dwie (zakładamy, że poprawne) liczby całkowite podane w argumentach
//jako napisy w zapisie dziesiętnym — i zwróci wynik również jako napis.
//Uwaga: dodawanie należy przeprowadzić pisemnie, bowiem liczby mogą być dowolnie duże. Przykłady:
/*dodaj_pismnie("1", "3") == "4"
dodaj_pismnie("1", "3") == "4"
dodaj_pismnie("8", "3") == "11"
dodaj_pismnie("10", "23") == "33"
dodaj_pismnie("1", "0") == "1"
dodaj_pismnie("11", "00") == "11"
dodaj_pismnie("131", "9900") == "10031"
dodaj_pismnie("998", "7") == "1005"
dodaj_pismnie("24872947", "294729478") == "319602425"
dodaj_pismnie("5924729874298749827418582", "6782893629472094209740298") == "12707623503770844037158880"*/


fn number_of_digits(number: usize) -> usize {
    let mut num = number;
    let mut result = 0;
    while num != 0 {
        result += 1;
        num /= 10;
    }
    result
}

fn how_much_left(number: usize) -> usize {
    number % 10
}

fn dodaj_pisemnie(a: &str, b: &str) -> String {
    let mut temp_a = a.chars().rev().map(|c| c.to_digit(10).unwrap_or(0));
    let mut temp_b = b.chars().rev().map(|c| c.to_digit(10).unwrap_or(0));
    let mut result = Vec::new();
    let mut rest = 0;

    loop {
        if let (Some(digit_a), Some(digit_b)) = (temp_a.next(), temp_b.next()) {
            let sum = digit_a + digit_b + rest;
            result.push((sum % 10).to_string());
            rest = sum / 10;
        } else if let Some(digit_a) = temp_a.next() {
            let sum = digit_a + rest;
            result.push((sum % 10).to_string());
            rest = sum / 10;
        } else if let Some(digit_b) = temp_b.next() {
            let sum = digit_b + rest;
            result.push((sum % 10).to_string());
            rest = sum / 10;
        } else {
            break;
        }
    }

    if rest > 0 {
        result.push(rest.to_string());
    }

    result.into_iter().rev().collect()
}

fn main() {
    println!("{}", dodaj_pisemnie("5924729874298749827418582", "6782893629472094209740298"));
}

