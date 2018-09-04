"""These are password generating objects that modify the case of the seed
password.

CamelCase
SnakeCase
Upper
Lower
RandomCase

Case - a function for obtaining any case object with a string key input

"""
import random

from passwords.base import Base


class CamelCase(Base):
    def generate(self, seed=""):
        seed = seed.lstrip().rstrip()
        words = seed.split(' ')
        return ''.join([word.title() for word in words])


class SnakeCase(Base):
    def generate(self, seed=""):
        seed = seed.lstrip().rstrip()
        words = seed.split(' ')
        return '_'.join([word.lower() for word in words])


class Upper(Base):
    def generate(self, seed=""):
        return seed.upper()


class Lower(Base):
    def generate(self, seed=""):
        return seed.lower()


class Capitalize(Base):
    def generate(self, seed=""):
        return seed.capitalize()


class RandomCase(Base):
    def generate(self, seed=""):
        options = [str.upper, str.lower]
        return ''.join([random.choice(options)(char)
                        for char in seed])


def Case(case, *args, **kwargs):
    """A function for obtaining any case object with a string key input

    case (string) available options are: camel_case, snake_case, upper, lower
    capitalize, random_case.

    """
    cases = dict(camel_case=CamelCase,
                 snake_case=SnakeCase,
                 upper=Upper,
                 lower=Lower,
                 capitalize=Capitalize,
                 random_case=RandomCase)

    if case in cases:
        return cases[case](*args, **kwargs)
    else:
        raise IOError("invalid case: allowed cases are: " + str(cases.keys()))
