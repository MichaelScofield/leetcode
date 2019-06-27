/* The guess API is defined in the parent class GuessGame.
   @param num, your guess
   @return -1 if my number is lower, 1 if my number is higher, otherwise return 0
      int guess(int num); */

public class Solution extends GuessGame {
    public int guessNumber(int n) {
        int i = 0, j = n;
        while (i < j) {
            int m = i / 2 + j / 2;
            switch (guess(m)) {
                case -1:
                    j = m - 1;
                    break;
                case 1:
                    i = m + 1;
                    break;
                case 0:
                    return m;
                default:
                    throw new AssertionError();
            }
        }
        return i;
    }
}
