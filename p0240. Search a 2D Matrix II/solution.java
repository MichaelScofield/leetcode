class Solution {
    public boolean searchMatrix(int[][] matrix, int target) {
        if (matrix == null || matrix.length == 0) {
            return false;
        }
        int m = matrix[0].length;
        int n = matrix.length;
        for (int i = m - 1, j = 0; i >= 0 && j < n; ) {
            if (matrix[j][i] == target) {
                return true;
            }
            if (matrix[j][i] < target) {
                j += 1;
            } else {
                i -= 1;
            }
        }
        return false;
    }
}
