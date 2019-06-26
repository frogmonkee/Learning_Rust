fn main() {
    let mut meta_counter = 0;
    let mut _counter = 0;
    let _days = ["1st", "2nd", "3rd", "4th", "5th", "6th", "7th", "8th", "9th", "10th", "11th", "12th"];
    let _verses = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "10 lords a-leaping",
        "11 pipers piping",
        "12 drummers drumming"
    ];
    while meta_counter < 12 {
        println!("On the {:#?} day of christmas my true love sent to me {:#?}", _days[meta_counter], _verses[meta_counter]);
        while _counter < meta_counter {
            println!("{:#?}", _verses[_counter]);
            if _counter > 0 {
                _counter -= 1;            
            } 
            else {_counter = meta_counter;}
        }
        _counter = meta_counter;
        meta_counter += 1;      
    }
}
