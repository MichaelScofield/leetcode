public class Solution {
    public String reverseString(String s) {
        if (s == null || s.length() <= 1) {
            return s;
        }
        int l = s.length();
        char[] chars = new char[l];
        for (int i = 0, j = l - 1; i < l; ++i) {
            chars[j--] = s.charAt(i);
        }
        return new String(chars);
    }
}
