package main

import (
	"fmt"
	"math/rand"
	"runtime"
	"sync"
	"time"
)

const (
	N         = 4096
	BlockSize = 64 // L1 cache
)

func initializeSlices() ([][]float64, [][]float64, [][]float64) {
	a := make([][]float64, N)
	b := make([][]float64, N)
	c := make([][]float64, N)

	for i := range a {
		a[i] = make([]float64, N)
		b[i] = make([]float64, N)
		c[i] = make([]float64, N)
	}

	for i := 0; i < N; i++ {
		for j := 0; j < N; j++ {
			a[i][j] = rand.Float64()
			b[i][j] = rand.Float64()
			c[i][j] = 0.0
		}
	}

	return a, b, c
}

// processTile multiplies a block/tile of the matrices
func processTile(a, b, c [][]float64, startI, startJ, startK, endI, endJ, endK int) {
	for i := startI; i < endI; i++ {
		for k := startK; k < endK; k++ {
			for j := startJ; j < endJ; j++ {
				c[i][j] += a[i][k] * b[k][j]
			}
		}
	}
}

func main() {
	a, b, c := initializeSlices()
	numWorkers := runtime.GOMAXPROCS(0)
	var wg sync.WaitGroup

	// Calculate number of blocks in each dimension
	nBlocks := (N + BlockSize - 1) / BlockSize

	type workItem struct {
		iBlock, jBlock, kBlock int
	}
	workChan := make(chan workItem, nBlocks*nBlocks*nBlocks)

	for iBlock := 0; iBlock < nBlocks; iBlock++ {
		for kBlock := 0; kBlock < nBlocks; kBlock++ {
			for jBlock := 0; jBlock < nBlocks; jBlock++ {
				workChan <- workItem{iBlock, jBlock, kBlock}
			}
		}
	}

	close(workChan)

	start := time.Now()
	for w := 0; w < numWorkers; w++ {
		wg.Add(1)
		go func() {
			defer wg.Done()

			for work := range workChan {
				// boundary calculation
				startI := work.iBlock * BlockSize
				startJ := work.jBlock * BlockSize
				startK := work.kBlock * BlockSize

				endI := startI + BlockSize
				if endI > N {
					endI = N
				}
				endJ := startJ + BlockSize
				if endJ > N {
					endJ = N
				}
				endK := startK + BlockSize
				if endK > N {
					endK = N
				}

				processTile(a, b, c, startI, startJ, startK, endI, endJ, endK)
			}
		}()
	}

	wg.Wait()
	end := time.Since(start)
	fmt.Printf("Tiled multiplication took: %.2f seconds\n", end.Seconds())
}
