package main

import (
	"log"
	"math/rand"
	"os"
	// 	"runtime/pprof"
	"strconv"
	"time"
)

const (
	CacheLineSize = 64
	BlockSize     = CacheLineSize / 4
)

func isort(arr []uint32) {
	for cur := 1; cur < len(arr); cur++ {
		val := arr[cur]
		index := cur - 1

		for index >= 0 && arr[index] > val {
			arr[index+1] = arr[index]
			index--
		}

		arr[index+1] = val
	}
}

func isortUnroll(arr []uint32) {
	l := len(arr)
	for cur := 1; cur < l; cur++ {
		val := arr[cur]
		index := cur - 1

		// Unrolled inner loop - move 4 elements at a time
		for index >= 3 && arr[index] > val {
			arr[index+1] = arr[index]
			arr[index] = arr[index-1]
			arr[index-1] = arr[index-2]
			arr[index-2] = arr[index-3]
			index -= 4
		}

		// Clean up remaining elements
		for index >= 0 && arr[index] > val {
			arr[index+1] = arr[index]
			index--
		}

		arr[index+1] = val
	}
}

func sortBlock(arr []uint32, start, end int) {
	// Pre-calculate boundary for unrolled loop to avoid repeated checks
	boundary := start + 3

	for cur := start + 1; cur <= end; cur++ {
		val := arr[cur]
		index := cur - 1

		// Fast path: directly copy if in order
		if arr[index] <= val {
			continue
		}

		// Unrolled loop with minimal bounds checking
		for index >= boundary {
			if arr[index] <= val {
				break
			}
			// Process 4 elements at once without individual checks
			copy4(arr, index)
			index -= 4
		}

		// Clean up remaining elements
		for index >= start && arr[index] > val {
			arr[index+1] = arr[index]
			index--
		}
		arr[index+1] = val
	}
}

//go:inline
func copy4(arr []uint32, index int) {
	arr[index+1] = arr[index]
	arr[index] = arr[index-1]
	arr[index-1] = arr[index-2]
	arr[index-2] = arr[index-3]
}

func isortBlock(arr []uint32) {
	n := len(arr)
	if n <= 1 {
		return
	}

	// First sort within blocks
	for blockStart := 0; blockStart < n; blockStart += BlockSize {
		blockEnd := blockStart + BlockSize - 1
		if blockEnd >= n {
			blockEnd = n - 1
		}
		sortBlock(arr, blockStart, blockEnd)
	}

	for mergePoint := BlockSize; mergePoint < n; mergePoint++ {
		val := arr[mergePoint]
		index := mergePoint - 1

		// Vodoo magic optimizing branch prediction
		for index >= 0 {
			// arr[index] > val
			diff := arr[index] - val

			// mask will be 0xFFFFFFFF if `arr[index] > val`, otherwise 0x00000000
			mask := uint32(-(int32(diff) >> 31))

			// bit-trick: conditionally move arr[index] to arr[index+1]
			arr[index+1] = (arr[index+1] & ^mask) | (arr[index] & mask)

			// decrement index only if the condition is true
			index -= int(mask & 1)

			if mask == 0 {
				break
			}
		}

		arr[index+1] = val
	}
}

func main() {
	args := os.Args[1:]
	argc := len(args)

	if argc == 2 {
		args = append(args, "")
	}

	if argc < 2 {
		log.Println("Usage: isort <size> <iterations> [block, unroll]")
		log.Fatal("Error: wrong number of arguments")
	}

	N, _ := strconv.Atoi(args[0])
	K, _ := strconv.Atoi(args[1])

	flag := args[2]

	data := make([]uint32, N)

	logged := false
	llog := func(hasLogged bool, item string) {
		if !hasLogged {
			log.Printf("Using %s version\n", item)
			logged = true
		}
	}

	// 	cpuProfile, err := os.Create("cpu.prof")
	// 	if err != nil {
	// 		log.Fatal("Could not create CPU profile: ", err)
	// 	}
	// 	pprof.StartCPUProfile(cpuProfile)
	// 	defer pprof.StopCPUProfile()

	for j := 0; j < K; j++ {
		for i := 0; i < N; i++ {
			data[i] = rand.Uint32()
		}

		start := time.Now()
		switch flag {
		case "block":
			llog(logged, "block")
			isortBlock(data)
		case "unroll":
			llog(logged, "unroll")
			isortUnroll(data)
		default:
			llog(logged, "un-optimized")
			isort(data)
		}
		elapsed := time.Since(start)
		log.Printf("elapsed: %s\n", elapsed)
	}
}
