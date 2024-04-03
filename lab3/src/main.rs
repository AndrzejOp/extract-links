
fn print_ascii(){
    for i in 33..=126{
        print!("{} ",(i as u8) as char);
        if i%31 == 0{
            println!();
        }
    }
}

fn collatz(mut n: u64, mut result: u64) -> u64{
    /*Napisz funkcję, która dla danego całkowitego dodatniego n zwraca numer iteracji,
 w której osiągamy jedynkę w problemie Collatza (np. dla n=12 wynikiem jest 9).
 */
    if n == 1 {
        return result;
    }
    else if n%2 == 0 {
        n /= 2;
    }
    else {
        n = 3*n + 1;
    }
    result += 1;
    collatz(n,result)

}
fn armstrong(number: i32) -> bool{
    let mut sum = 0;
    let mut n = 0;
    let mut temp = number;
    while temp > 0 {
        temp /= 10;
        n += 1;
    }
//    println!("{:?}", n);
    temp = number;
    while temp > 0 {
        let d = temp % 10;
        temp /= 10;
        sum += d.pow(n);
    }
    sum == number
}

fn perfect_number(number: i32) -> bool{
    let mut sum = 0;
    for i in 1..=(number)/2 {
        if number % i == 0 {
            sum += i;
        }
    }
    sum == number && number > 0
}

fn factorization (number: i128) {
    let mut temp = 2;
    let mut n = number;
    let sqrt_of_num = (number as f64).powf(0.5) as i128;
    while n > 1 && temp <= sqrt_of_num {
        while n % temp == 0 {
            println!("{:?}",temp);
            n /= temp;
        }
        temp+=1;
    }
    if n > 1 {
        println!("{:?}",n);
    }
}

fn pow_mod(mut x: u128, mut n: u128, p: u128) -> u128 {
    if p == 1 {
        return 0;
    }
    let mut temp = 1;
    let mut b = x;
    let mut e = n;
    while e > 0 {
        if e%2 == 1 {
            temp *= b;
        }
        b *= b;
        e /= 2;
    }
    temp%p
}


// Napisz funkcję pow_mod(x: u128, n: u128, p: u128) -> u128 która obliczy (x^n)%p w taki sposób,
// by działało to prawidłowo dla jak największych danych. W celu ewentualnej optymalizacji czasowej użyj algorytmu szybkiego potęgowania.

fn main() {
    //print_ascii();
    //println!("{} dla {}",collatz(12,0), 12);
    //println!("{}",armstrong(1741725));
    //println!("{}",perfect_number(496));
    //println!("{:?}",factorization(121244));
    //println!("{:?}", pow_mod(223,10,43));


}
