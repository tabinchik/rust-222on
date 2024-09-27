// Fix the error and fill the blanks
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
#[test]
fn main() {
    let v = Color(0, 127, 255);
    check_color(v);

    println!("Success!");
}

fn check_color(p: Color) {
    let Color (x, _, z) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
}