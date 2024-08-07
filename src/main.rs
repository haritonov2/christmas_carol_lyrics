fn main() {
    let lines: [String; 13] = [
        String::from("On the twelfth day of Christmas, my true love sent to me"),
        String::from("A partridge in a pear tree"),
        String::from("Two turtle doves and"),
        String::from("Three french hens"),
        String::from("Four calling birds"),
        String::from("Five golden rings"),
        String::from("Six geese a-laying"),
        String::from("Seven swans a-swimming"),
        String::from("Eight maids a-milking"),
        String::from("Nine ladies dancing"),
        String::from("Ten lords a-leaping"),
        String::from("Eleven pipers piping"),
        String::from("Twelve drummers drumming"),
    ];

    for block_number in 1..13 {
        // Verse 1
        println!("[Verse {}]", block_number);
        print_block(&lines, block_number);
    }
}

fn print_block(lines: &[String; 13], block_number: u8) {
    println!("{}", lines[0]);

    for line_index in (1..block_number).rev() {
        println!("{}", lines[line_index as usize]);
    }
}
