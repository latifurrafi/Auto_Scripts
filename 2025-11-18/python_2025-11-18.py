```python
import random
import time

def typing_speed_test():
  """
  This script tests your typing speed and accuracy using a random quote.
  It's a fun way to learn about strings, time, and input in Python.
  """

  quotes = [
    "The quick brown fox jumps over the lazy dog.",
    "Programming is the art of telling computers what to do.",
    "Coding is my superpower.",
    "Practice makes perfect.",
    "Never give up on your dreams."
  ]

  quote = random.choice(quotes)
  print("Type the following quote as fast and accurately as possible:\n")
  print(quote)

  input("\nPress Enter to start...")  # Wait for the user to get ready
  start_time = time.time()

  user_input = input("\nStart typing: ")

  end_time = time.time()
  time_taken = end_time - start_time

  # Calculate words per minute (WPM)
  word_count = len(quote.split())
  wpm = (word_count / time_taken) * 60

  # Calculate accuracy
  correct_chars = sum(1 for a, b in zip(quote, user_input) if a == b)
  accuracy = (correct_chars / len(quote)) * 100

  print("\n--- Results ---")
  print(f"Time taken: {time_taken:.2f} seconds")
  print(f"Words per minute (WPM): {wpm:.2f}")
  print(f"Accuracy: {accuracy:.2f}%")

  if accuracy < 90:
    print("\nKeep practicing for better accuracy!")
  elif wpm < 30:
    print("\nYou can type faster! Keep practicing!")
  else:
    print("\nGreat job! You're a typing wizard!")

if __name__ == "__main__":
  typing_speed_test()
```

**How it teaches programming concepts:**

*   **Strings:**  Uses strings for quotes and user input, demonstrating string manipulation.
*   **Randomness:**  Uses the `random` module to select a random quote.
*   **Time:**  Uses the `time` module to measure the time taken for typing.  This introduces the concept of tracking time in programs.
*   **Input/Output:** Uses `input()` to get user input and `print()` to display information.
*   **Loops (Implicit):** The `zip` function combined with `sum` provides a concise way to iterate through two strings and count matching characters.
*   **Conditional Statements (if/else):**  Uses `if/else` statements to provide feedback based on typing speed and accuracy.
*   **Functions:**  Organizes the code into a `typing_speed_test()` function, promoting modularity and reusability.
*   **String Formatting:** Uses f-strings for clear output with formatted numbers (e.g., `:.2f` for two decimal places).

**How it's fun:**

*   **Interactive:**  The user actively participates in the test.
*   **Measurable Results:** Provides concrete metrics (WPM, accuracy) that users can try to improve.
*   **Encouraging Feedback:** Gives positive reinforcement for good performance and motivates improvement.
*   **Real-World Application:**  Tests a skill that is useful in many contexts.

**How to run it:**

1.  Save the code as a `.py` file (e.g., `typing_test.py`).
2.  Open a terminal or command prompt.
3.  Navigate to the directory where you saved the file.
4.  Run the script using `python typing_test.py`.

The script will then guide you through the typing test.  Enjoy!