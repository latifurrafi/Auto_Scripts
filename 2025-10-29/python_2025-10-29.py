```python
import random

def storyteller(num_adjectives=3):
  """
  Generates a short, silly story using random adjectives to demonstrate string formatting and lists.

  Args:
    num_adjectives: The number of random adjectives to use in the story.  Defaults to 3.
                    Increasing this makes the story more absurd.

  Returns:
    A string containing the generated story.
  """

  adjectives = ["fluffy", "grumpy", "sparkly", "invisible", "gigantic", "tiny", "smelly", "purple", "melodious", "electric"]

  # Generate a list of random adjectives.  This is our first learning point: lists!
  selected_adjectives = random.sample(adjectives, num_adjectives)

  # Our second learning point: string formatting!  We'll use f-strings for readability.
  story = f"""
  Once upon a time, there was a {selected_adjectives[0]} dragon who lived in a {selected_adjectives[1]} cave.  

  One day, the dragon decided to go on a {selected_adjectives[2]} adventure!  It flew over mountains of cheese and past rivers of chocolate.  

  The end.  (Or is it?)
  """

  return story


# Run the storyteller and print the result.
if __name__ == "__main__":
  story = storyteller() # Or storyteller(5) for a longer story!
  print(story)
```

Key Concepts Demonstrated:

* **Lists:** The `adjectives` list stores possible adjectives, and `selected_adjectives` stores the chosen ones. This demonstrates how to create, access, and manipulate lists.  `random.sample` is used to pick elements from a list without replacement.
* **String Formatting (f-strings):**  The `story` is constructed using f-strings, allowing variables (the selected adjectives) to be easily embedded into the string.  This is a modern and readable way to format strings.
* **Functions:** The `storyteller` function encapsulates the story generation logic, making the code reusable and organized. It takes an optional argument to control the number of adjectives.
* **Randomness:** The `random.sample` function introduces an element of surprise and makes each story unique.
* **`if __name__ == "__main__":`:**  This ensures the `storyteller()` function is only called when the script is run directly, not when it's imported as a module.
* **Docstring:**  The function includes a docstring, explaining what it does, its arguments, and its return value. Good practice!

How to run the script:

1. Save the code as a Python file (e.g., `story_generator.py`).
2. Open a terminal or command prompt.
3. Navigate to the directory where you saved the file.
4. Run the script using the command: `python story_generator.py`

Each time you run the script, it will generate a slightly different, silly story.  You can experiment with different values for `num_adjectives` in the `storyteller()` function call to see how it affects the story's length and absurdity.  You can also easily add more adjectives to the `adjectives` list to expand the possibilities.