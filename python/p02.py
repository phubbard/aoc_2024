import peek
from utils import get_data_as_lines

def is_safe(report):
    # Check if levels are either all increasing or all decreasing
    increasing = all(1 <= report[i + 1] - report[i] <= 3 for i in range(len(report) - 1))
    decreasing = all(1 <= report[i] - report[i + 1] <= 3 for i in range(len(report) - 1))
    return increasing or decreasing


def generate_all_reports(report):
    # Generate all reports with one level removed
    for i in range(len(report)):
        yield report[:i] + report[i+1:]


def is_safe_with_damper(report):
    # Check if levels are either all increasing or all decreasing. In part 2, we
    # can now have one bad step before it's unsafe. For each report, we need to try the list as-is,
    # and try removing each level one at a time.
    if is_safe(report):
        return True

    for report in generate_all_reports(report):
        if is_safe(report):
            return True
 
    return False


def count_safe_reports(data, part_one=True):
    safe_count = 0
    for line in data:
        # Convert the line into a list of integers
        report = list(map(int, line.split()))
        if part_one:
            if is_safe(report):
                safe_count += 1
        else:
            if is_safe_with_damper(report):
                safe_count += 1

    return safe_count

# Count the safe reports
sample_data = get_data_as_lines(2, 's')
safe_reports = count_safe_reports(sample_data)
peek(safe_reports)

full_data = get_data_as_lines(2)
safe_reports = count_safe_reports(full_data)
peek(safe_reports)

peek(count_safe_reports(sample_data, part_one=False))
peek(count_safe_reports(full_data, part_one=False))
