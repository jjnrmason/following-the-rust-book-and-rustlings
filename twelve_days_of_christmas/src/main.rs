fn main() {
    for i in 1..13 {
        println!("On the {i} day of Christmas\nmy true love sent to me");
        for y in (1..i+1).rev() {
            if y == 1 {
                println!("a partridge in a pear tree.");
            } else if y == 2 {
                println!("Two turtle doves,");
            } else if y == 3 {
                println!("Three French hens,");
            } else if y == 4 {
                println!("Four calling birds,");
            } else if y == 5 {
                println!("Five golden rings,");
            } else if y == 6 {
                println!("Six geese a-laying,");
            } else if y == 7 {
                println!("Seven swans a-swimming,");
            } else if y == 8 {
                println!("Eight maids a-milking,");
            } else if y == 9 {
                println!("Nine ladies dancing,");
            } else if y == 10 {
                println!("Ten lords a-leaping,");
            } else if y == 11 {
                println!("Eleven pipers piping,");
            } else if y == 12 {
                println!("Twelve drummers drumming,");
            } else {
                continue;
            }
        }
    }
}