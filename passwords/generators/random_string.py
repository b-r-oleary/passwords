"""These are password generating objects that generate random strings from an
allowed set of characters with a uniform distribution.

"""
import random

from passwords.generators.base import Base, Password, Constant


class RandomString(Base):
    """Generate random strings from an allowed set of characters with a
    uniform distribution.

    """
    def __init__(self, n: int, chars: str) -> None:
        """

        Parameters
        ----------
        n (int) length of the random string
        chars (string) characters allowed for the random string generation

        """
        self.n = n
        self.chars = chars

    def generate_string(self):
        return ''.join([
            random.choice(self.chars)
            for i in range(self.n)
        ])

    def generate(self, seed=""):
        return seed + self.generate_string()

    def __add__(self, other):
        """
        two random string objects added together
        result in an object that can generate
        characters from both objects
        """
        if isinstance(other, RandomString):
            n = max([self.n, other.n])
            chars = self.chars + other.chars
            return RandomString(n, chars)
        else:
            raise NotImplementedError("addition of a random string to " + other.__class__.__name__ + " is not defined.")

    def get_string_attributes(self):
        n = 'n={n}'.format(n=self.n)
        if self.__class__.__name__ == 'RandomString':
            return ', '.join([n, self.chars])
        else:
            return n


class Integers(RandomString):
    def __init__(self, n):
        RandomString.__init__(self, n, "0123456789")


class Hexadecimal(RandomString):
    def __init__(self, n,
                 upper=False):
        chars = "0123456789abcdef"
        if upper:
            chars = chars.upper()
        RandomString.__init__(self, n, chars)


class Letters(RandomString):
    def __init__(self, n,
                 upper=True,
                 lower=True):
        alphabet = "abcdefghijklmnopqrstuvwxyz"
        chars = ""
        if upper:
            chars += alphabet.upper()
        if lower:
            chars += alphabet.lower()

        RandomString.__init__(self, n, chars)


def AlphaNumeric(n,
                 numbers=True,
                 upper=True,
                 lower=True):

    an = RandomString(n, "")

    if upper or lower:
        an += Letters(n,
                      upper=upper,
                      lower=lower)

    if numbers:
        an += Integers(n)

    return an


def UUID4(variant=1):
    """Generates a random UUID
    (https://en.wikipedia.org/wiki/Universally_unique_identifier)
    with variant specified in the keyword argument,
    for version 4 (random)

    """
    M = "4"  # this is the version
    chars = "0123456789abcdef"
    variants = [chars[0:8],
                chars[8:12],
                chars[12:14],
                chars[14:16]]

    if variant not in range(len(variants)):
        raise NotImplementedError(
            "variant must be in range(" + str(len(variants)) + ")")

    N = RandomString(1, variants[variant])

    return Password([
        Hexadecimal(8),
        Constant('-'),
        Hexadecimal(4),
        Constant('-'),
        Constant(str(M)),
        Hexadecimal(3),
        Constant('-'),
        N,
        Hexadecimal(3),
        Constant('-'),
        Hexadecimal(12),
    ])

uuid4 = UUID4().generate
