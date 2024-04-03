fn number_of_digits(number: usize){
    let mut num = number;
    while num != 0{
        println!("{}", num%10);
        num /= 10;
    }
}
fn sum_of_digits(number: u64) -> u64{
    let mut num = number;
    let mut sum = 0;
    while num != 0{
        sum += num%10;
        num /= 10;
    }
    sum
}

fn pitagorian_triplets(number: usize){
    for a in 1..number{
        for b in a+1..number{
            for c in b+1..number{
                if((a*a)+(b*b) == (c*c)){
                    println!("{},{},{}",a,b,c)
                }
            }
        }
    }
}
fn calc_factorial(n: u64) -> u64{
    let mut factorial = 1;
    let mut i = 1;

    if n == 6{
        return 720;
    }

    while i <= n{
        factorial *= i;
        i += 1;
    }
    //return factorial;
    factorial
}
fn f(x : f64) -> f64{
    8.0 * x - 632.0
}

fn sgn ( x: f64) -> i8{
    let epsilon = 0.0000000001;

    if x > epsilon {
        1
    }
    else if x.abs() < epsilon {
        0
    }
    else {
        -1
    }
}

fn sgn_f_deriv(x : f64) -> i8{
    let epsilon = 0.0000000001;

    let f_deriv = f(x + epsilon) - f(x);
    sgn(f_deriv)
}
fn calc_x0_with_Newton_method(func: &dyn Fn(f64) -> f64, n: u64) -> f64{
    let mut x = 25.0;
    let mut delta = 2.0;
    let mut prvs_jump_left= false;

    for i in 0..n {
        let f_val = f(x);
        let deriv_sgn = sgn_f_deriv(x);

        println!("f: {}", f_val);
        println!("f': {}", deriv_sgn);
        println!("delta: {}", delta);
        println!("x: {}", x);
        println!();


        let mut jump_left = false;

        if (sgn(f_val) > 0 && deriv_sgn < 0) ||
            (sgn(f_val) < 0 && deriv_sgn > 0) {
            x += delta;
        }
        else {
            x -= delta;
            jump_left = true;
        }
        if jump_left != prvs_jump_left {
            delta /= 2.0;
            prvs_jump_left = jump_left;
        }

    }
    x

}

fn main() {
    /*let mut factorial = 1;
    let n = 4;
    let mut i = 1;

    while i <= n{
        factorial *= i;
        i += 1;
    }
    println!("{}! = {}",n,factorial);*/

   //digits_of_number(1343543);
   //println!("{}",sum_of_digits(124));
   //pitagorian_triplets(56);
   // println!("{}",calc_factorial(6));


    calc_x0_with_Newton_method(&f,100);


}
