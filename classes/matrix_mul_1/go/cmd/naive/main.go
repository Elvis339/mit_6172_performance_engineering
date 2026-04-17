package main

import (
	"fmt"
	"math/rand"
	"runtime"
	"sync"
	"time"
)

const N = 4096

type matrix = [][]float64

func initializeSlices() (matrix, matrix, matrix) {
	a := make([][]float64, N)
	b := make([][]float64, N)
	c := make([][]float64, N)

	for i := range N {
		a[i] = make([]float64, N)
		b[i] = make([]float64, N)
		c[i] = make([]float64, N)
		for j := range N {
			a[i][j] = rand.Float64()
			b[i][j] = rand.Float64()

		}
	}

	return a, b, c
}

func matmul(a, b, c matrix) {
	numWorkers := runtime.GOMAXPROCS(0)
	var wg sync.WaitGroup
	wg.Add(numWorkers)

	for w := range numWorkers {
		go func(workerID int) {
			defer wg.Done()
			for i := workerID; i < N; i += numWorkers {
				for j := range N {
					for k := range N {
						c[i][j] += a[i][k] * b[k][j]
					}
				}
			}
		}(w)
	}

	wg.Wait()
}

func main() {
	a, b, c := initializeSlices()

	start := time.Now()
	matmul(a, b, c)
	fmt.Printf("matmul done in %v\n", time.Since(start))
	fmt.Printf("c[0][0] = %f\n", c[0][0])
}
