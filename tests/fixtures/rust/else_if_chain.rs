// Expected metrics:
// classify: cognitive=9, cyclomatic=7, sloc=11
// Cognitive breakdown:
//   if(+1, nesting 0) + else_clause(+1) + if(+1+1 nesting) + else_clause(+1)
//   + if(+1+1 nesting) + else_clause(+1) + else_clause(+1) = 9
// Cyclomatic: 1(base) + 3(if) + 3(else_clause) = 7
fn classify(x: i32) -> &'static str {
    if x > 100 {
        "high"
    } else if x > 50 {
        "medium"
    } else if x > 0 {
        "low"
    } else {
        "negative"
    }
}
