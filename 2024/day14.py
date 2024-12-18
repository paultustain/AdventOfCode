import matplotlib.pyplot as plt
import numpy as np 
from matplotlib.widgets import Slider

HEIGHT = 103
WIDTH = 101
# HEIGHT = 7
# WIDTH = 11 

with open("input_day14.txt", 'r') as f:
    test_input = f.read()

class Robot():
    def __init__(self, x, y, vel_x, vel_y):
        self.x = x
        self.y = y
        self.vel_x = vel_x
        self.vel_y = vel_y


    def __repr__(self):
        return f"Robot({self.x}, {self.y}, {self.vel_x}, {self.vel_y})"


    def move(self):
        self.x = (self.x + self.vel_x) % WIDTH
        self.y = (self.y + self.vel_y) % HEIGHT


class Grid():
    def __init__(self):
        self.map = [[0 for w in range(WIDTH)] for h in range(HEIGHT)]
        self.robots = []
    

    def __repr__(self):
        v = ''
        for y in range(HEIGHT):
            for x in range(WIDTH):
                robots_square = [r for r in self.robots if r.x == x and r.y == y]
                if len(robots_square) > 0:
                    v += '#' # str(len(robots_square))
                else:
                    v += ' ' #str(self.map[y][x])
            v += '\n'
        return v


    def add_robots(self):
        for r in self.robots:
            self.map[r.y][r.x] += 1


    def update_robots(self, moves):
        for _ in range(moves):
            for r in self.robots:
                self.map[r.y][r.x] -= 1
                r.move()
                self.map[r.y][r.x] += 1

        return [[r.x for r in self.robots], [-r.y for r in self.robots]]

    def sum_quad(self, h_start, h_stop, v_start, v_stop):
        count = 0
        for h in range(h_start, h_stop):
            for v in range(v_start, v_stop):
                count += self.map[h][v]
        return count

    def calculate_quadrants(self):
        h_split = HEIGHT // 2
        v_split = WIDTH // 2
        
        q1 = self.sum_quad(0, h_split, 0, v_split)
        q2 = self.sum_quad(0, h_split, v_split + 1, WIDTH)
        q3 = self.sum_quad(h_split+1, HEIGHT, 0, v_split)
        q4 = self.sum_quad(h_split+1, HEIGHT,  v_split + 1, WIDTH)

        return q1 * q2 * q3 * q4

def create_robots():
    robots = []
    lines = test_input.split('\n')
    for line in lines:
        position, velocity = line.split(' ')
        pos_x, pos_y = position[2:].split(',')
        vel_x, vel_y = velocity[2:].split(',')

        robots.append(Robot(int(pos_x), int(pos_y), int(vel_x), int(vel_y)))
    return robots


def check_tree(grid):
    for line in grid.map:
        for subset_start in range(len(line) - 7):
            x = [1 for i in line[subset_start: subset_start + 7] if i > 0]
            if len(x) == 7:
                return True
    return False

robots = create_robots()

grid = Grid()
grid.robots = robots 
grid.add_robots()

i = 1
RUN_LIMIT = 1000000
while True:
    grid.update_robots(1)
    if check_tree(grid):
        print(f"# --- {i} --- #")
        print(grid)

    if i == RUN_LIMIT:
        break
    i += 1

# print(grid)
# day 1
# Set run limit to 100 
print(grid.calculate_quadrants())

# day2
# set run limit to 100000
