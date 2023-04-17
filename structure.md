# Planned Structure

[[toc]]

## Structure

### Overarching game

- Fields:
    - bag: Vec\<Letter>
    - board: \[\[Tile]]
    - players: Vec\<Player>
    - set_words: Vec\<Word>
    - validator: WordChecker
- Functions:
    - place(x,y,Letter) -> Option\<(u32, viable_words)>
    - resign() -> void
    - reset() -> (hand)
    - skip() -> void
    - submit() -> (hand, u32)
    - swap() -> (hand)

### Fields

- bag
    - The collection of letters the players take from.
    - Shuffled when game created
    - Letters are only removed once they are played, not when they enter a player's hand
- board
    - The board the game is played on, has several types of bonuses
        - None
        - Double letter
        - Triple letter
        - Double word
        - Triple word
    - Bonuses are calculated in the order of letter -> word.
    - Letter bonuses apply to the letter than rests on them
    - Word bonuses apply to all words that use the tile the bonus rests on.
- players
    - The collection of players, should be 2 or more
- set_words
    - The words placed on the board

The fields are all set at creation of the Game, with there being a `standard()` that can be used, and a `custom()` if the user wishes to switch this up.

The fields are made up of various elementary components:

### Components

- Letter
    - Represents a letter of a certain set of characters that has been defined at creation of the Game.
    - The only other function it provides is containing the points a letter holds
    - By default there are some amount of blank tiles, which can be used to be any letter
    - There are only one of each given letter
- Tile
    - Represents a square on the board, including a:
        - Bonus multiplier
        - Letter
- Player
    - Represents a player and their:
        - Points
        - Hand (the letters they can play)
        - Name
- Word
    - Represents a word that has been played, including
        - The Letters making up the word
        - The points this word gave
- WordChecker
    - Is used to check if a word is valid
