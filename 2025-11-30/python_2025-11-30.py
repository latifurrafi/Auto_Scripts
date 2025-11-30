```python
import random
import time

def typing_speed_test():
    """Tests the user's typing speed and accuracy with a random quote."""

    quotes = [
        "The quick brown fox jumps over the lazy dog.",
        "Programming is like playing chess: one wrong move can ruin the whole game.",
        "The best way to predict the future is to create it.",
        "Write code that is easy to understand, not clever.",
        "Simplicity is the soul of efficiency."
    ]

    quote = random.choice(quotes)

    print("Type the following quote as quickly and accurately as possible:")
    print("-" * 50)
    print(quote)
    print("-" * 50)

    start_time = time.time()
    user_input = input("\nStart Typing: ")
    end_time = time.time()

    time_taken = end_time - start_time
    words_typed = len(user_input.split())  # Count words based on spaces
    characters_typed = len(user_input)

    # Calculate words per minute (WPM)
    wpm = int((words_typed / time_taken) * 60)

    # Calculate accuracy (simple character-based comparison)
    correct_chars = sum(a == b for a, b in zip(quote, user_input))
    accuracy = (correct_chars / len(quote)) * 100

    print("\n--- Results ---")
    print(f"Time taken: {time_taken:.2f} seconds")
    print(f"Words per minute (WPM): {wpm}")
    print(f"Accuracy: {accuracy:.2f}%")

    # Learning moment: Demonstrate conditional statements (if/else)
    if accuracy > 90:
        print("\nGreat job! Your typing skills are excellent.")
    elif accuracy > 75:
        print("\nNot bad! Keep practicing to improve your accuracy and speed.")
    else:
        print("\nPractice makes perfect! Focus on accuracy and then gradually increase speed.")


if __name__ == "__main__":
    print("Welcome to the Typing Speed Test!\n")
    typing_speed_test()
```

**How it works and the concept it teaches:**

1. **`import random` and `import time`:**  Imports necessary libraries. `random` is used to select a random quote.  `time` is used to measure the duration of the typing test.

2. **`typing_speed_test()` function:** Contains the core logic.
   - **Quotes List:** A list of strings containing different quotes.
   - **Random Quote Selection:**  `random.choice(quotes)` picks a random quote from the list.
   - **Input and Timing:**
     - `start_time = time.time()` records the time *before* the user starts typing.
     - `user_input = input("\nStart Typing: ")` prompts the user to type the quote.
     - `end_time = time.time()` records the time *after* the user finishes typing.
   - **Calculations:**
     - `time_taken` is the difference between `end_time` and `start_time`.
     - `words_typed` counts the number of words in the user's input (using `split()` which splits a string at each space).
     - `characters_typed` counts the total number of characters typed.
     - `wpm` (words per minute) is calculated.
     - `accuracy` is calculated by comparing the characters in the quote and the user's input.
   - **Results Display:**  The script prints the time taken, WPM, and accuracy.
   - **Conditional Statements (if/elif/else):** *This is the main teaching point.* The script uses `if`, `elif`, and `else` statements to provide different feedback messages based on the user's accuracy.  This demonstrates how to execute different blocks of code depending on a condition.

3. **`if __name__ == "__main__":`**:  This is a standard Python idiom. It ensures that the `typing_speed_test()` function is only called when the script is run directly (not when it's imported as a module into another script).

**Learning Concept Demonstrated: Conditional Statements (if/elif/else)**

This script effectively teaches the use of `if`, `elif`, and `else` statements to control the flow of execution in a program.  Based on the calculated accuracy of the typing test, the script prints different feedback messages, demonstrating how conditional logic can be used to create dynamic and responsive programs.

**How to Run:**

1. Save the code as a `.py` file (e.g., `typing_test.py`).
2. Open a terminal or command prompt.
3. Navigate to the directory where you saved the file.
4. Run the script using the command: `python typing_test.py`

The script will then prompt you to type the quote, and it will calculate and display your typing speed and accuracy.