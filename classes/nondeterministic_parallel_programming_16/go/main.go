package main

import (
	"fmt"
	"sync"
	"time"
)

// Golang Mutex Mechanism
// Spinning Phase: When a goroutine tries to acquire a lock and the mutex is already locked, it initially spins for a short duration. Spinning avoids the cost of a context switch.
// Yielding Phase: If the mutex remains locked after spinning, the goroutine yields control, and the runtime parks it. The goroutine is then added to a wait queue. This phase relies on the operating system's scheduler to manage the goroutine and wake it up when the lock becomes available.
// The spinning duration is limited, so a thread does not waste too many processor cycles.

// Fairness
// Go's mutex is not strictly FIFO.
// While the sync.Mutex implementation tries to be fair, fairness isn't guaranteed.
// Goroutines waiting for the lock are queued, and when the lock becomes available, the runtime wakes up a goroutine from the queue, but the exact order depends on internal scheduling.
func main() {
	mu := sync.Mutex{}

	wg := sync.WaitGroup{}
	wg.Add(3)

	// all goroutines attempt to acquire the lock, leading to contention
	for i := 0; i < 3; i++ {
		go func(id int) {
			start := time.Now()
			mu.Lock()
			defer mu.Unlock()
			fmt.Printf("Goroutine %d acquired lock after %v\n", id, time.Since(start))
			time.Sleep(500 * time.Millisecond) // Simulate work
			wg.Done()
		}(i)
	}

	wg.Wait()
}
