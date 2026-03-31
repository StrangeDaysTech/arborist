fun classify(x: Int): String {
    if (x > 100) {
        return "big"
    } else if (x > 50) {
        return "medium"
    } else if (x > 0) {
        return "small"
    } else {
        return "non-positive"
    }
}
