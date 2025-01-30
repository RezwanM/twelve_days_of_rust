fn main() {
    println!("Welcome! Here are the lyrics to the song Twelve Days of Christmas.\n");
    
    print_lyrics();
}

fn print_lyrics() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth",
                "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    
    let words = ["a patridge in a pear tree",
                 "two turtle doves, and",
                 "three French hens,",
                 "four calling birds,",
                 "five gold rings,",
                 "six geese a-laying,",
                 "seven swans a-swimming,",
                 "eight maids a-milking,",
                 "nine ladies dancing,",
                 "ten lords a-leaping,",
                 "eleven pipers piping,",
                 "twelve drummers drumming,"
                ];
    
    let mut iter = 0;
    
    for day in days {
        println!("On the {day} day of Christmas, my true love gave to me");
        
        let mut index = iter+1;

        while index > 0 {
            println!("{}", words[index-1]);
            index -= 1;
        }
        println!("");
        iter += 1;
    }
}

