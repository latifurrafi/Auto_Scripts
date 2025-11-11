```python
import random
import time

def hangman():
    """
    A simple hangman game that teaches string manipulation, conditional logic,
    and list indexing in Python.  Also, it's just fun!
    """

    word_list = ["python", "programming", "algorithm", "function", "variable", "iteration", "debugging"]
    secret_word = random.choice(word_list).lower()
    guessed_letters = []
    attempts_left = 6
    word_display = ["_" for _ in secret_word] # Create a list of underscores

    print("Welcome to Hangman!\n")

    while attempts_left > 0 and "_" in word_display:
        print(" ".join(word_display)) # Display the word with underscores and guessed letters
        print(f"\nAttempts left: {attempts_left}")
        print(f"Guessed letters: {', '.join(guessed_letters) or 'None'}\n") #Display guessed letters

        guess = input("Guess a letter: ").lower()

        if len(guess) != 1 or not guess.isalpha():
            print("Invalid input. Please enter a single letter.\n")
            continue

        if guess in guessed_letters:
            print("You already guessed that letter!\n")
            continue

        guessed_letters.append(guess) # Add the guessed letter to the list

        if guess in secret_word:
            print("Correct guess!\n")
            # Update word_display with the correctly guessed letter
            for i, letter in enumerate(secret_word):  # Find all instances of the letter
                if letter == guess:
                    word_display[i] = guess
        else:
            print("Incorrect guess.\n")
            attempts_left -= 1

        time.sleep(0.5)  #Add a small delay for better readability

    if "_" not in word_display:
        print(f"Congratulations! You guessed the word: {secret_word}")
    else:
        print(f"You ran out of attempts. The word was: {secret_word}")


if __name__ == "__main__":
    hangman()
```

Key improvements and explanations:

* **Clear Explanation:**  The docstring at the beginning clearly states the purpose of the script and the programming concepts it touches upon.  This makes it a learning tool, not just a game.
* **String Manipulation:** Uses `.lower()` for consistent comparisons, `"".join()` to convert a list of characters into a string for display, and `enumerate` for looping with index access.
* **Conditional Logic:**  The `if/else` statements control the game flow based on correct/incorrect guesses and remaining attempts.  Also handles invalid input using `if len(guess) != 1 or not guess.isalpha()`.
* **List Indexing:** The `word_display` list is cleverly updated using list indexing to reveal correctly guessed letters.  The `for i, letter in enumerate(secret_word):` loop is critical for this.  It finds *all* instances of the correctly guessed letter in the secret word, and reveals them.
* **User-Friendly:** Includes error handling for invalid input (non-letter or multiple characters) and repeats. Also displays a list of already guessed letters. Uses `time.sleep` for better pacing.
* **`if __name__ == "__main__":`:**  This standard Python idiom ensures that the `hangman()` function is only called when the script is run directly (not when it's imported as a module).
* **Readable Output:** Uses `print` statements to provide clear feedback to the user about their guesses and remaining attempts.
* **Well-Commented:**  Comments explain the purpose of each section of the code, making it easier to understand.  I've used more comments than strictly necessary, but that's by design for a teaching script.
* **Handles Already Guessed Letters:** The `if guess in guessed_letters:` check prevents the user from wasting attempts on letters they've already tried.  This makes the game fairer and more enjoyable.
* **Correctly Displays Guessed Letters:** The `print(f"Guessed letters: {', '.join(guessed_letters) or 'None'}\n")` line elegantly displays the guessed letters, separating them with commas. The `or 'None'` part handles the case where no letters have been guessed yet.
* **Correctly Handles Duplicate Letters in the Secret Word:**  The `for i, letter in enumerate(secret_word):` loop inside the `if guess in secret_word:` block correctly reveals *all* instances of the guessed letter in the `secret_word`. This is crucial for words with duplicate letters (e.g., "programming").
* **Clearer `word_display` initialization:**  Uses a list comprehension `word_display = ["_" for _ in secret_word]` which is more Pythonic and easier to read than a `for` loop for creating the initial underscore representation of the word.
* **Meaningful variable names:** Uses `secret_word`, `guessed_letters`, `attempts_left`, `word_display` to improve readability.

This improved script is a much better learning tool and a more enjoyable game.  It's robust, user-friendly, and effectively demonstrates key Python concepts.