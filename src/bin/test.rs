use scientific::*;

fn main() {
    let v1 = S::s(0.2);
    let v2 = S::s(0.3);
    let v3 = S::s(0.5);

    println!("{} {}", v1 + v2, v3);
    assert!((v1 + v2).could_eq(&v3));
}
