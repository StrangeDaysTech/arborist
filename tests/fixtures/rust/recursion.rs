// Expected metrics:
// factorial: cognitive=3, cyclomatic=3, sloc=7
// Cognitive: if(+1, nesting 0) + else_clause(+1) + recursive call(+1) = 3
// Cyclomatic: 1(base) + 1(if) + 1(else_clause) = 3
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}
