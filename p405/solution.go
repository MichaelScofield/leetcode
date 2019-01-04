
import (
    "strings"
)

func toHex(num int) string {
    if num == 0 {
        return "0"
    }

    s1 := getHexChar((num & 0xF0000000) >> 28)
    s2 := getHexChar((num & 0x0F000000) >> 24)
    s3 := getHexChar((num & 0x00F00000) >> 20)
    s4 := getHexChar((num & 0x000F0000) >> 16)
    s5 := getHexChar((num & 0x0000F000) >> 12)
    s6 := getHexChar((num & 0x00000F00) >> 8)
    s7 := getHexChar((num & 0x000000F0) >> 4)
    s8 := getHexChar(num & 0x0000000F)
    s := s1 + s2 + s3 + s4 + s5 + s6 + s7 + s8
    return strings.TrimLeft(s, "0")
}

func getHexChar(num int) string {
    switch num {
    case 0x0:
        return "0"
    case 0x1:
        return "1"
    case 0x2:
        return "2"
    case 0x3:
        return "3"
    case 0x4:
        return "4"
    case 0x5:
        return "5"
    case 0x6:
        return "6"
    case 0x7:
        return "7"
    case 0x8:
        return "8"
    case 0x9:
        return "9"
    case 0xA:
        return "a"
    case 0xB:
        return "b"
    case 0xC:
        return "c"
    case 0xD:
        return "d"
    case 0xE:
        return "e"
    case 0xF:
        return "f"
    default:
        return ""
    }
}
