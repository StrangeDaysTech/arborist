// Expected metrics:
// process: cognitive=6, cyclomatic=4, sloc=11
fn process(items: &[i32]) -> i32 {
    let mut sum = 0;
    if !items.is_empty() {
        for item in items {
            if *item > 0 {
                sum += item;
            }
        }
    }
    sum
}
