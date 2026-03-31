fun classify(x: Int): String {
    return when {
        x > 100 -> "big"
        x > 50 -> "medium"
        x > 0 -> "small"
        else -> "non-positive"
    }
}
