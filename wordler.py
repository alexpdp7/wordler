import enum


words = list(map(str.strip, open("words").readlines()))


class LetterState(enum.Enum):
    GREEN = "O"
    YELLOW = "X"
    GREY = "."


class Hint:
    """
    >>> Hint("cabal", "banal").pretty()
    '.OXOO'
    """
    def __init__(self, guess, word):
        self.guess = guess
        self.word = word

        state = [LetterState.GREY] * len(word)
        matched = [False] * len(word)

        for i in range(len(guess)):
            if word[i] == guess[i]:
                state[i] = LetterState.GREEN
                matched[i] = True
                continue
            for j in range(len(guess)):
                if word[i] == guess[j] and not matched[j]:
                    state[j] = LetterState.YELLOW
                    matched[i] = True
                    break

        self.state = state

    def pretty(self):
        return "".join([s.value for s in self.state])
