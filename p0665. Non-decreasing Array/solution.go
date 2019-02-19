
func checkPossibility(nums []int) bool {
    if isNonDecreasingArray(nums) {
        return true
    }
    for i, l := 1, len(nums); i < l; i++ {
        if nums[i-1] > nums[i] {
            stash := nums[i-1]
            nums[i-1] = nums[i]
            if isNonDecreasingArray(nums) {
                return true
            } else {
                nums[i-1] = stash
                nums[i] = nums[i-1]
                return isNonDecreasingArray(nums)
            }
        }
    }
    return true
}

func isNonDecreasingArray(nums []int) bool {
    l := len(nums)
    if l <= 1 {
        return true
    }
    for i := 1; i < l; i++ {
        if nums[i-1] > nums[i] {
            return false
        }
    }
    return true
}
