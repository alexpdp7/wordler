import enum
import multiprocessing
import string

try:
    from tqdm import tqdm
except:
    def tqdm(x):
        return x


words = list(map(str.strip, open("words").readlines()))


class WordsWo:
    def __init__(self):
        self.words_without = None
        self.words_with = None

    def words_wo(self):
        if self.words_without and self.words_with:
            return self.words_without, self.words_with

        self.words_without = dict()
        self.words_with = dict()

        for l in string.ascii_lowercase:
            self.words_with[l] = set()
            self.words_without[l] = set()

            for word in words:
                if l in word:
                    self.words_with[l].add(word)
                else:
                    self.words_without[l].add(word)

        return self.words_without, self.words_with

words_wo = WordsWo()

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

    def set_of_color(self, color):
        """
        >>> Hint("cabal", "banal").set_of_color(LetterState.GREY)
        {'c'}
        >>> Hint("cabal", "banal").set_of_color(LetterState.YELLOW)
        {'b'}
        >>> Hint("cabal", "banal").set_of_color(LetterState.GREEN) == {'a', 'l'}
        True
        """
        return set([l for i, l in enumerate(self.guess) if self.state[i] == color])


def possible_words(hint):
    """
    >>> possibles_after_fuzzy_if_banal = possible_words(Hint("fuzzy", "banal"))
    >>> len(possibles_after_fuzzy_if_banal)
    2921
    >>> possibles_after_eases_if_banal = possible_words(Hint("eases", "banal"))
    >>> len(possibles_after_eases_if_banal)
    210
    >>> "later" in possibles_after_eases_if_banal
    False
    >>> "later" in possibles_after_fuzzy_if_banal
    True
    """
    # all words are possible
    result = set(words)

    words_without, words_with = words_wo.words_wo()

    # remove words that have a letter that appears only as grey
    for gl in hint.set_of_color(LetterState.GREY) - (hint.set_of_color(LetterState.YELLOW) | hint.set_of_color(LetterState.GREEN)):
        result.intersection_update(words_without[gl])

    # has = non-grey letters
    has = set()
    for i in range(len(hint.guess)):
        if hint.state[i] != LetterState.GREY:
            has.add(hint.guess[i])

    # only keep words which have all grey letters
    for ha in has:
        result.intersection_update(words_with[ha])

    # only keep words matching greens
    not_matching_greens = set()
    for word in result:
        def matches_greens():
            for i in range(len(hint.guess)):
                if hint.state[i] == LetterState.GREEN and hint.guess[i] != word[i]:
                    return False
            return True

        if not matches_greens():
            not_matching_greens.add(word)

    result.difference_update(not_matching_greens)

    yellows_in_guess = dict()
    for i in range(len(hint.guess)):
        if hint.state[i] == LetterState.YELLOW:
            yellows_in_guess[hint.guess[i]] = yellows_in_guess.get(hint.guess[i], 0) + 1

    to_remove = set()

    for word in result:
        letters_in_word = dict()

        for i in range(len(hint.guess)):
            if hint.state[i] != LetterState.GREEN:
                letters_in_word[word[i]] = letters_in_word.get(word[i], 0) + 1

        good_yellows = True
        for yellow_in_guess, quantity in yellows_in_guess.items():
            if letters_in_word.get(yellow_in_guess, 0) < quantity:
                good_yellows = False
                break

        if not good_yellows:
            to_remove.add(word)

    result.difference_update(to_remove)

    return result


def average_possibles_after(guess):
    a = 0
    b = 0
    for word in words:
        if word == guess:
            continue
        hint = Hint(guess, word)
        possibles = len(possible_words(hint))
        a = a + 1
        b = b + possibles
    return b/a


def _words_according_to_average_possibles_after_to_dict(w):
    return {'guess': w, 'avg': average_possibles_after(w)}


def words_according_to_average_possibles_after():
    result = []
    with multiprocessing.Pool() as pool:
        with tqdm(total=len(words)) as progress:

            for guess_result in pool.imap_unordered(_words_according_to_average_possibles_after_to_dict, words):
                result.append(guess_result)
                progress.update()
    return result
