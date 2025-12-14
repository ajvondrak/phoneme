# ☎️ phoneme

A toy program that lists all the [phone words](https://en.wikipedia.org/wiki/Phoneword) that can be spelled from a given phone number.

```console
$ phoneme --help
Searches for phone words that can be spelled from a given phone number

Usage: phoneme [OPTIONS] <DIGITS>

Arguments:
  <DIGITS>  Phone number to match, any string of digits 0-9

Options:
  -d, --dict <DICT>  Path to dictionary file, one word per line [default: /usr/share/dict/words]
  -h, --help         Print help
```

## Background

As I was solving [Advent of Code 2025](https://adventofcode.com/2025) in Rust, I remembered an old programming contest problem I first saw posed in [Ruby Quiz #20](http://rubyquiz.com/quiz20.html). I never solved it back then (in any language), but I've thought about it off & on for years. Carrying on with the momentum from AoC, I decided to finally scratch that mental itch and get some more Rust practice at the same time.

The name [phoneme](https://en.wikipedia.org/wiki/Phoneme) was a bit of wordplay on "phone" using a term from linguistics, which I enjoyed even though the program isn't *actually* searching for phonemes. You could also read it as "phone me".

## Example

The phone number 746-6363 can spell "phoneme", but what other mnemonics might be usable?

The quality of the output is highly dependent not just on the phone number, but also the dictionary file. On my computer, the default [words file](https://en.wikipedia.org/wiki/Words_%28Unix%29) leads to a bunch of garbage:

```console
$ phoneme 7466363 | wc -l
   22615
```

So I curated a custom dictionary by filtering out the weird non-words I didn't want to consider:

```console
$ grep -v '^\(.\|oe\|od\|ne\|od\|om\|mo\|si\|sho\|rio\|fod\|dod\|eme\|phoo\|pino\|mneme\|sion\)$' /usr/share/dict/words > /tmp/curated-dict
```

Then supplying the curated dictionary yields a few actually fun results:

```console
$ phoneme --dict /tmp/curated-dict 7466363
sin of of
sin of me
sin me of
sin mend
sin me me
shoo foe
shoo end
shoo doe
shood of
shood me
shone of
shone me
rim of of
rim of me
rim me of
rim mend
rim me me
rho of of
rho of me
rho me of
rho mend
rho me me
pi on foe
pi on end
pi one of
pi one me
pi on doe
pi no foe
pi no end
pi no doe
pi nod of
pi nod me
pin of of
pin of me
pin me of
pin mend
pin me me
pho of of
pho of me
phon foe
phon end
phone of
phone me
phoneme
phon doe
pho me of
pho mend
pho me me
```
