
from sympy import Symbol, Dummy, cse, parse_expr
import sympy.parsing.sympy_parser as spp

def flint(eq):
    reps = {}
    e = eq.replace(
        lambda x: x.is_Float and x == int(x),
        lambda x: reps.setdefault(x, Dummy()))
    return e.xreplace({v: int(k) for k, v in reps.items()})

def get_syms(symbols):
    syms = {}
    for sym in symbols:
        syms[sym] = Symbol(sym)
    return syms

def get_expr(expr, syms):

    symexpr = parse_expr(expr, local_dict=syms, transformations=(spp.convert_xor, spp.auto_number), evaluate=False)
    symexpr = flint(symexpr.simplify())

    return str(symexpr)

