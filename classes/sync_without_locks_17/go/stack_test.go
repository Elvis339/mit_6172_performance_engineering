package main

import (
	"fmt"
	"math/rand"
	"sync/atomic"
	"testing"
	"unsafe"
)

type node struct {
	next *node
	data uint32
}

func (n *node) String() string {
	return fmt.Sprintf("Node{data: %d, next: %p}", n.data, n.next)
}

// stack: Compare-and-swap acquires a cache line in exclusive
// mode, invalidating the cache line in other caches.
// Result: High contention if all processors are doing
// CAS’s to same cache line.
type stackImpl interface {
	push(n *node)
	pop() *node
}

type optimizedStack struct {
	head *node
}

type nonOptimizedStack struct {
	head *node
}

func newStack(optimized bool) stackImpl {
	if optimized {
		return &optimizedStack{}
	}
	return &nonOptimizedStack{}
}

func (s *optimizedStack) push(n *node) {
	for {
		head := atomic.LoadPointer((*unsafe.Pointer)(unsafe.Pointer(&s.head)))
		n.next = (*node)(head)
		casResult := atomic.CompareAndSwapPointer(
			(*unsafe.Pointer)(unsafe.Pointer(&s.head)),
			head,
			unsafe.Pointer(n))

		if uintptr(head)^uintptr(unsafe.Pointer(n.next)) != 0 || casResult {
			return
		}
	}
}

func (s *optimizedStack) pop() *node {
	for {
		current := atomic.LoadPointer((*unsafe.Pointer)(unsafe.Pointer(&s.head)))
		if current == nil {
			return nil
		}
		currentNode := (*node)(current)

		if current == atomic.LoadPointer((*unsafe.Pointer)(unsafe.Pointer(&s.head))) &&
			atomic.CompareAndSwapPointer(
				(*unsafe.Pointer)(unsafe.Pointer(&s.head)),
				current,
				unsafe.Pointer(currentNode.next),
			) {
			return currentNode
		}
		continue
	}
}

func (s *nonOptimizedStack) push(n *node) {
	for {
		head := atomic.LoadPointer((*unsafe.Pointer)(unsafe.Pointer(&s.head)))
		n.next = (*node)(head)
		if atomic.CompareAndSwapPointer(
			(*unsafe.Pointer)(unsafe.Pointer(&s.head)),
			head,
			unsafe.Pointer(n),
		) {
			return
		}
	}
}

func (s *nonOptimizedStack) pop() *node {
	for {
		current := atomic.LoadPointer((*unsafe.Pointer)(unsafe.Pointer(&s.head)))
		if current == nil {
			return nil
		}
		currentNode := (*node)(current)
		if atomic.CompareAndSwapPointer(
			(*unsafe.Pointer)(unsafe.Pointer(&s.head)),
			current,
			unsafe.Pointer(currentNode.next),
		) {
			return currentNode
		}
	}
}

func BenchmarkStack(b *testing.B) {
	benchmarks := []struct {
		name      string
		optimized bool
		threads   int
	}{
		{"Optimized_LowContention_1Thread", true, 1},
		{"NonOptimized_LowContention_1Thread", false, 1},
		{"Optimized_HighContention_4Threads", true, 4},
		{"NonOptimized_HighContention_4Threads", false, 4},
	}

	for _, bm := range benchmarks {
		b.Run(bm.name, func(b *testing.B) {
			s := newStack(bm.optimized)
			b.ResetTimer()

			b.RunParallel(func(pb *testing.PB) {
				for pb.Next() {
					n := &node{data: rand.Uint32()}
					s.push(n)
					s.pop()
				}
			})
		})
	}
}
