#!/usr/bin/env python3

# pfh 12/1/2024 Yay advent of code! Let's get started.
# https://adventofcode.com/2024/day/1

from loguru import logger
import pandas as pd

from utils import make_data_filenames
log = logger

def read_data_pd(day: int) -> tuple[pd.DataFrame, pd.DataFrame]:    
    # Read the files with a two columns
    sample_data = pd.read_csv(f'data/{day:02d}s.txt', sep=r'\s+', header=None, names=['value1', 'value2'])
    full_data = pd.read_csv(f'data/{day:02d}.txt', sep=r'\s+', header=None, names=['value1', 'value2'])
    return sample_data, full_data

def part_one(data: pd.DataFrame) -> None:
    # Extract the value1 and value2 columns into separate python lists
    value1 = data['value1'].tolist()
    value2 = data['value2'].tolist()
    value1.sort()
    value2.sort()

    total = sum([abs(element[0] - element[1]) for element in zip(value1, value2)])
    print(f'Part one sum: {total}')

if __name__ == '__main__':
    sample_data, full_data = read_data_pd(1)
    part_one(sample_data)
    part_one(full_data)
#    log.info(f'Sample data: {sample_data}')
#    log.info(f'Full data: {full_data}')
