import common
import collections

_INPUT_LINES = common.get_input_lines(day=6)
_INPUT_COORDINATES = [tuple(int(s) for s in line.split(', ')) for line in _INPUT_LINES]
BoundingBox = collections.namedtuple('BoundingBox', ('x', 'y', 'width', 'height'))


def compute_bounding_box():
    sel_x = lambda t: t[0]
    sel_y = lambda t: t[1]

    top_left_x = min(map(sel_x, _INPUT_COORDINATES))
    top_left_y = min(map(sel_y, _INPUT_COORDINATES))
    bottom_right_x = max(map(sel_x, _INPUT_COORDINATES))
    bottom_right_y = max(map(sel_y, _INPUT_COORDINATES))
    return BoundingBox(x=top_left_x, y=top_left_y, width=bottom_right_x-top_left_x, height=bottom_right_y-top_left_y)


def compute_zone_sizes():
    occupied = {}
    coords_to_zone_sizes = collections.defaultdict(lambda: 1)

