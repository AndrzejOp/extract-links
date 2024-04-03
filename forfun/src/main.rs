fn pw_o_1(mut x: i32) -> i32{
    x += 1;
    x
}
fn pws_o_1(x: &mut i32) {
    *x += 1;
}

fn main() {
    // let mut  a = 12;
    // let b = pw_o_1(a);
    // println!("{}", b == 13);
    //
    // pws_o_1(&mut a);
    // println!("{}", a == 13);
    // pws_o_1(&mut a);
    // println!("{}", a == 14);

    // let mut s1 = "Ala ma kota".to_string();
    //
    // s1.push_str(" i psa");
    // s1.push('.');
    // println!("{}", s1);
    //
    // println!("{:?}", s1.find('a'));
    // println!("{:?}", s1.find('x'));
    // println!("{:?}", s1.find("a"));
    // println!("{:?}", s1.find("kot"));
    //
    // let s4 = s1.replace("kota", "szczura");
    //
    // let a = s1.as_bytes();
    // println!("{:?}", a);

    let s0 = "Witaj swiecie";

    for c in s0.chars() {
        println!("{}", c);
    }

    for c in s0.bytes() {
        println!("{}", c);
    }

}
