fun checkAll(a: Boolean, b: Boolean, c: Boolean): Boolean {
    return a && b && c
}

fun checkMixed(a: Boolean, b: Boolean, c: Boolean): Boolean {
    return a && b || c
}
