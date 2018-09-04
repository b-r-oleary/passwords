"""These are password generating objects from text - this depends on
a nltk sentence tokenizer to identify phrases from which to use
for passwords

TextPhrase      - given an input text, identify a random phrase
CorpusPhrase    - given an nltk corpus, select a random text, and select a
                  random phrase
Words           - random string of words from a list of words

"""
import re
import random
import os

from nltk import sent_tokenize
from nltk.corpus import words as nltk_words

from passwords.generators.base import Base


class TextPhrase(Base):
    """Given an input text, identify a random phrase.

    """
    def __init__(self,
                 text=None,
                 n=None,
                 n_min=None,
                 n_max=None,
                 remove_regex=r"([^a-zA-Z0-9\s]|\s'\s)",
                 split_regex=r"[\s]",
                 retries=10):
        """

        Parameters
        ----------
        text (string) a text from which to mine sentences
        n_min (int) min number of words to include in a phrase
        n_max (int) max number of words to include in a phrase
        remove_regex (regex string) regex for removing characters for
                        cleaning up the text
        split_regex (regex string) regex for splitting the text into
                        words
        retries (int) if the generated phrase length does not meet the
                        input criteria, this is the number of retries
                        allowed before returning a phrase that does
                        not satisfy the input criteria

        """
        ns = [i for i in
              [n, n_min, n_max]
              if i is not None]

        if len(ns) == 0:
            # default value for number of words
            ns = [4]

        self.n_min = min(ns)
        self.n_max = max(ns)

        self.remove_regex = remove_regex
        self.split_regex = split_regex
        self.retries = retries

        self.text = text
        self._sentences = None

    def get_text(self):
        return self.text

    def get_sentence(self):
        if self._sentences is None:
            self._sentences = sent_tokenize(self.get_text())
        return random.choice(self._sentences)

    def get_phrase(self):
        phrase = self.get_sentence()
        phrase = phrase.lower()
        phrase = re.sub(self.remove_regex, '', phrase)
        phrase = re.sub(self.split_regex, ' ', phrase)
        return phrase

    def get_valid_phrase(self):

        retries = 0
        while retries < self.retries:
            phrase = self.get_phrase()

            words = [word for word in
                     phrase.split(' ')
                     if word != '']

            if len(words) < self.n_min:
                retries += 1
                continue
            else:
                break

        if len(words) > self.n_max:
            words = words[:self.n_max]
        return ' '.join(words)

    def generate(self, seed=""):
        return seed + self.get_valid_phrase()

    def get_string_attributes(self):
        n = "n=[{n_min}, {n_max}]".format(n_min=self.n_min,
                                          n_max=self.n_max)
        text = '"{text}"'.format(
            text=shorten(self.get_text(), width=25, placeholder='...')
        )
        return ', '.join([n, text])


class CorpusPhrase(TextPhrase):
    """Given an nltk corpus, select a random text, and select a random phrase

    """
    def __init__(self,
                 corpus,
                 **kwargs):
        """

        Parameters
        ----------
        corpus (nltk corpus) - a corpus from which to select a random text
                    from which to select a random phrase.

        """
        self.corpus = corpus
        TextPhrase.__init__(self, **kwargs)

    def get_text(self):
        random_id = random.choice(self.corpus.fileids())
        return ' '.join(self.corpus.words(random_id))

    def get_sentence(self):
        sentences = sent_tokenize(self.get_text())
        return random.choice(sentences)

    def get_string_attributes(self):
        n = "n=[{n_min}, {n_max}]".format(n_min=self.n_min,
                                          n_max=self.n_max)
        corpus = os.path.split(self.corpus.root.path)[-1]
        return ', '.join([n, corpus])


class Words(Base):
    """Given a list of words (nltk.corpus.words.words('en-basic') by default),
    provide n randomly chosen words

    """
    def __init__(self,
                 words=None,
                 n=None,
                 n_min=None,
                 n_max=None):
        """

        Parameters
        ----------
        words (list) a list of words
        n (int) number of words
        n_min (int) min number of words
        n_max (int) max number of words

        """
        if words is None:
            words = nltk_words.words('en-basic')

        ns = [i for i in
              [n, n_min, n_max]
              if i is not None]

        if len(ns) == 0:
            # default value for number of words
            ns = [4]

        self.words = words
        self.n_min = min(ns)
        self.n_max = max(ns)

    def generate(self, seed=""):
        n = random.randint(self.n_min, self.n_max)
        words = [random.choice(self.words) for i in range(n)]
        return seed + " ".join(words)


def shorten(text, width=25, placeholder='...'):
    if len(text) < width:
        return text
    else:
        return text[:width] + placeholder
