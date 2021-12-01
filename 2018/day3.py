import re
import common
import collections

_CLAIM_REGEX = re.compile(r'#(?P<claim_id>\d+) @ (?P<left_gap>\d+),(?P<top_gap>\d+): (?P<width>\d+)x(?P<height>\d+)')
_INPUT_LINES = common.get_input_lines(day=3)
Claim = collections.namedtuple('Claim', ('claim_id', 'left_gap', 'top_gap', 'width', 'height'))


def parse_claim_line(line):
    match = _CLAIM_REGEX.match(line)
    assert match
    return Claim(*(int(match.group(name)) for name in Claim._fields))


_INPUT_CLAIMS = list(map(parse_claim_line, _INPUT_LINES))


def solve():
    grid_cells = {}
    claim_ids = set(c.claim_id for c in _INPUT_CLAIMS)

    for claim in _INPUT_CLAIMS:
        for i in range(claim.left_gap, claim.left_gap + claim.width):
            for j in range(claim.top_gap, claim.top_gap + claim.height):
                grid_cells.setdefault((i, j), set()).add(claim.claim_id)

    part1_result = 0
    for claim_id_set in grid_cells.values():
        if len(claim_id_set) > 1:
            part1_result += 1
            claim_ids.difference_update(claim_id_set)

    assert len(claim_ids) == 1
    part2_result = claim_ids.pop()

    return part1_result, part2_result


print(solve())
