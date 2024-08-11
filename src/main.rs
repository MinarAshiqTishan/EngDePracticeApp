// TODO:
// 1. interactive loop until user inputs exit()
// 2. pretty-fy interaction . like a interpreter or a terminal
// 3. query the .db file with sql command
// 4. print the output of query
// 5. query according to
//      a. difficulty of sentences
//      b. randomizer
// 6. let user decide on difficulty.
// 7. make the user choice persistent
// 8. make the user result of practice persistent and queriable

fn main() {
    let mut i = 0;
    loop {
        println!("{i}");
        if i == 2 {
            println!("Hello, world!");
            break;
        }
        i += 1;
    }
}
