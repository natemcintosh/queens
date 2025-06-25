fn main() {
    let x: u64 = 1;
    // Print out what happens when the bit is shifted "backwards"
    for shift in 0..=64 {
        // Using wrap-around operators
        let y = x.rotate_left(shift);
        println!("x.rotate_left({shift}) = {y:#064b}");
    }
}
