#[allow(dead_code)]
pub fn beer_song(number: u32) {
    for i in (1..number).rev() {
        if i == 1 {
            println!(
                "One bottle of beer on the wall, one bottle of beer.
            Take it down and pass it around, no bottles of beer on the wall."
            );
        } else {
            println!(
                "{0} bottles of beer on the wall, {0} bottles of beer.
            Take one down and pass it around, {1} bottles of beer on the wall.",
                i,
                i - 1
            );
        }
    }
    println!(
        "No more bottles of beer on the wall, no more bottles of beer.
            Go to the store and buy some more, {0} bottles of beer on the wall.",
        number
    );
}
