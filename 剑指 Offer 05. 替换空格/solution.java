class Solution {
    public String replaceSpace(String s) {
        if (s == null || s.isEmpty()) {
            return s;
        }
        int whitespaces = (int) s.chars().filter(c -> c == ' ').count();
        if (whitespaces < 1) {
            return s;
        }
        char[] chars = s.toCharArray();
        char[] newChars = new char[chars.length - whitespaces + whitespaces * 3];
        for (int i = chars.length - 1, j = newChars.length - 1; i >= 0; i--) {
            if (chars[i] == ' ') {
                newChars[j--] = '0';
                newChars[j--] = '2';
                newChars[j--] = '%';
            } else {
                newChars[j--] = chars[i];
            }
        }
        return new String(newChars, 0, newChars.length);
    }
}
