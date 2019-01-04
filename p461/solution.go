func hammingDistance(x int, y int) int {
    d := 0
	z := x ^ y
	for i := 0; i < 32; i++ {
		if z & 1 == 1 {
			d++
		}
		z = z >> 1
	}
	return d
}
