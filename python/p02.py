import peek
from utils import get_data_as_lines

def is_safe(report):
    # Check if levels are either all increasing or all decreasing
    increasing = all(1 <= report[i + 1] - report[i] <= 3 for i in range(len(report) - 1))
    decreasing = all(1 <= report[i] - report[i + 1] <= 3 for i in range(len(report) - 1))
    return increasing or decreasing

def count_safe_reports(data):
    safe_count = 0
    for line in data:
        # Convert the line into a list of integers
        report = list(map(int, line.split()))
        if is_safe(report):
            safe_count += 1
    return safe_count

# Count the safe reports
sample_data = get_data_as_lines(2, 's')
safe_reports = count_safe_reports(sample_data)
peek(safe_reports)
full_data = get_data_as_lines(2)
safe_reports = count_safe_reports(full_data)
peek(safe_reports)