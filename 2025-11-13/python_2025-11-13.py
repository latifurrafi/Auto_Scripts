```python
import random

def word_scramble(word):
  """
  Scrambles the letters of a word (except the first and last)
  Demonstrates string slicing, list manipulation, and random number generation.

  Args:
    word: The word to scramble (string).

  Returns:
    The scrambled word (string).
  """

  if len(word) <= 3:
    return word  # No need to scramble short words

  first_letter = word[0]
  last_letter = word[-1]
  middle_letters = list(word[1:-1])  # Convert to list for easy manipulation

  random.shuffle(middle_letters)  # Shuffle the middle letters

  scrambled_word = first_letter + "".join(middle_letters) + last_letter  # Reassemble

  return scrambled_word

def main():
  """
  Asks the user for a sentence and scrambles each word in it.
  Demonstrates string splitting and function usage.
  """
  sentence = input("Enter a sentence to scramble: ")
  words = sentence.split()  # Split the sentence into a list of words

  scrambled_sentence = " ".join([word_scramble(word) for word in words])  # Scramble each word and join them back

  print("Scrambled sentence:", scrambled_sentence)


if __name__ == "__main__":
  main()
```

**How it works and what it teaches:**

* **`word_scramble(word)` function:**
    * **Concept:** String slicing and list manipulation.  It shows how to extract parts of a string using `[start:end]`, convert a string to a list of characters using `list()`, and join a list of characters back into a string using `"".join()`.
    * **Concept:** Random number generation. `random.shuffle()` modifies the list in-place, randomly reordering the elements.
    * **Usefulness:** Implements a basic word scrambling algorithm, which has applications in games or simple text obfuscation.
* **`main()` function:**
    * **Concept:** String splitting and list comprehension.  `sentence.split()` breaks the sentence into words, and a list comprehension `[word_scramble(word) for word in words]` applies the scrambling function to each word in the list.
    * **Concept:** Function calls.  The script demonstrates how to define a function and then call it from another part of the code.
    * **Usefulness:** Combines the word scrambling functionality into a complete program that takes user input and provides output.
* **`if __name__ == "__main__":` block:**
    * **Concept:**  This ensures that the `main()` function is only executed when the script is run directly (not when it's imported as a module).  It's a standard practice in Python scripts.
* **Overall Fun:** It's a fun way to play with words and demonstrate how Python can be used for simple text processing.  The scrambled sentences are often surprisingly readable, which makes it engaging for the user.

**To run the script:**

1. Save the code as a `.py` file (e.g., `scrambler.py`).
2. Open a terminal or command prompt.
3. Navigate to the directory where you saved the file.
4. Run the script using `python scrambler.py`.
5. Enter a sentence when prompted.

Example:

```
Enter a sentence to scramble: This is a fun Python script
Scrambled sentence: Tihs is a fun Pyhton scirpt
```