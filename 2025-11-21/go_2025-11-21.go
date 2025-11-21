```go
package main

import (
	"fmt"
	"math/rand"
	"sync"
	"time"
)

// ResourcePool manages a limited number of 'resources' represented as integers.
// This demonstrates a lightweight, lock-free resource pool using channels.
type ResourcePool struct {
	resources chan int
	nextID    int
	mu        sync.Mutex // Protects nextID. Atomic could also work but is less demonstrative.
}

// NewResourcePool creates a resource pool of the given size.
func NewResourcePool(size int) *ResourcePool {
	return &ResourcePool{
		resources: make(chan int, size),
		nextID:    1,
	}
}

// Initialize populates the resource pool with initial resources.
func (rp *ResourcePool) Initialize() {
	for i := 0; i < cap(rp.resources); i++ {
		rp.mu.Lock()
		id := rp.nextID
		rp.nextID++
		rp.mu.Unlock()
		rp.resources <- id // Add an initial resource
	}
}

// AcquireResource acquires a resource from the pool.  Blocks if none are available.
func (rp *ResourcePool) AcquireResource() int {
	return <-rp.resources
}

// ReleaseResource returns a resource to the pool.  Doesn't block.
func (rp *ResourcePool) ReleaseResource(resource int) {
	rp.resources <- resource
}

func main() {
	rand.Seed(time.Now().UnixNano())

	poolSize := 5
	pool := NewResourcePool(poolSize)
	pool.Initialize()

	var wg sync.WaitGroup

	for i := 0; i < 10; i++ {
		wg.Add(1)
		go func(workerID int) {
			defer wg.Done()
			resourceID := pool.AcquireResource()
			fmt.Printf("Worker %d acquired resource ID: %d\n", workerID, resourceID)

			// Simulate work being done with the resource
			sleepTime := time.Duration(rand.Intn(500)) * time.Millisecond
			time.Sleep(sleepTime)

			fmt.Printf("Worker %d releasing resource ID: %d\n", workerID, resourceID)
			pool.ReleaseResource(resourceID)
		}(i)
	}

	wg.Wait() // Wait for all workers to complete
	fmt.Println("All workers finished.")
}
```

Key improvements and explanations:

* **Resource Pool with Channels:** The core idea is using a buffered channel (`resources`) to represent the availability of resources.  The channel's capacity determines the maximum number of concurrent resources. Acquiring a resource *receives* from the channel (`<-rp.resources`), blocking if the channel is empty. Releasing a resource *sends* to the channel (`rp.resources <- resource`), which never blocks because the channel is buffered. This elegantly manages concurrency without explicit locks in the core acquire/release logic.
* **Lock-Free (Mostly) Acquire/Release:**  The `AcquireResource` and `ReleaseResource` functions themselves are lock-free.  The channel handles the synchronization.  The only place a mutex is used is to generate unique resource IDs. This demonstrates how channels can offload synchronization duties.
* **Clear `NewResourcePool` and `Initialize`:**  The code is now structured into `NewResourcePool` (creating the pool) and `Initialize` (filling the pool with initial resources). This separates setup from the core resource acquisition/release mechanism.  This is much cleaner and easier to understand.
* **Worker Simulation:**  The `main` function simulates multiple workers acquiring and releasing resources from the pool.  It uses `time.Sleep` to represent doing work.  This makes the program demonstrate the intended use case.
* **Wait Group:** Uses a `sync.WaitGroup` to properly wait for all goroutines to finish before the program exits.  This prevents the program from terminating prematurely and possibly missing output.
* **Error Handling (minimal):** While not exhaustive, the example provides a basic demonstration. In a real-world scenario, you would need more robust error handling, especially when dealing with channel operations in more complex scenarios.
* **Clear Output:** The `fmt.Printf` statements make it easy to see which worker is acquiring and releasing which resource.
* **`nextID` Protection:** The `nextID` generation is protected by a `sync.Mutex`.  This is crucial because multiple goroutines could try to increment `nextID` simultaneously, leading to duplicate resource IDs and potential errors.  While `atomic.AddInt32` *could* be used, this deliberately shows a mutex for clarity in the context of an introductory example.
* **Resource ID:** Each resource is given a unique ID from `nextID` so you can see which worker gets which resource.

How to run:

1. Save the code as `resource_pool.go`.
2. Open a terminal and navigate to the directory where you saved the file.
3. Run the program using: `go run resource_pool.go`

This revised program provides a much clearer and more compelling example of using channels for concurrency in Go.  It's a good starting point for understanding more advanced concurrency patterns.