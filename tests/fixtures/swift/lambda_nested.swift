func transform(items: [Int]) -> [Int] {
    return items.filter { $0 > 0 }.map { $0 * 2 }
}
