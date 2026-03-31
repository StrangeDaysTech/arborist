func classify(x: Int) -> String {
    switch x {
    case let n where n > 100:
        return "big"
    case let n where n > 50:
        return "medium"
    case let n where n > 0:
        return "small"
    default:
        return "non-positive"
    }
}
