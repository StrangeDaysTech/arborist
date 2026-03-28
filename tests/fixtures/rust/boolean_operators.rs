// Expected metrics:
// check_same: cognitive=3, cyclomatic=4, sloc=6
// check_mixed: cognitive=3, cyclomatic=4, sloc=6
fn check_same(a: bool, b: bool, c: bool) -> bool {
    if a && b && c {
        return true;
    }
    false
}

fn check_mixed(a: bool, b: bool, c: bool) -> bool {
    if a && b || c {
        return true;
    }
    false
}
