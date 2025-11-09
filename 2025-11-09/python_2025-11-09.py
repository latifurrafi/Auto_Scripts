```python
import random
import time

def typing_test():
  """
  A simple typing test game that introduces string manipulation, 
  time tracking, and basic game logic.
  """

  sentences = [
      "The quick brown fox jumps over the lazy dog.",
      "Programming is fun and challenging.",
      "Practice makes perfect, keep coding!",
      "Never give up on your dreams.",
      "Innovation drives the future."
  ]

  sentence = random.choice(sentences)
  print("\nType the following sentence:")
  print(sentence)

  input("Press Enter to start...")

  start_time = time.time()
  user_input = input("Your input: ")
  end_time = time.time()

  time_taken = end_time - start_time

  #Calculate accuracy
  correct_chars = 0
  for i in range(min(len(sentence), len(user_input))):
      if sentence[i] == user_input[i]:
          correct_chars += 1

  accuracy = (correct_chars / len(sentence)) * 100

  #Calculate WPM (Words Per Minute) - approximate
  words = len(sentence.split())
  wpm = (words / time_taken) * 60  # Crude approximation without error correction


  print("\nTime taken: {:.2f} seconds".format(time_taken))
  print("Accuracy: {:.2f}%".format(accuracy))
  print("Approximate WPM: {:.2f}".format(wpm))

  if user_input == sentence:
      print("Perfect!")
  else:
      print("Keep practicing!")

if __name__ == "__main__":
  print("Welcome to the Typing Speed Test!")
  typing_test()
```

**How it Works and Teaches Concepts:**

1.  **String Manipulation:** The script uses strings (`sentence`, `user_input`) and performs operations like `.split()` to count words. It also iterates through characters in strings to calculate accuracy.

2.  **Random Number Generation:** `random.choice(sentences)` selects a random sentence for the user, introducing the concept of randomness.

3.  **Time Tracking:** `time.time()` is used to measure the time taken for the typing test, demonstrating how to track elapsed time in Python.

4.  **Input/Output:** `input()` prompts the user for input, and `print()` displays information.

5.  **Basic Game Logic:** The script compares the user's input with the original sentence and calculates accuracy and WPM, providing feedback to the user. This is a simple example of game development logic.

6.  **Conditional Statements:**  `if user_input == sentence:`  demonstrates conditional logic based on comparing the strings to evaluate if the entry was perfect.

**How to Run the Script:**

1.  Save the code as a `.py` file (e.g., `typing_test.py`).
2.  Open a terminal or command prompt.
3.  Navigate to the directory where you saved the file.
4.  Run the script using `python typing_test.py`.

The script will then guide you through the typing test.

**Improvements/Extensions:**

*   **Error Correction:** Implement a more sophisticated WPM calculation that accounts for errors.
*   **Different Difficulty Levels:** Add sentence banks with varying difficulty.
*   **User Interface:** Use a GUI library (like Tkinter or PyQt) for a more visually appealing interface.
*   **Profile Tracking:** Save user scores and track progress over time.
*   **Word Highlighting:** Highlight the word the user is currently typing.