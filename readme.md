# Spellchecker
---

A super simple spellchecker that compares spelling of files passed as arguments to the unix
dictionary. It generates a set to compare against before every iteration, and simply separates
words on whitespace to compare versus this set.

```
$ spellchecker writing.txt
```

# TODO
---

Some sort of normalization of words so that they can checked with their variations such as
plural.
