public class Solution {
    public int addDigits(int num) {
        while (num / 10 > 0) {
            int _num = 0;
            while (num / 10 > 0) {
                _num += num % 10;
                num /= 10;
            }
            num += _num;
        }
        return num;
    }
}
