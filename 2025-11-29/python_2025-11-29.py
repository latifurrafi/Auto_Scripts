```python
import random
import time

def typing_speed_test():
  """
  A fun typing speed test that reinforces string manipulation,
  time tracking, and user input.
  """

  sentences = [
      "The quick brown fox jumps over the lazy dog.",
      "Programming is fun and challenging.",
      "Never give up on your dreams.",
      "Practice makes perfect.",
      "Coding is the future."
  ]

  sentence = random.choice(sentences)
  print("Type the following sentence as fast as you can:\n")
  print(sentence + "\n")

  start_time = time.time()
  user_input = input("Your input: ")
  end_time = time.time()

  time_taken = end_time - start_time

  if user_input == sentence:
    words = sentence.split()
    word_count = len(words)
    words_per_minute = round((word_count / time_taken) * 60)

    print("\nCorrect!")
    print(f"Time taken: {time_taken:.2f} seconds")
    print(f"Your typing speed: {words_per_minute} words per minute")
  else:
    print("\nIncorrect! Try again.")
    print(f"You typed: {user_input}")

if __name__ == "__main__":
  print("Welcome to the Typing Speed Test!")
  typing_speed_test()
```

**Explanation:**

1. **Concept Reinforced:**
   - **String manipulation:**  Splitting the sentence into words using `sentence.split()`.
   - **Time tracking:** Using `time.time()` to measure the time taken.
   - **User input:**  Using `input()` to get the user's typed sentence.
   - **Conditional logic:** Using `if/else` to check if the user's input matches the original sentence.

2. **How it Works:**
   - **Sentences:** A list of sentences is defined.
   - **Random Selection:** A random sentence is chosen for the user to type.
   - **Input and Timing:** The script records the time before and after the user types the sentence.
   - **Accuracy Check:** It compares the user's input to the original sentence.
   - **Speed Calculation:**  If correct, it calculates words per minute (WPM).
   - **Output:** It displays the time taken, accuracy, and WPM.

3. **Why it's Useful and Fun:**
   - **Practice:** It provides a fun way to practice typing skills.
   - **Measurement:** It gives you a measure of your typing speed.
   - **Simple and Engaging:**  The code is easy to understand and the test is quick and engaging.

**To run the script:**

1. Save it as a `.py` file (e.g., `typing_test.py`).
2. Open a terminal or command prompt.
3. Navigate to the directory where you saved the file.
4. Run the script using `python typing_test.py`.

You'll be prompted to type the sentence, and the script will calculate your speed and accuracy.  You can extend it further by adding features like difficulty levels (shorter/longer sentences), tracking history, etc.