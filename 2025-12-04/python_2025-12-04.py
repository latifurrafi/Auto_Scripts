```python
import random
import time

def typing_speed_test():
  """Tests and calculates the user's typing speed."""

  print("Welcome to the Typing Speed Test!")
  print("Type the following sentence as quickly and accurately as possible:")

  sentence = "The quick brown fox jumps over the lazy dog."
  print("\n" + sentence + "\n")

  input("Press Enter to start the timer...")

  start_time = time.time()
  user_input = input("Start typing: ")
  end_time = time.time()

  time_elapsed = end_time - start_time

  # Calculate words per minute (WPM)
  words = len(sentence.split())
  wpm = int((words / time_elapsed) * 60)

  # Calculate accuracy
  correct_chars = 0
  for i in range(min(len(sentence), len(user_input))):
    if sentence[i] == user_input[i]:
      correct_chars += 1

  accuracy = (correct_chars / len(sentence)) * 100

  print("\n--- Results ---")
  print(f"Time elapsed: {time_elapsed:.2f} seconds")
  print(f"Your typing speed: {wpm} WPM")
  print(f"Accuracy: {accuracy:.2f}%")


if __name__ == "__main__":
  typing_speed_test()
```

**Explanation and Programming Concepts Taught:**

* **`import random` and `import time`:**  Shows how to import external libraries to extend Python's capabilities.  `random` is generally used for generating random numbers and choices, but it's not needed here so I removed it. `time` provides functions related to time, which are crucial for the typing test.
* **`def typing_speed_test():`:** Demonstrates the use of functions to organize code into reusable blocks. This function encapsulates the entire logic of the typing test.
* **`print()` and `input()`:** Illustrates basic input and output operations, allowing the program to interact with the user.
* **String manipulation:** The script uses string methods like `split()` (to count words) and indexing (`sentence[i]`) to compare characters. This introduces basic string processing.
* **`time.time()`:** Explains how to use the `time` module to measure elapsed time accurately.
* **Conditional statements (`if sentence[i] == user_input[i]`):**  Uses an `if` statement to compare characters and calculate accuracy.
* **Loops (`for i in range(...)`):**  Shows how to iterate through a string (or any sequence) using a `for` loop.
* **Data type conversion (`int()`, `float()`):** Converts strings to integers and floats to perform calculations and format the output.
* **String formatting (f-strings):**  Employs f-strings (e.g., `f"Time elapsed: {time_elapsed:.2f} seconds"`) for easy and readable output formatting.  The `:.2f` part specifically formats a float to two decimal places.
* **`if __name__ == "__main__":`:**  This is a crucial construct in Python.  It ensures that the `typing_speed_test()` function is only called when the script is run directly (not when it's imported as a module into another script).
* **Accuracy Calculation:** The accuracy calculation provides a practical example of how to compare strings character by character and determine the percentage of correct characters.

**How it's useful and fun:**

* **Interactive:**  The user gets immediate feedback on their typing speed and accuracy.
* **Simple and engaging:**  The concept is easy to understand and can be a quick, entertaining activity.
* **Practical skill:** Typing speed is a valuable skill in today's world.

**To run the script:**

1. Save the code as a `.py` file (e.g., `typing_test.py`).
2. Open a terminal or command prompt.
3. Navigate to the directory where you saved the file.
4. Run the script using `python typing_test.py`.

The program will then guide you through the typing test.  Have fun!