```python
import random
import time

def psychic_number_guesser():
  """
  A fun little script that tries to guess a number you're thinking of,
  demonstrating binary search and how to take user input.
  """

  print("Welcome to the Psychic Number Guesser!")
  print("Think of a number between 1 and 100 (inclusive).  Don't tell me!")
  time.sleep(2)  # Dramatic pause!

  low = 1
  high = 100
  guesses = 0

  while low <= high:
    guesses += 1
    guess = (low + high) // 2  # Integer division for the midpoint

    print(f"\nIs your number {guess}?")
    answer = input("Enter 'higher', 'lower', or 'yes': ").lower()

    if answer == 'yes':
      print(f"Ha! I guessed it in {guesses} tries!")
      return

    elif answer == 'higher':
      low = guess + 1
    elif answer == 'lower':
      high = guess - 1
    else:
      print("Invalid input. Please enter 'higher', 'lower', or 'yes'.")

  print("You tricked me!  Or maybe you didn't follow the rules...")


if __name__ == "__main__":
  psychic_number_guesser()
```

**Explanation and Programming Concepts Demonstrated:**

* **Binary Search:** The core of the script is the binary search algorithm.  Instead of guessing numbers randomly, the script systematically narrows down the range of possible numbers by repeatedly dividing the search interval in half.  This is a very efficient search technique for sorted data (or, in this case, a range of numbers).

* **User Input (`input()`):**  The script uses `input()` to get information from the user (whether their number is higher, lower, or the guess is correct).  This interaction is essential for making the game engaging.

* **Control Flow (`while` loop, `if`/`elif`/`else`):** The `while` loop continues until the correct number is guessed.  The `if`/`elif`/`else` statements determine how to update the `low` and `high` bounds based on the user's input.  This demonstrates fundamental control flow mechanisms.

* **Integer Division (`//`):** The `//` operator performs integer division, ensuring that the `guess` is always a whole number.  This is crucial because we're dealing with a range of integers.

* **String Manipulation (`.lower()`):**  The `.lower()` method converts the user's input to lowercase. This makes the script more robust, as it will accept "Higher", "HIGHER", "higher", etc.

* **Function Definition (`def`):** The code is organized into a function `psychic_number_guesser()`. This promotes code reusability and makes the script more readable.

* **`if __name__ == "__main__":`:** This ensures that the `psychic_number_guesser()` function is only called when the script is run directly (not when it's imported as a module into another script).

**How to Run:**

1.  Save the code as a Python file (e.g., `guesser.py`).
2.  Open a terminal or command prompt.
3.  Navigate to the directory where you saved the file.
4.  Run the script using `python guesser.py`.

**Why it's Creative and Useful:**

*   **Fun and Engaging:** It's a playful way to demonstrate a powerful algorithm.
*   **Interactive:**  The user interaction makes it more memorable.
*   **Illustrative:**  It clearly shows how binary search works in practice.
*   **Educational:** It teaches fundamental programming concepts.  A beginner could easily understand the logic and modify the script.
*   **Practical (in a limited way):** While not a real-world problem solver, it illustrates the basic principle behind search algorithms, which are used everywhere.

This script is a good starting point for learning more about binary search, user input, and control flow in Python.  You could extend it by:

*   Adding error handling for invalid number ranges.
*   Tracking the time taken to guess the number.
*   Increasing the range of numbers.
*   Letting the computer *think* for a random amount of time before each guess to make it seem more "psychic".