# Expected metrics:
# categorize: cognitive=3, cyclomatic=6, sloc=12
def categorize(x):
    match x:
        case 1:
            return "one"
        case 2:
            if x > 0:
                return "positive two"
            return "two"
        case 3:
            return "three"
        case _:
            return "other"
