use prog11::module_test;
use prog11::Point;


fn main() {
    println!("Hello, world!");
    module_test();
    // let p = Point { y: 3.0, x: 4.0 };
    let mut p = Point::new(4.0, 3.0);
    println!("Distance = {}", p.distance_from_origin());
    // println!("x = {}", p.x);


    println!("Het punt is ({}, {})", p.get_x(), p.get_y()); // => (4, 3)

    // Zet de `x` waarde op -4.0 (toegestaan)
    p.set_x(-4.0);

    // Zet de `y` waarde op -5.0 (niet toegestaan!)
    p.set_y(-5.0);

    println!("Het punt is nu ({}, {})", p.get_x(), p.get_y()); // => (-4, 3)

    let pi = 3.14159;                // Types worden berekend door de compiler, maar kunnen handmatig
    let s = String::from("Hello World");  //    worden toegekend. Soms is dit zelfs noodzakelijk (error).
    println!("Square (borrow): {}", square_borrow(&pi));  // pi wordt geleend               ✅  (let op: &pi ipv pi)
    println!("Len (borrow): {}", len_borrow(&s));         // p wordt geleend                ✅  (let op: &s ipv s)
    println!("Square (move): {}", square_move(pi));       // pi wordt gekopieerd            ✅
    println!("Square (move): {}", square_move(pi));       // pi wordt gekopieerd            ✅
    println!("Len (move): {}", len_move(s.clone()));      // p wordt expliciet gekopieerd   ✅        lines te swappen
    println!("Len (move): {}", len_move(s));              // p wordt verplaatst             ⚠️  TODO: fix de error door


    let mut m = Point::new(4.0, 3.0);     // m is mutable
    let foo_m: &mut f64 = prog11::foo(&mut m);    // foo verwacht een mutable borrow, dus &mut m
    // foo_m is een mutable borrow van een 64-bits float
    let bar_foo_m = prog11::bar (foo_m);          // bar verwacht een mutable borrow, foo_m heeft het juiste type
    println!("{}", bar_foo_m);

    let m2 = m.clone();
    dbg!(m2);

    let mut x = 42;
    let mut y = 67;
    swap_pointer(&mut x, &mut y);
    dbg!(x, y);

    let mut hello = String::from("Hello world");
    exclaim(&mut hello);
    dbg!(&hello);

    let cloned = send_in_the_clones(&hello);
    dbg!(cloned);

}

fn swap_pointer(a: &mut usize, b: &mut usize) {
    let old_a = *a;
    *a = *b;
    *b = old_a;
}

fn exclaim(string: &mut String) {
    string.push('!');
}

fn send_in_the_clones(string: &String) -> String {
    string.clone()
}


pub fn len_move(s: String) -> usize { s.len() }
pub fn len_borrow(s: &String) -> usize { s.len() }
pub fn square_move(num: f64) -> f64 { num * num }
pub fn square_borrow(num: &f64) -> f64 { num * num }
