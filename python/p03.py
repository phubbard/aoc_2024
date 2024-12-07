import re

import peek
from utils import get_data_as_lines
pattern = r"mul\((-?\d{1,3}),\s*(-?\d{1,3})\)"

do_dont_pattern = r"do\(\)(.*?)don't\(\)"

def part_one(data):
    total_sum = 0
    for line in data:
        matches = re.findall(pattern, line)
        assert(len(matches) > 0)

        for match in matches:
            total_sum += int(match[0]) * int(match[1])

    return total_sum

def part_two(data):
    total_sum = 0
    data = ["do()", *data, "don't()"]
    big_line = ''.join(data)
    dd_matches = re.findall(do_dont_pattern, big_line)
    assert(len(dd_matches) > 0)
    for dd_match in dd_matches:
        matches = re.findall(pattern, dd_match)
        assert(len(matches) > 0)

        for match in matches:
            total_sum += int(match[0]) * int(match[1])

    return total_sum

sample_data = get_data_as_lines(3, 's')
peek(part_one(sample_data))
peek(part_one(get_data_as_lines(3)))
peek(part_two(["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"]))
peek(part_two(get_data_as_lines(3)))
