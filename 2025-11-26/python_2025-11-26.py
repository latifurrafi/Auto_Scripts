```python
import random

def poetic_madlib(noun1, adjective1, verb1, noun2, adjective2):
  """
  Generates a short, nonsensical poem using user-provided words.
  Demonstrates string formatting and random choice.

  Args:
    noun1: A noun (e.g., cat, sun, idea)
    adjective1: An adjective (e.g., fluffy, bright, abstract)
    verb1: An verb (e.g., jumps, shines, floats)
    noun2: Another noun (e.g., cloud, planet, feeling)
    adjective2: Another adjective (e.g., lonely, enormous, strange)

  Returns:
    A string containing the generated poem.
  """

  lines = [
      f"The {adjective1} {noun1} {verb1} with glee,",
      f"Across the {adjective2} expanse, wild and free.",
      f"A {noun2} whispers secrets untold,",
      f"As stories of yesterday unfold."
  ]

  # Optional: Introduce a random element to line order
  # random.shuffle(lines)  # Uncomment to scramble the lines

  poem = "\n".join(lines)
  return poem

if __name__ == "__main__":
  print("Welcome to the Poetic Madlib Generator!\n")

  noun1 = input("Enter a noun: ")
  adjective1 = input("Enter an adjective: ")
  verb1 = input("Enter a verb: ")
  noun2 = input("Enter another noun: ")
  adjective2 = input("Enter another adjective: ")

  poem = poetic_madlib(noun1, adjective1, verb1, noun2, adjective2)
  print("\nYour Poem:\n")
  print(poem)

  print("\nProgramming Concepts Demonstrated:")
  print("- String formatting (f-strings)")
  print("- Function definition and arguments")
  print("- List creation and joining")
  print("- Basic input/output")
  print("- (Optional) Random module (if shuffling is enabled)")
```

**How it Works and Programming Concepts Demonstrated:**

1. **`poetic_madlib(noun1, adjective1, verb1, noun2, adjective2)` Function:**
   - Takes five string arguments: two nouns, two adjectives, and one verb.
   - **String Formatting (f-strings):** Uses f-strings to create the poem lines. F-strings allow you to embed variables directly within a string by placing them inside curly braces `{}`. This is a clean and efficient way to build strings dynamically.
   - **List Creation:** Creates a list called `lines` containing the formatted poem lines.
   - **`join()` Method:**  Uses the `"\n".join(lines)` method to combine the elements of the `lines` list into a single string, separated by newline characters (`\n`). This creates the poem's formatting with each line on a new line.

2. **`if __name__ == "__main__":` Block:**
   - This block ensures that the code within it only runs when the script is executed directly (not when it's imported as a module into another script).
   - **Input/Output:**
     - Prompts the user to enter the required words using the `input()` function.
     - Calls the `poetic_madlib()` function to generate the poem.
     - Prints the generated poem to the console using `print()`.
   - **Explanation:** Provides a brief explanation of the programming concepts used in the script.

3. **Randomization (Optional):**
   - The `random.shuffle(lines)` line is commented out.  If you uncomment it, the script will also demonstrate the use of the `random` module. The `shuffle()` function shuffles the order of elements within a list in place (it modifies the original list). This would make the poem even more nonsensical and fun.

**How to Run It:**

1. Save the code as a `.py` file (e.g., `madlib_poem.py`).
2. Open a terminal or command prompt.
3. Navigate to the directory where you saved the file.
4. Run the script using: `python madlib_poem.py`

The script will then prompt you to enter the words, and it will generate a short, absurd poem based on your input.  It's a lighthearted way to illustrate string manipulation, functions, and basic user interaction in Python.