
fn main(){

    let _twelve_days = [   
    ("first", "A partridge in a pear tree"),
    ("second", "Two turtle doves"), 
    ("third", "Three French hens"),
    ("fourth", "Four calling birds"),
    ("fifth", "Fiiiiive golden rings"),
    ("sixth", "Six geese a-laying"),
    ("seventh", "Seven swans a-swimming"),
    ("eighth", "Eight maids a-milking"),
    ("ninth", "Nine ladies dancing"),
    ("tenth", "Ten lords a-leaping"),
    ("eleventh", "Eleven pipers piping"),
    ("twelfth", "Twelve drummers drumming"),
    ];

   

    for (index, tup) in _twelve_days.iter().enumerate() { //returns (usize, &(&str, &str))
        println!("On the {} day of Christmas, my true love gave to me", tup.0);
    
        println!("{}", tup.1);

        for i in (0..index).rev() {
            let tup = &_twelve_days[i]; //uses the concept of shadowing
            println!("{}", tup.1);

        }
        
    }

}
