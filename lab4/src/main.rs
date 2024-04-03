use std::time::{SystemTime, UNIX_EPOCH};

fn d2(a:(f64, f64), b: (f64, f64)) -> f64{
    let dif1 = a.0 - b.0;
    let dif2 = a.1 - b.1;
    let sum = dif1*dif1 + dif2*dif2;
    sum.sqrt()

}
fn d3(a:(f64, f64, f64), b: (f64, f64, f64)) -> f64{
    let dif1 = a.0 - b.0;
    let dif2 = a.1 - b.1;
    let dif3 = a.2 - b.2;
    let sum = dif1*dif1 + dif2*dif2 + dif3*dif3;
    sum.sqrt()

}

fn print_backwards(){
    const N: usize = 76;
    let mut tab: [u32;N] = [0;N];
    for i in 1..=N{
        tab[i - 1] = 100 % i  as u32;
    }
 //   println!("{:?}", tab);
    for i in 1..=N{
        print!("{:?} ",tab[N - i]);
    }
}

fn avg(tab: &[u32]) -> f32{
    let mut sum = 0;
    for i in tab{
        sum += i;
    }
    sum as f32/tab.len() as f32
}

fn sort(a: &mut u32,  b: &mut u32, c: &mut u32) {
    if a > b{
        let tmp = *a;
        *a = *b;
        *b = tmp;
    }
    if b > c {
        let tmp = *b;
        *b = *c;
        *c = tmp;
    }
    if a > b {
        let tmp = *a;
        *a = *b;
        *b = tmp;
    }
}

fn swap_range(tab:&mut [u32], r1:(usize, usize), r2:(usize, usize)){
    let mut range1 = r1;
    let mut range2 = r2;

    let len1 = range1.1 - range1.0;
    let len2 = range2.1 - range2.0;

    if len1 > len2 {
        range1.1 = range1.0 + len2;
    }
    else if len2 > len1{
        range2.1 = range2.0 + len1;
    }

    let mut offset = 0;
    for i in range1.0..=range1.1 {
        let tmp = tab[range2.0 + offset];
        tab[range2.0 + offset] = tab[i];
        tab[i] = tmp;
        offset += 1;

    }
}

fn rand(mut seed: u128, min: u128, max: u128) -> u128 {
    const A: u128 = 6364136223846793005;
    const C: u128 = 1442695040888963407;
    const M: u128 = ( 1 << 63 ) - 1;
    let temp = max - min + 1;
    seed *= (A * C) % M;
    seed % temp + min
}


fn main() {
    // let mut krotka = (1,2,3.0);
    // let tab = [1,2,3];
    // println!("{}", krotka.0);
    // println!("{}", krotka.0);
    // println!("{}", krotka.1);
    // println!("{}", krotka.2);
    // krotka.0 = 6;
    // krotka = (1,8,9.0);
    // println!("{}", krotka.0);
    // println!("{}", krotka.1);
    // println!("{}", krotka.2);
    //
    // let k1 = (1.9,2.0,2.5);
    // let k2 = (1.2,4.2,4.6);
    // println!("{}",d3(k1,k2));
    //
    //
    // let array: [i32;3] = [0,1,2];
    // for i in array{
    //     println!("{}", i);
    // }
    // for i in 0..3{
    //     print!("{}", array[i]);
    // }



    //print_backwards();
    // let tab = [1,2,3,4,5,6 ];
    // println!("{}",avg(&tab));
    // let mut  a = 5;
    // let mut b = 32;
    // let mut c = 1;
    // sort(&mut a,&mut b,&mut c);
    // println!("{}, {}, {}",a,b,c);
    // let mut tab = [0,1,2,3,4,5,6,7,8,9];
    // let p1 = (1,4);
    // let p2 = (5,9);
    // swap_range(&mut tab,p1,p2);
    //
    // println!("{:?}",tab);

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("WRONG")
        .as_millis();
    println!("{:?}",rand(seed, 1, 3457));

}
