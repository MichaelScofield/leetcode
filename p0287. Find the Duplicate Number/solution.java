class Solution {
     public int findDuplicate(int[] nums) {
        if (nums == null) {
            return -1;
        }
        int i = 1, j = nums.length - 1;
        while (i != j) {
            int mid = i + (j - i) / 2;
            int c = countNumsInRange(nums, i, mid);
            int expected = mid - i + 1;
            if (c <= expected) {
                i = mid + 1;
            } else {
                j = mid;
            }
        }
        return i;
    }

    private int countNumsInRange(int[] nums, int i, int j) {
        return (int) Arrays.stream(nums).filter(n -> n >= i && n <= j).count();
    }
}
