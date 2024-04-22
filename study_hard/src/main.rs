

// Napisz funkcję trójargumentową, która poprzestawia wartości swoich argumentów (dla ustalenia uwagi: typu i32) tak, by były uporządkowane niemalejąco.
// Stwórz generator liczb pseudolosowych, którego ziarno przechowywane będzie na zewnątrz i podawane w pierwszym parametrze, a w parametrze drugim i trzecim podawany będzie przedział losowanych liczb.
// fn rand(seed: &mut ..., min_rand: ..., max_rand: ...) -> ...
// Możesz wybrać któryś z: https://en.wikipedia.org/wiki/Linear_congruential_generator
// Napisz funkcję
// swap_arr(arr: ..., i: usize, j: usize)
// która zamieni wartości dwóch podanych elementów pewnej tablicy.
// Stwórz funkcję
// rand_perm(arr: ..., seed: ...)
// permutującą w miejscu w sposób losowy wartości tablicy przekazanej w argumencie.

mod kolos;

use std::time::{SystemTime, UNIX_EPOCH};

fn biggest_of_3 (a: &mut i32, b: &mut i32, c: &mut i32){
    let mut temp = 0;
    if *a <= *b {
        temp = *b;
        *b = *a;
        *a = temp;
    }
    if *b <= *c {
        temp = *b;
        *b = *c;
        *c = temp;
    }
    if *a <= *b {
        temp = *b;
        *b = *a;
        *a = temp;
    }
}



fn swap (a: &mut u32, b: &mut u32) {
    let temp = *a;
    *a = *b;
    *b = temp;

}



fn time_differ(time1: (i32,i32,i32), time2: (i32,i32,i32)) -> (i32,i32,i32) {
    let mut hh = (time1.0 - time2.0).abs();
    let mut mm = (time1.1 - time2.1).abs();
    let mut ss = (time1.2 - time2.2).abs();
    if time1.0 > 24 || time2.0 > 24 || time1.0 < 0 || time2.0 < 0
        || time1.1 < 0 || time2.1 < 0 || time1.1 > 60 || time2.1 > 60
        || time1.2 < 0 || time2.2 < 0 || time1.2 > 60 || time2.2 > 60 {
        println!("Nieprawdiłowe dane !!!");
        return (00,00,00);
    }



    (hh,mm,ss)

}
fn pierwiastki(a: f32, b: f32, c: f32) -> u8{
    let d = ((b*b) - 4.0*a*c).sqrt();
    if d > 0.0 {
        return 2;
    }
    else if d == 0.0 {
        return 1;
    }
    0
}


fn factorial(liczba: i32) -> i32 {
    let mut out = liczba;
    let mut i = liczba;
    while i > 1 {
        i-=1;
        out *= i;
    }
    out
}

fn perfect_number(number: i32) -> bool {
    let temp = number;
    let mut i = 1;
    let mut result = 0;
    while i <= (temp/2) {
        if number % i == 0 {
            result += i;
        }
        i += 1;
    }
    number == result
}

fn factorization(number: i32) {
    //Napisz funkcję, która wyświetli rozkład podanej liczby na czynniki pierwsze.
    let mut temp = number;
    let mut i = 2;

    while temp > 1 {
        if temp % i == 0{
            temp /= i;
            println!("{}", i);
            i = 2;
            continue
        }
        i +=1 ;
    }
}

fn swap_tab(tab:&mut [u32], r1:usize, r2: usize) {
    let mut idx1 = r1;
    let mut idx2 = r2;
    let temp = tab[idx1];
    tab[idx1] = tab[idx2];
    tab[idx2] = temp;

}
fn rand(mut seed: u128, min: u128, max: u128) -> u128 {
    const A: u128 = 6364136223846793005;
    const C: u128 = 1442695040888963407;
    const M: u128 = ( 1 << 63 ) - 1;
    let temp = max - min + 1;
    seed *= (A * C) % M;
    seed % temp + min
}
fn rand_perm(mut arr: &mut [u32], seed: u128) {
    let max = arr.len() as u128;
    for i in 0..=10 {
        swap_tab(&mut arr, rand(seed, 0, max - 1) as usize, rand(seed, 0, max - 1) as usize);
    }
}

/*fn main() {
    // let result = time_differ((-12,34,52), (8,34,4));
    // println!("{:02}:{:02}:{:02}",result.0, result.1, result.2);
    //println!("{:?}", factorial(12));
    // if perfect_number(412) {
    //     println!("Tak");
    // }
    // else {
    //     println!("Nie");
    // }

   // factorization(54535);
   //  let mut a = 12;
   //  let mut b = 1;
   //  println!("a: {}, b: {}", a, b);
   //  swap(&mut a, &mut b);
   //  println!("a: {}, b: {}", a, b);

    // let mut a = 5;
    // let mut b = 4;
    // let mut c = 6;
    // println!("a: {}, b: {}, c: {}", a, b, c);
    // biggest_of_3(&mut a, &mut b, &mut c);
    // println!("a: {}, b: {}, c: {}", a, b, c);


    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("WRONG")
        .as_millis();
    // println!("{}", rand(seed,0,67));
    let mut arr = [1,2,3,4,5,6];
    rand_perm(&mut arr,seed);
    for i in arr {
        println!("{}", i);
    }
}
*/
fn wyswietl(t: &[i32]) {
    println!("{:?}", t);
}

fn wyswietl_jeden(t: &[i32], i: usize) {
    let x = t.get(i);
    println!("{:?}, {:?}, {:?}", x, x.is_none(), x.unwrap_or(&999));
    println!("{:?}", x.unwrap());
}

fn main() {
    // let mut tablica = [10, 20, 40, 100];
    // wyswietl(&tablica);
    // println!("{}", tablica[1]);
    // tablica[2] = 567;
    // println!("{}", tablica.len());
    // wyswietl_jeden(&tablica, 1);
    // wyswietl_jeden(&tablica, 100);  // błąd -- w czasie wykonania
    println!("{}", pierwiastki(12.0,2.0,1.0));
}

