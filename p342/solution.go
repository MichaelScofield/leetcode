
func isPowerOfFour(num int) bool {
    if num < 0 {
        return false
    }
    zeroBytesCount := 0
    firstByte := num & 0xFF000000 >> 24
    if firstByte == 0 {
        zeroBytesCount++
    }
    secondByte := num & 0x00FF0000 >> 16
    if secondByte == 0 {
        zeroBytesCount++
    }
    thirdByte := num & 0x0000FF00 >> 8
    if thirdByte == 0 {
        zeroBytesCount++
    }
    lastByte := num & 0x000000FF
    if lastByte == 0 {
        zeroBytesCount++
    }
    if zeroBytesCount < 3 {
        return false
    }
    return isPowerOfFourAsOneByte(firstByte) ||
        isPowerOfFourAsOneByte(secondByte) ||
        isPowerOfFourAsOneByte(thirdByte) ||
        isPowerOfFourAsOneByte(lastByte)
}

func isPowerOfFourAsOneByte(b int) bool {
    return b == 0x40 || b == 0x10 || b == 0x04 || b == 0x01
}
