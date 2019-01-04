public class Solution {
    public int[] intersection(int[] nums1, int[] nums2) {
        Set<Integer> set1 = new HashSet<Integer>();
        addAll(nums1, set1);
        Set<Integer> set2 = new HashSet<Integer>();
        addAll(nums2, set2);

        List<Integer> list = new ArrayList<Integer>();
        for (Integer next : set1) {
            if (set2.contains(next)) {
                list.add(next);
            }
        }
        int[] result = new int[list.size()];
        for (int i = 0; i < list.size(); i++) {
            Integer integer = list.get(i);
            result[i] = integer;
        }
        return result;
    }

    private void addAll(int[] nums, Set<Integer> set) {
        if (nums != null) {
            for (int i : nums) {
                set.add(i);
            }
        }
    }
}
