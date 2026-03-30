// Expected metrics:
// categorize: cognitive=3, cyclomatic=6, sloc=20

class MatchSwitch {
    int categorize(int code) {
        int result;
        switch (code) {
            case 1:
                result = 10;
                break;
            case 2:
                result = 20;
                break;
            case 3:
                if (code > 0) {
                    result = 30;
                }
                break;
            default:
                result = -1;
                break;
        }
        return result;
    }
}
