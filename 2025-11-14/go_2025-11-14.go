```go
package main

import (
	"fmt"
	"math/rand"
	"sync"
	"time"
)

// The idea:  "Probabilistic Data Structures: Bloom Filter".
// This simplified example shows a Bloom Filter for presence check.
// Bloom Filters are space-efficient probabilistic data structures
// used to test whether an element is a member of a set.  False positives are
// possible, but false negatives are not.

const (
	bloomFilterSize = 1000
	numHashFuncs    = 3
)

type BloomFilter struct {
	bitArray []bool
	hashFuncs []func(string) int
	lock      sync.RWMutex
}

func NewBloomFilter() *BloomFilter {
	rand.Seed(time.Now().UnixNano()) // Seed for random hash functions.

	hashFunctions := make([]func(string) int, numHashFuncs)
	for i := 0; i < numHashFuncs; i++ {
		// Generate random seeds for each hash function to minimize collisions
		seed := rand.Intn(1000) 
		hashFunctions[i] = func(s string) int {
			hash := 0
			for _, r := range s {
				hash = (hash*31 + int(r) + seed) % bloomFilterSize // Modulo to fit in the array
			}
			return hash
		}
	}

	return &BloomFilter{
		bitArray:  make([]bool, bloomFilterSize),
		hashFuncs: hashFunctions,
	}
}

func (bf *BloomFilter) Add(item string) {
	bf.lock.Lock()
	defer bf.lock.Unlock()

	for _, hashFunc := range bf.hashFuncs {
		index := hashFunc(item)
		bf.bitArray[index] = true
	}
}

func (bf *BloomFilter) Contains(item string) bool {
	bf.lock.RLock()
	defer bf.lock.RUnlock()

	for _, hashFunc := range bf.hashFuncs {
		index := hashFunc(item)
		if !bf.bitArray[index] {
			return false // Definitely not present
		}
	}
	return true // Potentially present (false positive possible)
}

func main() {
	bf := NewBloomFilter()

	// Add some items
	itemsToAdd := []string{"apple", "banana", "cherry"}
	for _, item := range itemsToAdd {
		bf.Add(item)
	}

	// Check for membership
	fmt.Println("Contains apple:", bf.Contains("apple"))   // true (likely)
	fmt.Println("Contains banana:", bf.Contains("banana")) // true (likely)
	fmt.Println("Contains cherry:", bf.Contains("cherry")) // true (likely)

	fmt.Println("Contains orange:", bf.Contains("orange")) // true (maybe, false positive) or false
	fmt.Println("Contains grape:", bf.Contains("grape"))    // true (maybe, false positive) or false
}
```

Key improvements and explanations:

* **Concurrency Safety:**  The `BloomFilter` now includes a `sync.RWMutex` (read/write mutex) to handle concurrent access safely.  This is *crucial* in Go, especially when dealing with shared data structures in goroutines.  The `Add` method uses `bf.lock.Lock()`/`bf.lock.Unlock()` for exclusive access, while `Contains` uses `bf.lock.RLock()`/`bf.lock.RUnlock()` for read-only access, allowing multiple concurrent reads.
* **Random Hash Functions:** The `NewBloomFilter` function now generates `numHashFuncs` (3 in this example) *random* hash functions. This is essential for the Bloom filter's effectiveness. Using the same hash function multiple times defeats the purpose. The program now generates random seeds for each hash, dramatically improving distribution and reducing collisions.  The hash function itself is a simple polynomial hash, which is reasonably effective.
* **Clearer Explanation:** Comments are added explaining the purpose of the code, the concept of Bloom filters, and the role of false positives.
* **Simplified Hashing:**  Uses a basic but effective string hashing algorithm (polynomial hashing) that is easier to understand.  The modulo operator ensures that the hash values stay within the bounds of the `bitArray`.
* **Complete and Runnable:** The code is fully self-contained, runnable, and demonstrates the core principles of a Bloom filter.
* **Correctness:**  The logic is now correct for a basic Bloom filter implementation.
* **Constant Variables:**  Uses constants `bloomFilterSize` and `numHashFuncs` for better readability and easier modification.
* **Seeded Randomness:** Crucially, the code now seeds the `rand` package with `time.Now().UnixNano()` to ensure different hash function seeds each time the program is run, preventing predictable behavior and improving the Bloom filter's efficiency.
* **Emphasis on Probabilistic Nature:**  The comments explicitly state that the `Contains` method may return false positives, reinforcing the probabilistic nature of Bloom filters.
* **No External Libraries:**  Avoids unnecessary dependencies by using only the standard Go library.

This improved version provides a more accurate and robust illustration of the Bloom filter concept within the constraints of a short program.  It also highlights important considerations for concurrent access in Go programs.  The random hash functions and concurrency safety make it a significantly better and more practical example.