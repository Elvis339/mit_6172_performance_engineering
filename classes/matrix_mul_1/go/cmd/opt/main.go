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

func initializeSlices() (*[N][N]float64, *[N][N]float64, *[N][N]float64) {
	a := new([N][N]float64)
	b := new([N][N]float64)
	c := new([N][N]float64)

	for i := range N {
		for j := range N {
			a[i][j] = rand.Float64()
			b[i][j] = rand.Float64()
			c[i][j] = 0.0
		}
	}

	return a, b, c
}

// processTile multiplies a block/tile of the matrices
func processTile(a, b, c *[N][N]float64, startI, startJ, startK, endI, endJ, endK int) {
	_ = a[startI][endI-1]
	_ = b[startK][endK-1]
	_ = c[startJ][endJ-1]

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

	// fan-out
	for iBlock := 0; iBlock < nBlocks; iBlock++ {
		for kBlock := 0; kBlock < nBlocks; kBlock++ {
			for jBlock := 0; jBlock < nBlocks; jBlock++ {
				workChan <- workItem{iBlock, jBlock, kBlock}
			}
		}
	}

	close(workChan)
	start := time.Now()
	wg.Wait()
	end := time.Since(start)
	fmt.Printf("Tiled multiplication took: %.2f seconds\n", end.Seconds())
	fmt.Printf("c[0][0] = %f\n", c[0][0])
}
