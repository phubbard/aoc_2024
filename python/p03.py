import re

import peek
from utils import get_data_as_lines
pattern = r"mul\((-?\d{1,3}),\s*(-?\d{1,3})\)"

def part_one(data):
    total_sum = 0
    for line in data:
        matches = re.findall(pattern, line)
        assert(len(matches) > 0)

        for match in matches:
            total_sum += int(match[0]) * int(match[1])

    return total_sum

sample_data = get_data_as_lines(3, 's')
peek(part_one(sample_data))
peek(part_one(get_data_as_lines(3)))
