"""
these are the base classes for generating random strings that
constitute a password.

Base:       the base class that all password generating objects inherit
Password:   an object that constitutes a compound password generating object
            that consists of several password generating objects chained
            together
Constant:   This is the most trivial example of a password generating
            object that inherits from base. This object simply generates a
            constant
Switch:     performs a random switch amongst components in a password
            generating object

Surround:    surrounds the input password with characters

RandomSurround: randomly chooses a set of characters to surround a password

"""
import random
from typing import List, Optional, Tuple


class Base(object):
    """A base class that all password generating objects inherit.

    """
    def generate(self, seed: str='') -> str:
        """Objects that inherit from Base must implement the method 'generate'
        which takes a string input *seed* and outputs a string.

        """
        raise NotImplementedError

    def append(self, *args: 'Base') -> 'Password':
        """Appending another object that inherits from Base to an object that
        inherits from Base returns a compound password generating object
        *Password* which chains these two objects together.

        """
        return Password([self] + list(args))

    def generate_multiple(self, N: int, seed: str=''):
        """This method is included for convenience to generate an array of
        passwords rather than a single one.

        """
        if not isinstance(N, int):
            raise IOError('N must be an integer')
        for i in range(N):
            yield self.generate(seed=seed)

    def __str__(self) -> str:
        output = self.__class__.__name__
        attr = self.get_string_attributes()
        if attr is not None:
            output += ': ' + str(attr)
        return "<{output}>".format(output=output)

    def __repr__(self) -> str:
        return str(self)

    def get_string_attributes(self) -> Optional[str]:
        return None


class Password(Base):
    """An object that constitutes a compound password generating object
    that consists of several password generating objects chained together

    """
    def __init__(self, components: List[Base]) -> None:
        """

        Parameters
        ----------
        components (list of Base objects)

        """
        self.components = components

    def generate(self, seed: str='') -> str:
        """
        generate chains together the generate method for each component
        in self.components to generate a password
        """
        password = seed
        for component in self.components:
            password = component.generate(seed=password)
        return password

    def append(self, component: Base) -> 'Password':
        """
        this method overides Base.append to explictly append a Base object
        to the Password object rather than generating a new password object
        """
        self.components.append(component)
        return self

    def __str__(self) -> str:
        head = "<{cls}>".format(cls=self.__class__.__name__)
        components = indent(
            '\n'.join(
                [str(component) for component in self.components]
            ),
            '\t'
        )
        return '\n'.join([head, components])


class Switch(Password):
    """Performs a random switch amongst components in a password generating object.

    """
    def generate(self, seed: str="") -> str:
        return random.choice(
            self.components
        ).generate(seed=seed)


class Constant(Base):
    """This is the most trivial example of a password generating
    object that inherits from base. This object simply generates a
    constant

    """
    def __init__(self, value: str) -> None:
        self.value = value

    def generate(self, seed: str='') -> str:
        return seed + self.value

    def get_string_attributes(self) -> str:
        return self.value


class Surround(Base):
    def __init__(
            self,
            left: Optional[str]=None,
            right: Optional[str]=None) -> None:

        self.left = left or ''
        self.right = right or left or ''

    def generate(self, seed: str='') -> str:
        return ''.join([self.left, seed, self.right])

    def get_string_attributes(self) -> str:
        return '{} ... {}'.format(self.left, self.right)


def RandomSurround(braces: List[Tuple[str, str]]) -> Switch:
    return Switch([
        Surround(*brace) for brace in braces
    ])


def indent(text: str, indent_string: str) -> str:
    output = []
    for line in text.split('\n'):
        output.append(indent_string + line)
    return "\n".join(output)
