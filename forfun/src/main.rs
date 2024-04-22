#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Punkt3D {
    x: f64,
    y: f64,
    z: f64,
}
//kopiowanie albo przenoszenie, copy jest niejawne, ogólnie robią to samo, copy zajmuje wiecej pamieci, bo nie mamy kontroli

fn main() {
    let mut p1 = Punkt3D {
        x: 4.5,
        z: -7.5,
        y: 3.0,
    };
    println!("{:?}", p1);
    let p2 = Punkt3D {
        z: 47.5,
        ..p1
    };
    println!("{:?}", p2);
    p1.x = 34.12;
    println!("{:?}", p1);
    println!("{:?}", p1 == p2);
    println!("{:?}", p1 < p2);
    println!("{:?}", p1 > p2);

    let p3 = p1.clone();
    let p4 = p1.clone();
    println!("{:?}", p1);
    println!("{:?}", p3);
    println!("{:?}", p4);
    println!("{:?}", p1 == p4);
}