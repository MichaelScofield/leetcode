func imageSmoother(M [][]int) [][]int {
    grays := make([][]int, len(M))
    for i, l1 := 0, len(M); i < l1; i++ {
        grays[i] = make([]int, len(M[i]))
    }

    for i, l1 := 0, len(M); i < l1; i++ {
        for j, l2 := 0, len(M[i]); j < l2; j++ {
            sum := M[i][j]
            count := 1

            if i > 0 {
                // up left
                if j > 0 {
                    sum += M[i-1][j-1]
                    count++
                }

                // up middle
                sum += M[i-1][j]
                count++

                // up right
                if j < l2-1 {
                    sum += M[i-1][j+1]
                    count++
                }
            }

            // left
            if j > 0 {
                sum += M[i][j-1]
                count++
            }

            // right
            if j < l2-1 {
                sum += M[i][j+1]
                count++
            }

            if i < l1-1 {
                // down left
                if j > 0 {
                    sum += M[i+1][j-1]
                    count++
                }

                // down middle
                sum += M[i+1][j]
                count++

                // down right
                if j < l2-1 {
                    sum += M[i+1][j+1]
                    count++
                }
            }

            grays[i][j] = sum / count
        }
    }
    return grays
}
