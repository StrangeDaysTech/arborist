// Expected metrics:
// categorize: cognitive=4, cyclomatic=6, sloc=14
fn categorize(x: i32) -> &'static str {
    match x {
        1 => "one",
        2 => {
            if x > 0 {
                "positive two"
            } else {
                "two"
            }
        }
        3 => "three",
        _ => "other",
    }
}
