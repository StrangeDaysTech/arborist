// Expected metrics:
// apply_filter: cognitive=3, cyclomatic=3, sloc=14
// Cognitive: closure increments nesting to 1,
//   if at nesting 1: +1+1=2, else_clause: +1 => total=3
// Cyclomatic: 1(base) + 1(if) + 1(else_clause) = 3
fn apply_filter(items: &[i32]) -> Vec<i32> {
    let threshold = 10;
    items
        .iter()
        .filter(|&&x| {
            if x > threshold {
                true
            } else {
                false
            }
        })
        .copied()
        .collect()
}
