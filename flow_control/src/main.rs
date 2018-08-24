fn main() {
    // let _number = if true { 5 } else { 6 };
    // println!("{}", _number);

    // loop {
    //     println!("Rustaceous loop");
    //     break;
    // }

    // for number in 1..4 {
    //     println!("{}", number);
    // }

    // let fib_ret = fib(10);
    // println!("{}", fib_ret)

    sing_twelve_days_of_christmas();
}

// fn fib(n: i32) -> i32 {
//     if n < 2 {
//         return n;
//     }

//     return fib(n - 1) + fib(n - 2);
// }

fn sing_twelve_days_of_christmas() {
    let gifts = [
        "Partridge",
        "Turtle Doves",
        "French Hens",
        "Calling Birds",
        "Golden Rings",
        "Geese a Laying",
        "Swans A Swimming",
        "Maids a Milking",
        "Ladies Dancing",
        "Lords a Leaping",
        "Pipers Piping",
        "Drummers Drumming",
    ];

    for day in 0..gifts.len() {
        println!(
            "On the {} day of Christmas \n my true love sent to me: \n",
            day + 1
        );

        if day > 0 {
            for number in (1..day + 1).rev() {
                println!("{} {}", number + 1, gifts[number]);
            }
        }

        println!("A Partridge in a Pear Tree\n");
    }
}
