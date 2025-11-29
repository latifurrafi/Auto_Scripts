```python
import random

def mad_libs_generator():
  """
  Generates a Mad Libs story using user input.
  Demonstrates string formatting, user input, and lists.
  """

  # Story template with placeholders
  story_template = """
  Once upon a time, in a land filled with {adjective} {plural_noun}, 
  lived a {adjective2} {noun}. This {noun} loved to {verb} {adverb}. 
  One day, while {verb}ing {adverb2}, the {noun} stumbled upon a {adjective3} {noun2}.
  Intrigued, the {noun} decided to {verb2} the {noun2}.  
  And that's how the {adjective} {plural_noun} learned about {plural_noun2}! 
  The End.
  """

  # Prompt the user for input
  print("Let's play Mad Libs!")
  adjective = input("Enter an adjective: ")
  plural_noun = input("Enter a plural noun: ")
  adjective2 = input("Enter another adjective: ")
  noun = input("Enter a noun: ")
  verb = input("Enter a verb: ")
  adverb = input("Enter an adverb: ")
  verbing = input("Enter a verb ending in -ing: ")
  adverb2 = input("Enter another adverb: ")
  adjective3 = input("Enter a third adjective: ")
  noun2 = input("Enter a second noun: ")
  verb2 = input("Enter a second verb: ")
  plural_noun2 = input("Enter a third plural noun: ")


  # Create a dictionary to hold the user's inputs
  words = {
      "adjective": adjective,
      "plural_noun": plural_noun,
      "adjective2": adjective2,
      "noun": noun,
      "verb": verb,
      "adverb": adverb,
      "verbing": verbing,
      "adverb2": adverb2,
      "adjective3": adjective3,
      "noun2": noun2,
      "verb2": verb2,
      "plural_noun2": plural_noun2
  }

  # Format the story with the user's input
  final_story = story_template.format(**words)

  # Print the story
  print("\nHere's your Mad Libs story:")
  print(final_story)

if __name__ == "__main__":
  mad_libs_generator()
```

**How it works and what it teaches:**

1. **`mad_libs_generator()` function:** Encapsulates the entire Mad Libs game logic.
2. **String Formatting:** The `story_template` variable is a string containing placeholders like `{adjective}`, `{noun}`, etc.  The `format(**words)` method is used to substitute the placeholders with the values from the `words` dictionary. This demonstrates a powerful way to insert data into strings.  The double asterisk `**` unpacks the dictionary into keyword arguments for the `format()` method.  For example, `story_template.format(adjective=user_adjective, noun=user_noun, ...)`
3. **User Input:** The `input()` function is used to prompt the user for words to fill in the blanks. This teaches how to get input from the user and store it in variables.
4. **Dictionaries:** The `words` dictionary is used to store the user's input in a key-value pair format.  This allows you to easily access and use the words in the story.
5. **`if __name__ == "__main__":` block:**  This ensures that the `mad_libs_generator()` function is only called when the script is run directly (not when it's imported as a module into another script). This is a standard practice in Python.
6. **Lists (implicitly):** While this example doesn't explicitly use lists, you could easily modify it to have the user provide multiple adjectives or nouns and store them in lists.  Then, you could use `random.choice(my_list)` to randomly select a word from the list for each placeholder.  This would make the game even more unpredictable and fun.

**How to run it:**

1. Save the code as a Python file (e.g., `madlibs.py`).
2. Open a terminal or command prompt.
3. Navigate to the directory where you saved the file.
4. Run the script using the command: `python madlibs.py`
5. Follow the prompts to enter the requested words.
6. Enjoy your hilarious Mad Libs story!

**Possible Extensions and Improvements:**

* **Multiple Story Templates:**  Store several different story templates in a list and randomly choose one for each game.
* **Difficulty Levels:**  Introduce different levels of difficulty based on the number of words required.
* **GUI:** Create a graphical user interface (GUI) using a library like Tkinter or PyQt to make the game more visually appealing.
* **Random Word Suggestions:**  Provide the user with random word suggestions if they are stuck.  You could use a dictionary lookup to provide synonyms.
* **Error Handling:** Add error handling to ensure the user enters valid input (e.g., checking if the input is a valid number).

This script is a simple but fun way to learn about fundamental Python concepts like string formatting, user input, dictionaries, and basic program flow. It's also a good starting point for more complex text-based games.