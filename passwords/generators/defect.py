"""These are password generating object that insert defects into
passwords according to a specified set of rules:

DefectMapping:      Given a specified set of possible character mappings and
                    given a specified range of possible number of defects,
                    generate defects in a seed password
AlphaDefects:       Inheriting from DefectMapping, apply defects to letters in
                    the seed password
SymbolReplacements: Inheriting from DefectMapping, apply defects to the seed
                    password in such a way that letters are replaced by similar
                    symbols

"""
import random

from passwords.generators.base import Base


class DefectMapping(Base):
    """Given a specified set of possible character mappings and
    given a specified range of possible number of defects, generate
    defects in a seed password.

    """
    def __init__(self,
                 mappings,
                 n=None,
                 n_min=None,
                 n_max=None,
                 force_change=False,
                 ):
        """

        Parameters
        ----------
        mappings (dictionary with string keys and string values) - this
            specifies that the characters in the key are to be randomly mapped
            to the characters in the values.
        n, n_min, n_max (int) - these inputs can be used to specify the number
            of defects to apply. If none are specified, the maximum number of
            possible defects will be applied. If n is specified, n defects will
            be applied if possible.
            If n_min and n_max are specfied, a randomly chosen number of
            defects will be applied between n_min and n_max if possible.
        force_change (bool) - this specifies whether or not to force defect
            changes to map a character to a different character. If this
            option is True, then no deterministic mappings in the mapping
            dictionary can map to a single character that exists in the key.

        """
        ns = [i for i in
              [n, n_min, n_max]
              if i is not None]

        if len(ns) == 0:
            ns = [None]

        self.n_min = min(ns)
        self.n_max = max(ns)

        self.force_change = force_change

        self.mappings = mappings

        # check that the mappings imput is valid
        self.mappings_valid()

    def mappings_valid(self):
        """
        perform error checking on the mappings input
        """
        items = list(self.mappings.items())
        # shuffle items in case a given character is in multiple keys
        random.shuffle(items)

        for key, value in items:

            if (not(isinstance(key,   str)) or
                not(isinstance(value, str))):
                raise IOError('mappings must map strings to strings')

            unique = set(list(value))
            N = len(unique)

            if N == 0:
                raise IOError('mapping must map to at least one character')
            if N == 1:
                if value in key and force_change:
                    raise IOError(
                        "deterministic mappings are not allowed when "
                        " forcing a change on the defect character.")

    def get_defect_from_char(self, char):
        """
        given a character, randomly select a defect replacement from the
        mappings dictionary
        """
        for old_chars, new_chars in self.mappings.items():
            if char in old_chars:
                return random.choice(new_chars)
        return None

    def get_n_defects(self, seed):
        """
        given the seed password, determine the number of defects that could
        possibly be applied, resolve that specified range of number of inputs.
        If a range is still possible randomly select a number of defects.

        """
        n_possible = 0
        for char in seed:
            for old_chars in self.mappings.keys():
                if char in old_chars:
                    n_possible += 1

        n_mins = [i for i in [self.n_min, n_possible]
                  if i <= n_possible and i is not None]
        n_maxs = [i for i in [self.n_max, n_possible]
                  if i <= n_possible and i is not None]

        n_min = min(n_mins)
        n_max = min(n_maxs)

        return random.randint(n_min, n_max)

    def generate(self, seed=""):
        if len(seed) < 1:
            return ""

        N = len(seed)

        # determine the number of defects to apply
        n = self.get_n_defects(seed)

        defects = []
        while len(defects) < n:
            index = random.randint(0, N - 1)
            if index in defects:
                continue
            old_char = seed[index]

            # obtain a defect
            new_char = self.get_defect_from_char(old_char)
            if new_char is None:
                continue

            # if we require the new character to be different from the initial
            # character then retry:
            if self.force_change:
                if old_char == new_char:
                    continue

            seed = seed[:index] + new_char + seed[index + 1:]
            defects.append(index)

        return seed

    def get_string_attributes(self):
        n = "n=[{n_min}, {n_max}]".format(n_min=self.n_min,
                                          n_max=self.n_max)
        if self.__class__.__name__ == 'DefectMapping':
            return ', '.join([n, str(self.mappings)])
        else:
            return n


class AlphaDefects(DefectMapping):
    """Inheriting from DefectMapping, apply defects to letters in the seed
    password respecting case, and optionally respecting vowels/consonants.

    """
    vowels = 'aeiou'
    letters = 'abcdefghijklmnopqrstuvwxyz'

    def __init__(self,
                 respect_vowels=True,
                 replace_vowels=True,
                 replace_consonants=False,
                 respect_consonants=True,
                 **kwargs):
        """
        *inputs*
        replace_[vowels/consonants] (bool) whether or not to replace
            vowels/consonants
        respect_[vowels/consonants] (bool) whether or not to replace
            vowels/consonants with vowels/consonants exclusively (otherwise
            replace with any letter)
        **kwargs - optional inputs for DefectMapping.__init__

        """
        mapping = dict()

        if replace_vowels:
            if respect_vowels:
                target = self.vowels
            else:
                target = self.letters

            mapping[self.vowels] = target

        if replace_consonants:
            if respect_consonants:
                target = self.consonants
            else:
                target = self.letters

            mapping[self.consonants] = target

        self.respect_vowels = respect_vowels
        self.replace_vowels = replace_vowels
        self.respect_consonants = respect_consonants
        self.replace_consonants = replace_consonants

        # respect case
        for old_chars, new_chars in list(mapping.items()):
            mapping[old_chars.upper()] = new_chars.upper()

        DefectMapping.__init__(self, mapping, **kwargs)

        self.consonants = [l for l in self.letters
                           if l not in self.vowels]

    def get_string_attributes(self):
        n = "n=[{n_min}, {n_max}]".format(n_min=self.n_min,
                                          n_max=self.n_max)
        attributes = dict()
        if self.replace_vowels:
            attributes["vowels"] = ("vowels" if self.respect_vowels
                                    else "all")
        if self.replace_consonants:
            attributes["consonants"] = ("consonants" if self.respect_consonants
                                        else "all")
        return ', '.join([n, str(attributes)])


class SymbolReplacements(DefectMapping):
    """Inheriting from DefectMapping, apply defects to the seed password
    in such a way that letters are replaced by similar symbols

    """
    def __init__(self, **kwargs):
        mappings = {
            "A": "4",
            "OoQ": "0",
            "E": "3",
            "LlIJ": "1",
            "ij": "!:;",
            "Ss": "$5",
            "Zz": "2",
            "LVv": "7^",
            "a": "@",
            "N": "\%",
            "B": "8\%&",
            "Ppq": "9",
            "bd": "6&",
            "XxfF": "+",
            "H": "#",
        }

        DefectMapping.__init__(self, mappings, **kwargs)
