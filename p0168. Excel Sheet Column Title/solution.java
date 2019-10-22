class Solution {
    public String convertToTitle(int n) {
        List<Character> chars = new ArrayList<>(10);
        while (n > 0) {
            int r = n % 26;
            if (r == 0) {
                r += 26;
            }
            chars.add((char) ('A' + r - 1));
            n = (n - r) / 26;
        }
        Collections.reverse(chars);
        char[] _chars = new char[chars.size()];
        for (int i = 0, l = chars.size(); i < l; i++) {
            _chars[i] = chars.get(i);
        }
        return new String(_chars);
    }
}
