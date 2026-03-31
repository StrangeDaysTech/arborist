fun processItems(items: List<Int>): Int {
    var count = 0
    if (items.isNotEmpty()) {           // +1 cognitive (nesting=0), +1 cyclomatic
        for (item in items) {           // +2 cognitive (nesting=1), +1 cyclomatic
            if (item > 0) {             // +3 cognitive (nesting=2), +1 cyclomatic
                count += item
            }
        }
    }
    return count
}
