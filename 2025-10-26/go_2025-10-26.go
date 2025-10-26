```go
package main

import (
	"fmt"
	"math/rand"
	"time"
)

// weightedChoice implements a probability-based choice from a set of options.
// Each option has an associated weight.
func weightedChoice[T any](options []T, weights []int) (T, error) {
	if len(options) != len(weights) {
		var zero T
		return zero, fmt.Errorf("options and weights slices must have the same length")
	}

	totalWeight := 0
	for _, weight := range weights {
		totalWeight += weight
	}

	if totalWeight <= 0 {
		var zero T
		return zero, fmt.Errorf("total weight must be positive")
	}

	randNum := rand.Intn(totalWeight)

	currentWeight := 0
	for i, weight := range weights {
		currentWeight += weight
		if randNum < currentWeight {
			return options[i], nil
		}
	}

	// Should never reach here if weights are correctly configured
	var zero T
	return zero, fmt.Errorf("internal error: failed to select an option")
}

func main() {
	rand.Seed(time.Now().UnixNano()) // Seed the random number generator

	emojis := []string{"😄", "😊", "😍", "🤔", "😭"}
	weights := []int{5, 3, 7, 1, 2} // Weights corresponding to the emojis

	for i := 0; i < 10; i++ {
		emoji, err := weightedChoice(emojis, weights)
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		fmt.Println(emoji)
	}
}
```

**Explanation and Innovation:**

1. **`weightedChoice` Function:** This is the core of the program. It demonstrates the concept of *weighted random choice*. Instead of selecting an option with uniform probability, each option has a weight associated with it.  Options with higher weights are more likely to be selected. This is a very common technique in game development (e.g., loot drops), simulations, and machine learning (e.g., sampling biased datasets).

2. **Generics:** The `weightedChoice` function uses generics (`[T any]`) which allows it to work with any data type. This makes it reusable across different contexts.  This is a modern feature of Go, enhancing code flexibility and type safety.

3. **Error Handling:** The code includes error handling to ensure that the input parameters are valid (e.g., options and weights slices have the same length, total weight is positive). This makes the function more robust.

4. **Emoji Demo:** The `main` function uses the `weightedChoice` function to select emojis.  The `weights` slice defines the probability distribution for the emojis. For example, `😍` is more likely to be selected than `🤔`. This provides a visually engaging demonstration of the weighted random selection process.

5. **Clear and Concise:** The code is well-structured and easy to understand. It avoids unnecessary complexity and focuses on clearly demonstrating the concept of weighted random choice.

**How it works:**

* The `weightedChoice` function calculates the total weight of all options.
* It generates a random number between 0 and `totalWeight - 1`.
* It iterates through the options and their corresponding weights, accumulating the `currentWeight`.
* When `randNum` becomes less than `currentWeight`, it means that the current option has been selected.

**Why it's innovative (relatively speaking):**

While weighted random choice is a known concept, the innovative aspect here lies in:

* **Combining Generics and a Practical Algorithm:** The Go program cleanly implements this algorithm using Go's generics, creating a reusable and type-safe component.
* **Emoji-Based Demonstration:** The use of emojis makes the output more engaging and immediately understandable.  It helps to visualize the effects of the weights on the selection process.
* **Error Handling:**  The inclusion of robust error handling ensures the function's reliability, a critical aspect of well-designed Go programs.

This program showcases a useful algorithm, implemented with modern Go features and a touch of creativity in the demonstration.  It provides a solid foundation for using weighted random choice in more complex applications.