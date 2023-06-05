fn main() {
    // my Version
    let lyrics = [
        ["First", "partridge in a pear tree"],
        ["Second", "Two turtle doves"],
        ["Third", "Three French hens"],
        ["Fourth", "Four calling birds"],
        ["Fifth", "Five golden rings"],
        ["Sixth", "Six geese a-laying"],
        ["Seventh", "Seven swans a-swimming"],
        ["Eighth", "Eight maids a-milking"],
        ["Ninth", "Nine ladies dancing"],
        ["Tenth", "Ten lords a-leaping"],
        ["Eleventh", "Eleven pipers piping"],
        ["Twelfth", "Twelve drummers drumming"],
    ];

    for i in 0..12 {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            lyrics[i][0]
        );
        for j in (0..i + 1).rev() {
            if j == 0 {
                if i > 0 {
                    print!("And a ")
                } else {
                    print!("A ")
                }
            }

            let lyric = lyrics[j][1];
            println!("{lyric}");
        }
    }

    // Chat GPT
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for day in 0..12 {
        println!("On the {} day of Christmas,", days[day]);
        println!("My true love gave to me:");

        for gift in (0..=day).rev() {
            if day == 0 && gift == 0 {
                println!("{}", gifts[gift]);
            } else if gift == 0 {
                println!("And {}", gifts[gift]);
            } else {
                println!("{},", gifts[gift]);
            }
        }

        println!();
    }
}
