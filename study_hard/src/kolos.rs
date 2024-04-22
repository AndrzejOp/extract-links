fn zad1 (arr: &[u32]) -> u32 {
    let mut temp = 100000;
    for i in 0..=9 {
        let help = arr.binary_search(&i);
        if help.is_err() && i < temp {
        temp = i;
        }

    }
    temp
}



fn zad3(arr: &mut[u32]) -> Vec<&u32>{
    let out: Vec<_> = arr.iter().rev().collect();
    out
}



fn main() {
    let mut arr = [0,1,6,5,3,7,4,8,9];
    let mut binding = arr.clone();
    println!("{}", zad1(&arr));
    println!();
    println!();
    for i in arr {
    print!("{} ", i);
    }
    println!();
    let result = zad3(&mut binding);
    for i in 0..arr.len() {
        arr[i] = *result[i];
    }
    for i in arr {
    print!("{} ", i);
    }
    println!();
}
