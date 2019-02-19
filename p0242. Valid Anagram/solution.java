import java.math.BigInteger;

public class Solution {
    public boolean isAnagram(String s, String t) {
        int[] distribution1 = getLetterDistribution(s);
        int[] distribution2 = getLetterDistribution(t);
        for (int i = 0; i < 26; ++i) {
            if (distribution1[i] != distribution2[i]) {
                return false;
            }
        }
        return true;
    }

    private int[] getLetterDistribution(String s) {
        int[] distribution = new int[26];
        char[] chars = s.toCharArray();
        for (char aChar : chars) {
            distribution[aChar - 'a']++;
        }
        return distribution;
    }
}
