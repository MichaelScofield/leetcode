public class Solution {
    public int firstUniqChar(String s) {
        char[] chars = s.toCharArray();
        l:
        for (int i = 0; i < chars.length; i++) {
            char c = chars[i];
            for (int j = 0; j < chars.length; ++j) {
                if (i == j) {
                    continue;
                }
                char aChar = chars[j];
                if (c == aChar) {
                    continue l;
                }
            }
            return i;
        }
        return -1;
    }
}
