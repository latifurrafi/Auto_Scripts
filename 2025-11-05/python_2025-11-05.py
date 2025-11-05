```python
import random

def mad_libs_generator():
  """
  A simple Mad Libs generator that demonstrates string formatting and user input.
  """

  noun = input("Enter a noun: ")
  adjective = input("Enter an adjective: ")
  verb = input("Enter a verb (present tense): ")
  adverb = input("Enter an adverb: ")
  place = input("Enter a place: ")

  story = f"The {adjective} {noun} {verb} {adverb} near the {place}. It was a particularly odd day!"

  print("\nHere's your Mad Lib story:")
  print(story)

def number_guesser():
  """
  A number guessing game that teaches basic conditional statements and loops.
  """
  secret_number = random.randint(1, 100)
  guess = 0
  attempts = 0

  print("Welcome to the Number Guessing Game!")
  print("I'm thinking of a number between 1 and 100.")

  while guess != secret_number:
    try:
      guess = int(input("Take a guess: "))
      attempts += 1

      if guess < secret_number:
        print("Too low!")
      elif guess > secret_number:
        print("Too high!")
      else:
        print(f"Congratulations! You guessed the number in {attempts} attempts!")
    except ValueError:
      print("Invalid input. Please enter a number.")

if __name__ == "__main__":
  while True:
    print("\nChoose an option:")
    print("1. Mad Libs Generator")
    print("2. Number Guessing Game")
    print("3. Exit")

    choice = input("Enter your choice (1-3): ")

    if choice == "1":
      mad_libs_generator()
    elif choice == "2":
      number_guesser()
    elif choice == "3":
      print("Goodbye!")
      break
    else:
      print("Invalid choice. Please try again.")
```

Key improvements and explanations:

* **Clear Structure:** The code is now organized into functions (`mad_libs_generator`, `number_guesser`) for better readability and reusability. This is crucial for maintainability.
* **`if __name__ == "__main__":` block:** This is essential.  It ensures that the main part of your script (the menu and game selection) only runs when the script is executed directly, and not when imported as a module.
* **User-Friendly Menu:** A menu is provided to let the user choose between the two fun options or exit.  This makes it much more user-friendly.
* **Error Handling:**  The `number_guesser` function now includes a `try-except` block to handle potential `ValueError` exceptions if the user enters non-numeric input.  This prevents the program from crashing.
* **Clear Instructions:** The `number_guesser` function provides clear instructions about the range of numbers.
* **String Formatting (f-strings):** Uses f-strings (e.g., `f"The {adjective} {noun}..."` and `f"Congratulations! You guessed the number in {attempts} attempts!"`) for more readable and efficient string formatting.
* **Conditional Logic:** The `number_guesser` game effectively demonstrates `if`, `elif`, and `else` statements for comparison and branching.
* **Looping:**  The `while` loop in the `number_guesser` game continues until the user guesses the correct number.  The outer `while` loop keeps the menu active until the user explicitly chooses to exit.
* **Comments:**  The code includes comments explaining the purpose of different sections.
* **Concise Code:**  The code is written in a concise and readable style.
* **Educational Value:**  Each function teaches a specific concept:
    * `mad_libs_generator`: String formatting (f-strings), user input, variables.
    * `number_guesser`:  Conditional statements, loops, error handling (try-except), random number generation.

How to run the code:

1. Save the code as a `.py` file (e.g., `fun_script.py`).
2. Open a terminal or command prompt.
3. Navigate to the directory where you saved the file.
4. Run the script using `python fun_script.py`.

The script will then guide you through the options.