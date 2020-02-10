fn display_lyrics_for_day(day: u32) {
    let mut days_to_presents = vec![
        ("first", "And a partridge in a pear tree."),
        ("second", "Two turtle doves"),
        ("third", "Three French hens"),
        ("fourth", "Four calling birds"),
        ("fifth", "Five gold rings"),
        ("sixth", "Six geese a-laying"),
        ("seventh", "Seven swans a-swimming"),
        ("eight", "Eight maids a-milking"),
        ("ninth", "Nine ladies dancing"),
        ("tenth", "Ten lords a-leaping"),
        ("eleventh", "Eleven pipers piping"),
        ("twelfth", "Twelve drummers drumming"),
    ];

    let details = days_to_presents[day as usize];

    let day_in_english = details.0;
    let present_for_day_in_english: &str;

    if day == 0 {
        // Special case lyrics for first day.
        present_for_day_in_english = "A partridge in a pear tree.";
    } else {
        present_for_day_in_english = details.1;
    }

    println!(
        "\nOn the {} day of Christmas, my true love sent to me\n{}",
        day_in_english, present_for_day_in_english
    );

    days_to_presents.reverse();

    for (index, details) in days_to_presents.iter().enumerate() {
        if index > days_to_presents.len() - day as usize - 1 {
            println!("{}", details.1);
        }
    }
}

pub fn run() {
    // Display the lyrics for the twelve days.
    for day in 0..12 {
        display_lyrics_for_day(day);
    }
}
