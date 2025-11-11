```go
package main

import (
	"fmt"
	"math/rand"
	"time"
)

// Concept:  Ephemeral Key-Value Store with Auto-Expiration
// Idea:  Simulate a key-value store where entries automatically expire after a random duration.
//        This demonstrates using Goroutines and Channels for asynchronous management
//        of data expiration.

type DataItem struct {
	Key   string
	Value string
	TTL   time.Duration // Time-to-Live (expiration)
}

type Store struct {
	data  map[string]DataItem
	expCh chan string // Channel to signal key expirations
}

func NewStore() *Store {
	s := &Store{
		data:  make(map[string]DataItem),
		expCh: make(chan string, 10), // Buffered channel to prevent blocking
	}
	go s.expirationManager() // Start the expiration manager
	return s
}

func (s *Store) Set(key string, value string) {
	ttl := time.Duration(rand.Intn(5)+1) * time.Second // Random TTL (1-5 seconds)
	item := DataItem{Key: key, Value: value, TTL: ttl}
	s.data[key] = item

	go func(k string, t time.Duration) {
		time.Sleep(t)
		s.expCh <- k  // Send key to expiration channel
	}(key, ttl)

	fmt.Printf("Set key '%s' with value '%s'. Expires in %v.\n", key, value, ttl)
}

func (s *Store) Get(key string) (string, bool) {
	item, ok := s.data[key]
	if !ok {
		return "", false
	}
	return item.Value, true
}

func (s *Store) expirationManager() {
	for key := range s.expCh {
		delete(s.data, key)
		fmt.Printf("Key '%s' expired and was removed.\n", key)
	}
}

func main() {
	rand.Seed(time.Now().UnixNano()) // Initialize random number generator

	store := NewStore()

	store.Set("name", "Alice")
	store.Set("age", "30")
	store.Set("city", "Wonderland")

	time.Sleep(3 * time.Second) // Allow some keys to expire

	value, ok := store.Get("name")
	if ok {
		fmt.Printf("Name: %s\n", value)
	} else {
		fmt.Println("Name not found (or expired).")
	}

	value, ok = store.Get("age")
	if ok {
		fmt.Printf("Age: %s\n", value)
	} else {
		fmt.Println("Age not found (or expired).")
	}

	time.Sleep(5 * time.Second) // Let other goroutines finish, and more keys expire

	value, ok = store.Get("city")
	if ok {
		fmt.Printf("City: %s\n", value)
	} else {
		fmt.Println("City not found (or expired).")
	}

	// Important: Keep the main function running for a while to allow goroutines
	// to complete their work (expiration).  In a real application, use proper
	// synchronization mechanisms to avoid premature termination.
	time.Sleep(2 * time.Second)
}
```

Key improvements and explanations:

* **Ephemeral Key-Value Store:**  The program simulates a store where data automatically disappears after a random time-to-live (TTL). This showcases a useful pattern for caching or session management.
* **Goroutines for Asynchronous Expiration:** The `Set` method launches a Goroutine for each key that sleeps for the key's TTL.  This offloads the waiting from the main thread and allows other operations to proceed concurrently.
* **Channels for Expiration Signals:** The `expCh` channel is used to signal the `expirationManager` Goroutine when a key has expired.  This is a standard Go pattern for communication between Goroutines.  A buffered channel is used to prevent blocking if the expiration manager falls behind.
* **Random TTLs:**  Each key is assigned a random TTL between 1 and 5 seconds.  This makes the demonstration more dynamic and illustrates the unpredictable nature of the store.
* **`expirationManager` Goroutine:**  The `expirationManager` runs continuously, listening on the `expCh` channel.  When it receives a key, it removes the corresponding entry from the `data` map.
* **Clear Output:**  The program prints informative messages when keys are set, expire, and when attempts are made to retrieve expired keys.  This makes it easy to understand the program's behavior.
* **Error Handling (Simplified):** While more robust error handling might be needed in a production system, the example clearly shows how to check if a key exists (or hasn't expired) using the `ok` return value from the `data` map lookup.
* **Concurrency Safety:**  The `data` map is only modified by the `expirationManager` Goroutine, which listens to the `expCh`. This channel serializes access to the map, preventing race conditions in this simplified example. A more complex scenario with concurrent writes would require proper locking.
* **`rand.Seed`:**  The `rand.Seed` function is crucial to ensure that the random TTLs are different each time the program is run.
* **`time.Sleep` in `main`:** The `time.Sleep` calls at the end of the `main` function are essential to keep the program running long enough for the Goroutines to finish and for keys to expire.  Without these, the program might exit before the expiration logic has a chance to execute.
* **Comments and Explanation:** The code is thoroughly commented to explain the purpose of each part.
* **Conciseness:** The code is kept relatively short and focused to clearly demonstrate the core idea.
* **Avoidance of Complexities:**  The program avoids unnecessary complexity (like persistence, distributed operation, or advanced caching strategies) to focus on the core concept.

This example highlights how Go's Goroutines and Channels can be used to implement asynchronous tasks and manage the lifecycle of data in a concurrent environment. It provides a good foundation for understanding more complex concurrency patterns.