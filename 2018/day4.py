import itertools
import re
import common
from datetime import datetime, timedelta

_INPUT_LINES = common.get_input_lines(day=4)
_ENTRY_REGEX = re.compile(r'\[(?P<timestamp>.+)\] (?P<event>.+)')
_BEGIN_SHIFT_REGEX = re.compile('Guard #(?P<id>\d+) begins shift')


def parse_event_line(line):
    match = _ENTRY_REGEX.match(line)
    assert match
    return datetime.strptime(match.group('timestamp'), '%Y-%m-%d %H:%M'), match.group('event')


def build_timeline():
    timeline = [parse_event_line(line) for line in _INPUT_LINES]
    timeline.sort(key=lambda entry: entry[0])
    return timeline


def build_guard_shift_dict():
    guard_id = None
    shift = None
    guard_to_shifts = {}

    for timestamp, event in build_timeline():
        match = _BEGIN_SHIFT_REGEX.match(event)

        if match:
            if guard_id is not None:
                assert shift is not None
                guard_to_shifts.setdefault(guard_id, []).append(shift)

            guard_id = int(match.group('id'))
            shift = [timestamp]

        elif event in ('falls asleep', 'wakes up'):
            shift.append(timestamp)

        else:
            raise Exception()

    return guard_to_shifts


def compute_shift_sleep_time(shift):
    total = timedelta(minutes=0)
    for i in range(2, len(shift), 2):
        total += shift[i] - shift[i-1]
    return total


def compute_total_sleep_time(shifts):
    return sum(map(compute_shift_sleep_time, shifts), timedelta(minutes=0))


def total_minutes(delta):
    return int(delta.total_seconds()) // 60


def compute_best_minute(shifts):
    sleep_hist = [0] * 60
    for shift in shifts:
        for i in range(2, len(shift), 2):
            sleep_start = shift[i-1].minute
            sleep_end = shift[i].minute

            for j in range(sleep_start, sleep_end):
                sleep_hist[j] += 1

    return max(enumerate(sleep_hist), key=lambda t: t[1])


def solve_part_1():
    guard_to_shifts = build_guard_shift_dict()
    guard_id, shifts = max(guard_to_shifts.items(), key=lambda t: compute_total_sleep_time(t[1]))
    best_minute, sleep_count = compute_best_minute(shifts)
    return guard_id * best_minute


def solve_part_2():
    guard_to_shifts = build_guard_shift_dict()
    guard_to_best_minute = {guard_id: compute_best_minute(shifts) for guard_id, shifts in guard_to_shifts.items()}
    guard_id, (best_minute, sleep_count) = max(guard_to_best_minute.items(), key=lambda t: t[1][1])
    return guard_id * best_minute


print(solve_part_1())
print(solve_part_2())
