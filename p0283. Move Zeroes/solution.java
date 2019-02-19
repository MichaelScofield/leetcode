public class Solution {
    public void moveZeroes(int[] nums) {
        for (int i = 0; i < nums.length; ++i) {
            if (nums[i] == 0) {
                int nextPositiveNumIndex = findNextNonZeroNumIndex(nums, i);
                if (nextPositiveNumIndex < 0) {
                    return;
                }
                if (nextPositiveNumIndex < nums.length) {
                    swap(nums, i, nextPositiveNumIndex);
                }
            }
        }
    }

    private int findNextNonZeroNumIndex(int[] nums, int startIndex) {
        int i;
        for (i = startIndex + 1; i < nums.length; i++) {
            if (nums[i] != 0) {
                break;
            }
        }
        return i < nums.length ? i : -1;
    }

    private void swap(int[] nums, int i, int j) {
        int tmp = nums[i];
        nums[i] = nums[j];
        nums[j] = tmp;
    }
}
