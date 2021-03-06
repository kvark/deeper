package main

import (
	"math"
	"math/cmplx"
	"math/rand"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

//Math
const sqrt2 float64 = 1.41421356237309504880168872420969807

func parts(c complex128) (float64, float64) {
	return real(c), imag(c)
}

func cDiv(c complex128, f float64) complex128 {
	return c / complex(f, 0)
}

func cNorm(c complex128) complex128 {
	return c / complex(cmplx.Abs(c), 0)
}

func cMul(c complex128, f float64) complex128 {
	return c * complex(f, 0)
}

func cRand(radius float64) complex128 {
	return cmplx.Rect(radius, (rand.Float64()*math.Pi*2)-math.Pi)
}

func vicinity(pos complex128, radius float64) [][2]int {
	px, py := parts(pos)

	xi, yi := int(math.Round(px)), int(math.Round(py))
	result := make([][2]int, 0, int(4*(radius+1)*(radius+1)))

	for y := yi - int(radius); y < yi+int(radius+1); y++ {
		for x := xi - int(radius); x < xi+int(radius+1); x++ {
			if cmplx.Abs(pos-complex(float64(x), float64(y))) < radius {
				result = append(result, [2]int{y, x})
			}
		}
	}
	return result
}
