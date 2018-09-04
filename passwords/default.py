"""
this includes some default password generators constructed using password
generating components

DefectPhrasePassword
"""

from passwords.constructor.base import Password, Switch
from passwords.constructor.phrase import CorpusPhrase, Words
from passwords.constructor.case import Case
from passwords.constructor.defect import AlphaDefects, SymbolReplacements
from passwords.constructor.random_string import Integers

from nltk.corpus import movie_reviews


def DefectPhrasePassword(
        corpus=movie_reviews,
        phrase_min=3,
        phrase_max=4,
        case="camel_case",
        n_defect=1,
        n_integers=2,
        force_change=True):
    """

    Parameters
    ----------
    corpus (nltk corpus) - corpus from which to obtain random phrases
    phrase_min (int) - min number of words in a phrase
    phrase_max (int) - max number of words in a phrase
    case (str) - string key corresponding to the case to apply to phrase
    n_defect (int) - number of defects to include in the phrase
    n_integers (int) - length of the integer tail to apply to the password
    force_change (bool) - whether or not to require a character change in the
        defects

    """
    return Password([
        CorpusPhrase(movie_reviews,
                     n_min=phrase_min,
                     n_max=phrase_max),
        Case(case),
        Switch([
            SymbolReplacements(n=n_defect),
            AlphaDefects(n=n_defect,
                         force_change=force_change)
        ]),
        Integers(n=n_integers)
    ])

# this is the password generating method for DefectPhrasePassword, which we
# have selected to be the default password generating object
generate_password = DefectPhrasePassword().generate

# passwords with this specification https://xkcd.com/936/
xkcd_password = Words(n=4).generate
