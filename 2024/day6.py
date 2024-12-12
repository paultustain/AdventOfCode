from enum import Enum 
from copy import deepcopy 


class Direction(Enum):
    UP = "up"
    DOWN = "down"
    LEFT = "left"
    RIGHT = "right"

class Grid():
    def __init__(self, width, height):
        self.width = width
        self.height = height
        self.grid = [['.' for _ in range(height)] for _ in range(width)]

    def __repr__(self):
        v = ''
        for x in range(len(self.grid)):
            for y in range(len(self.grid)):
                v += self.grid[x][y]
            v += '\n'
        return v

    def mark_cell(self, x, y, mark='X'):
        self.grid[y][x] = mark

    def find_items(self, x, y):
        return self.grid[y][x]
    
    def count_type(self, type='X'):
        v = 0
        for x in range(len(self.grid)):
            for y in range(len(self.grid[0])):
                v += 1 if self.grid[y][x] == type else 0
        return v 


class Guard():
    def __init__(self, x, y, direction):
        self.initial_x = x
        self.initial_y = y
        self.x = x
        self.y = y
        self.direction = direction
        self.inside_grid = True 
        self.is_looping = False
        self.visited = []

    def reset(self):
        self.x = self.initial_x
        self.y = self.initial_y
        self.inside_grid = True
        self.is_looping = False
        self.direction = Direction.UP
        self.visited = []


    def is_loop(self):
        return (self.x, self.y, self.direction) in self.visited


    def take_steps(self, dx, dy, grid):
        moving = True 

        while moving:
            # print(grid)
            grid.mark_cell(self.x, self.y)
            
            if self.is_loop():
                print("Looping")
                moving = False
                self.is_looping = True

            self.visited.append((self.x, self.y, self.direction))
            if self.x == 0 or self.y == 0 or self.x == grid.width - 1 or self.y == grid.height - 1:
                moving = False 
                self.inside_grid = False 
            elif grid.find_items(self.x + dx, self.y + dy) == '#':
                moving = False 
            else:
                self.x += dx 
                self.y += dy


    def move(self, grid):
        if self.direction == Direction.UP:
            self.take_steps(0, -1, grid)
            self.direction = Direction.RIGHT

        elif self.direction == Direction.DOWN:
            self.take_steps(0, 1, grid)
            self.direction = Direction.LEFT
        
        elif self.direction == Direction.RIGHT:
            self.take_steps(1, 0, grid)
            self.direction = Direction.DOWN
        
        elif self.direction == Direction.LEFT:
            self.take_steps(-1, 0, grid)
            self.direction = Direction.UP


def find_location(grid):
    for i, r in enumerate(grid):
        if '^' in r:
            for j in range(len(r)):
                if r[j] == '^':
                    return j, i


with open('input_day6.txt', 'r') as f:
    input_grid = f.read().split('\n')

x, y = find_location(input_grid)
guard = Guard(x, y, Direction.UP)

def make_grid():
    grid_map = Grid(len(input_grid), len(input_grid[0]))
    # clean_grid_map = Grid(len(input_grid), len(input_grid[0]))

    for i, r in enumerate(input_grid):
        for j, c in enumerate(r):
            grid_map.mark_cell(j, i, c)
            # clean_grid_map.mark_cell(j, i, c)
    return grid_map
grid_map = make_grid()

while guard.inside_grid:
    guard.move(grid_map)

possible_locations = []
for y in range(grid_map.width):
    for x in range(grid_map.height):
        if grid_map.find_items(x, y) == 'X':
            possible_locations.append((x, y))

# print(clean_grid_map)
loops = 0
guard.reset()

for location in possible_locations:
    guard.reset()
    curr_grid = make_grid()
    curr_grid.mark_cell(location[0], location[1], '#')
    # print(curr_grid)
    # print(location)
    
    # print(guard.inside_grid, not(guard.is_looping))
    while guard.inside_grid and not(guard.is_looping):
        guard.move(curr_grid)
    # print(curr_grid)
    if guard.is_looping:
        loops += 1

print(loops)
#     if location != (guard.initial_x, guard.inital_y):
#         pass

# print(grid_map)
# print(grid_map.count_type())
