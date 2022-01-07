Some experiments with [Wordle](https://www.powerlanguage.co.uk/wordle/).

# `words_according_to_average_possibles_after`

```
>>> import wordler
>>> wordler.words_according_to_average_possibles_after()
```

This computes a list of dicts. Each entry contains a guess, and the number of average possible words after entering that guess. That takes about 6 hours to complete.

The results are stored in `words_according_to_average_possibles_after.py`.

```
>>> import ast
>>> words_according_to_average_possibles_after = ast.literal_eval(open("words_according_to_average_possibles_after.py").read())
>>> s = sorted(words_according_to_average_possibles_after, key=lambda e: e["avg"])
>>> s[0:10]
[{'guess': 'eases', 'avg': 130.9542782495101}, {'guess': 'sears', 'avg': 152.18811234487262}, {'guess': 'oases', 'avg': 157.75027215327674}, {'guess': 'seers', 'avg': 158.2094491617679}, {'guess': 'saree', 'avg': 158.80078380143698}, {'guess': 'sires', 'avg': 160.6485956890921}, {'guess': 'sales', 'avg': 160.75179621162638}, {'guess': 'sense', 'avg': 161.01850642281732}, {'guess': 'sores', 'avg': 161.33311561071196}, {'guess': 'roses', 'avg': 162.25234051817984}]
>>> s[-10:]
[{'guess': 'jazzy', 'avg': 1607.7594165033747}, {'guess': 'whizz', 'avg': 1631.6518615284128}, {'guess': 'jiffy', 'avg': 1638.4038754626606}, {'guess': 'fizzy', 'avg': 1639.1421728717614}, {'guess': 'fluff', 'avg': 1653.398432397126}, {'guess': 'yuppy', 'avg': 1731.7602873938602}, {'guess': 'puppy', 'avg': 1764.747006313956}, {'guess': 'yummy', 'avg': 1809.4726758110169}, {'guess': 'mummy', 'avg': 1847.1295449597212}, {'guess': 'fuzzy', 'avg': 1976.2268669714783}]
```
