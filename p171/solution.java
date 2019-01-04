public class Solution {
    public int titleToNumber(String s) {
        char[] chars = s.toCharArray();
        int n = 0;
        for (int i = 0; i < chars.length; i++) {
            char c = chars[i];
            n += (c - 'A' + 1) * Math.pow(26, chars.length - i - 1);
        }
        return n;
    }
}
