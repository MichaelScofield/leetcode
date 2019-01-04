public class Solution {
    public char findTheDifference(String s, String t) {
        char[] c1 = s.toCharArray();
        char[] c2 = t.toCharArray();
        loop:
        for (char c : c2) {
            for (int i = 0; i < c1.length; i++) {
                char _c = c1[i];
                if (_c == c) {
                    c1[i] = Character.MIN_VALUE;
                    continue loop;
                }
            }
            return c;
        }
        throw new AssertionError();
    }
}
