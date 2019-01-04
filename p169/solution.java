import java.util.Arrays;

public class Solution {
    public int majorityElement(int[] nums) {
        Arrays.sort(nums);

        int majority, lastMajority;
        majority = lastMajority = nums[0];
        int times, lastTimes;
        times = lastTimes = 1;
        for (int i = 1; i < nums.length; i++) {
            int num = nums[i];
            if (majority == num) {
                times += 1;
            } else {
                if (times > lastTimes) {
                    lastMajority = majority;
                    lastTimes = times;
                }
                majority = num;
                times = 1;
            }
        }
        if (times > lastTimes) {
            lastMajority = majority;
        }
        return lastMajority;
    }
}
