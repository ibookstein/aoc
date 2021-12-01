import common
import string

_INPUT_POLYMER = common.get_input(day=5).strip()


def reduce_polymer(poly):
    i = 0
    while i < len(poly) - 1:
        unit1 = poly[i]
        unit2 = poly[i+1]
        if unit1.lower() == unit2.lower() and unit1 != unit2:
            poly = poly[:i] + poly[i+2:]
            i = max(i - 1, 0)
        else:
            i += 1
    return poly


def reduced_polymer_length(poly):
    return len(reduce_polymer(poly))


def remove_unit_from_polymer(poly, unit):
    keys = map(ord, (unit.lower(), unit.upper()))
    translation_dict = dict.fromkeys(keys, None)
    return poly.translate(translation_dict)


def solve_part1():
    return reduced_polymer_length(_INPUT_POLYMER)


def solve_part2():
    candidates = [remove_unit_from_polymer(_INPUT_POLYMER, c) for c in string.ascii_lowercase]
    return min(map(reduced_polymer_length, candidates))


print(solve_part1())
print(solve_part2())
