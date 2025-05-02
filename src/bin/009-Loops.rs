fn main() {
    let mut count = 0;

    loop {
        count += 1;

        if count == 10 {
            break;
        }

        println!("count is {}", count);
    }
}