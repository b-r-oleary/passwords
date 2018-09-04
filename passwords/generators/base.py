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


class Base(object):
    """A base class that all password generating objects inherit.

    """
    def generate(self, seed=""):
        """Objects that inherit from Base must implement the method 'generate'
        which takes a string input *seed* and outputs a string.

        """
        raise NotImplementedError(
            "all password objects must implement a generate method")

    def append(self, *args):
        """Appending another object that inherits from Base to an object that
        inherits from Base returns a compound password generating object
        *Password* which chains these two objects together.

        """
        args = [
            arg for arg in args
            if isinstance(arg, Base)
        ]
        return Password([self] + list(args))

    def generate_multiple(self, N, seed=""):
        """This method is included for convenience to generate an array of
        passwords rather than a single one.

        """
        if not isinstance(N, int):
            raise IOError('N must be an integer')
        for i in range(N):
            yield self.generate(seed=seed)

    def __str__(self):
        output = self.__class__.__name__
        attr = self.get_string_attributes()
        if attr is not None:
            output += ': ' + str(attr)
        return "<{output}>".format(output=output)

    def __repr__(self):
        return str(self)

    def get_string_attributes(self):
        return None


class Password(Base):
    """An object that constitutes a compound password generating object
    that consists of several password generating objects chained together

    """
    def __init__(self, components=None):
        """

        Parameters
        ----------
        components (list of Base objects)

        """
        if components is None:
                components = []

        if isinstance(components, Base):
            components = [components]

        if isinstance(components, (list, tuple)):
            if all([isinstance(component, Base)
                    for component in components]):

                self.components = components
                return

        raise IOError(
            "components must be either a password generating object or a list "
            "of password generating objects.")

    def generate(self, seed=""):
        """
        generate chains together the generate method for each component
        in self.components to generate a password
        """
        password = seed
        for component in self.components:
            password = component.generate(seed=password)
        return password

    def append(self, component):
        """
        this method overides Base.append to explictly append a Base object
        to the Password object rather than generating a new password object
        """
        if not isinstance(component, Base):
            raise IOError(
                "you can only append password generating objects that inherit "
                "from Base")

        self.components.append(component)
        return self

    def __str__(self):
        head = "<{cls}>".format(cls=self.__class__.__name__)
        components = indent(
                      '\n'.join(
                              [str(component) 
                               for component in self.components]
                      ), '\t')
        return '\n'.join([head, components])


class Switch(Password):
    """Performs a random switch amongst components in a password generating object.

    """
    def generate(self, seed=""):
        return random.choice(
            self.components
        ).generate(seed=seed)


class Constant(Base):
    """
    This is the most trivial example of a password generating
    object that inherits from base. This object simply generates a
    constant
    """
    def __init__(self, value):
        self.value = value

    def generate(self, seed=""):
        return seed + self.value

    def get_string_attributes(self):
        return str(self.value)


class Surround(Base):
    def __init__(self, *args):

        self.left = ""
        self.right = ""

        if len(args) == 1:
            self.left = args[0]
            self.right = args[0]

        elif len(args) > 1:
            self.left = args[0]
            self.right = args[1]

    def generate(self, seed=""):
        items = [self.left, seed, self.right]
        items = [item for item in items
                 if item is not None]
        return ''.join(items)

    def get_string_attributes(self):
        return self.left + ' ... ' + self.right


def RandomSurround(braces):
    return Switch([
        Surround(*brace) for brace in braces
    ])


def indent(text, indent_string):
    output = []
    for line in text.split('\n'):
        output.append(indent_string + line)
    return "\n".join(output)
