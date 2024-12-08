import numpy as np 
import re 

def find_horizontal(row):
    
    c = re.findall('XMAS', row)
    f = re.findall('SAMX', row)
    return len(c) + len(f)

def find_vertical(grid):
    count = 0
    for r in range(len(grid[0])-3):
        for c in range(len(grid)):
            word = f"{grid[r][c]}{grid[r+1][c]}{grid[r+2][c]}{grid[r+3][c]}"
            if word == "XMAS" or word == "SAMX":
                print(word)
                count += 1
    return count 

def find_d_r(grid):
    count = 0
    for r in range(len(grid[0])-3):
        for c in range(len(grid)-3):
            word = f"{grid[r][c]}{grid[r+1][c+1]}{grid[r+2][c+2]}{grid[r+3][c+3]}"
            if word == "XMAS" or word == "SAMX":
                print(word)
                count += 1
    return count 

def find_d_l(grid):
    count = 0
    for r in range(3, len(grid[0])):
        for c in range(len(grid)-3):
            word = f"{grid[r][c]}{grid[r-1][c+1]}{grid[r-2][c+2]}{grid[r-3][c+3]}"
            if word == "XMAS" or word == "SAMX":
                print(word)
                count += 1
    return count 


def find_x(grid): 
    count=0
    for r in range(len(grid[0])-2):
        for c in range(len(grid)-2):
            word = f"{grid[r][c]}{grid[r][c+2]}-{grid[r+1][c+1]}-{grid[r+2][c]}{grid[r+2][c+2]}"
            if word == "MM-A-SS" or word == "MS-A-MS" or word == "SS-A-MM"  or word == "SM-A-SM":
                count += 1
    return count


def main():
    with open('test_input_day4.txt', 'r') as f:
        i = f.read()
    grid = i.split('\n')
    # h = 0
    # for row in grid:
    #     h += find_horizontal(row)
    # v = find_vertical(grid)
    # dr = find_d_r(grid)
    # dl = find_d_l(grid)
    print(find_x(grid))
    # print(np.transpose(grid))
    # print(h + v + dr + dl)

main()
