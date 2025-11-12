```python
import random
import time

def storyteller():
  """
  A simple program that tells a slightly different story each time it's run,
  demonstrating string formatting, lists, and random choice.
  """

  characters = ["a brave knight", "a mischievous gnome", "a wise old wizard", "a curious cat", "a dancing bear"]
  places = ["a haunted forest", "a sparkling river", "a towering mountain", "a secret garden", "a bustling marketplace"]
  adventures = ["searched for a lost treasure", "solved a mysterious riddle", "helped a friendly dragon", "thwarted an evil sorcerer", "discovered a hidden world"]
  outcomes = ["lived happily ever after", "learned a valuable lesson", "returned home a hero", "made a new friend", "became a legend"]

  # Randomly select elements from our lists
  character = random.choice(characters)
  place = random.choice(places)
  adventure = random.choice(adventures)
  outcome = random.choice(outcomes)

  # Build the story using string formatting
  story = f"Once upon a time, there was {character} who ventured into {place}.\n"
  time.sleep(1)  # Add a slight pause for dramatic effect
  story += f"They {adventure}.\n"
  time.sleep(1)
  story += f"And in the end, they {outcome}."

  print("\n" + story + "\n")


if __name__ == "__main__":
  print("Welcome to the Random Story Generator!\n")
  storyteller()
```

**How it works and programming concepts demonstrated:**

1. **Lists:** The script uses lists (`characters`, `places`, `adventures`, `outcomes`) to store different options for each part of the story. This allows for variation and makes the story different each time.

2. **`random.choice()`:**  The `random.choice()` function is used to randomly pick an element from each list. This is how the story's details are randomized.

3. **String Formatting (f-strings):**  The `f"..."` syntax is used for string formatting. This allows us to easily embed the randomly chosen words into the story. This is much cleaner than older string formatting methods (e.g., `%` formatting).  Using f-strings helps to avoid errors with types and allows you to include expressions inside of the strings.

4. **`time.sleep()`:** The `time.sleep(1)` function adds short pauses to the output, making the story feel a bit more like a real story being told.

5. **`if __name__ == "__main__":`:**  This is standard Python practice.  The code inside this block will only run if the script is executed directly (e.g., `python story.py`) and *not* if it's imported as a module into another script.

**How to run it:**

1. Save the code as a Python file (e.g., `story.py`).
2. Open a terminal or command prompt.
3. Navigate to the directory where you saved the file.
4. Run the script using the command: `python story.py`

Each time you run it, you'll get a slightly different (and potentially silly) story!