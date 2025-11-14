```python
import random
import time

def typing_test():
  """
  A fun typing test game that reinforces string manipulation, loops, and time management.
  """

  sentences = [
      "The quick brown fox jumps over the lazy dog.",
      "Programming is a superpower with great responsibility.",
      "Never underestimate the power of a well-placed semicolon.",
      "Coding can be challenging, but it's also incredibly rewarding.",
      "Practice makes perfect; keep coding!"
  ]

  sentence = random.choice(sentences)
  print("\nType the following sentence:")
  print(f"\033[1m{sentence}\033[0m")  # Bold the sentence for emphasis

  start_time = time.time()
  user_input = input("\nYour attempt: ")
  end_time = time.time()

  time_taken = end_time - start_time

  correct_chars = 0
  for i in range(min(len(sentence), len(user_input))):
      if sentence[i] == user_input[i]:
          correct_chars += 1

  accuracy = (correct_chars / len(sentence)) * 100

  words = sentence.split()
  wpm = len(words) / (time_taken / 60)

  print("\n--- Results ---")
  print(f"Time taken: {time_taken:.2f} seconds")
  print(f"Accuracy: {accuracy:.2f}%")
  print(f"Words per minute (WPM): {wpm:.2f}")


if __name__ == "__main__":
  print("Welcome to the Typing Test!")
  typing_test()
```

**How it Teaches (Programming Concepts):**

*   **Strings:** The core of the game relies on string comparison (`sentence[i] == user_input[i]`), length calculation (`len(sentence)`), and splitting strings into words (`sentence.split()`).
*   **Loops:** The `for` loop iterates through the characters of the sentence and user input to calculate accuracy.
*   **Time Management:** The `time` module is used to measure the duration of the typing attempt, demonstrating how to track time in Python.
*   **Functions:**  The code is organized into a function (`typing_test()`), promoting modularity and reusability.
*   **Conditional Statements:**  The `if` statement inside the loop checks for matching characters.
*   **Variables:** The code uses several variables to store and manipulate data, such as `sentence`, `user_input`, `time_taken`, `accuracy`, and `wpm`.

**How it's Fun:**

*   **Game-like:** It's an interactive typing test, providing a simple challenge.
*   **Measurable Progress:**  The results (time, accuracy, WPM) give immediate feedback on typing skills.
*   **Randomization:** The use of `random.choice` ensures that the sentence is different each time.
*   **Visual Emphasis:** The `\033[1m` and `\033[0m` characters add bold formatting to the sentence, making it more prominent.

**How to Run:**

1.  Save the code as a `.py` file (e.g., `typing_test.py`).
2.  Open a terminal or command prompt.
3.  Navigate to the directory where you saved the file.
4.  Run the script using the command: `python typing_test.py`

The script will prompt you to type a sentence, and then it will display your results.  Enjoy!