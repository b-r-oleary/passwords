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

from passwords.generators.base import Base


class CaseBase(Base):
    pass


class CamelCase(CaseBase):
    def generate(self, seed: str='') -> str:
        seed = seed.lstrip().rstrip()
        words = seed.split(' ')
        return ''.join([word.title() for word in words])


class SnakeCase(CaseBase):
    def generate(self, seed: str='') -> str:
        seed = seed.lstrip().rstrip()
        words = seed.split(' ')
        return '_'.join([word.lower() for word in words])


class Upper(CaseBase):
    def generate(self, seed: str='') -> str:
        return seed.upper()


class Lower(CaseBase):
    def generate(self, seed: str='') -> str:
        return seed.lower()


class Capitalize(CaseBase):
    def generate(self, seed: str='') -> str:
        return seed.capitalize()


class RandomCase(CaseBase):
    def generate(self, seed: str='') -> str:
        options = [str.upper, str.lower]
        return ''.join([
            random.choice(options)(char)
            for char in seed]
        )


def Case(case: str) -> CaseBase:
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
        return cases[case]()
    else:
        raise KeyError("invalid case: allowed cases are: " + str(cases.keys()))
