fn main() {
    // let mut s = String::from("hello");

    // change(&mut s);

    // println!("{}", s);

    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    let reference_to_nothing = dangle();
}

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

fn dangle() -> String {
    let s = String::from("hello");
    s
}
