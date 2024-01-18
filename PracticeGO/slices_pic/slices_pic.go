package main

import "golang.org/x/tour/pic"

func Pic(dx, dy int) [][]uint8 {
	pic_result := make([][]uint8, dy)
	for i := range pic_result {
		pic_result[i] = make([]uint8, dx)
		for j := range pic_result[i] {
			pic_result[i][j] = uint8((i + j) / 2)
		}
	}
	return pic_result
}

func main() {
	pic.Show(Pic)
}
