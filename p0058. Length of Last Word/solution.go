func lengthOfLastWord(s string) int {
    flip := true
    i, j := 0, 0
    for k, l := 0, len(s); k < l; k++ {
        if string(s[k]) != " " {
            if flip {
                i = k
                flip = false
            }
            j = k + 1
        } else {
            flip = true
        }
    }
    return j - i
}
