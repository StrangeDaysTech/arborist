// Expected metrics:
// categorize: cognitive=4, cyclomatic=6, sloc=16
function categorize(x) {
    switch (x) {
        case 1:
            return "one";
        case 2:
            if (x > 0) {
                return "positive two";
            } else {
                return "two";
            }
        case 3:
            return "three";
        default:
            return "other";
    }
}
