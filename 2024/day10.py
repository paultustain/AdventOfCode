from enum import Enum

top_map = """89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"""

top_mafp = """7770777
8881888
1112111
6543456
7111117
8111118
9111119"""

class Direction(Enum):
    UP = "u"
    DOWN = "d"
    LEFT = "l"
    RIGHT = "r"

move_map = {
    "l": (-1, 0),
    "r": (1, 0),
    "u": (0, -1), 
    "d": (0, 1)
}

grid_map = [[s for s in r] for r in top_map.split('\n')]
HEIGHT = len(grid_map)-1
WIDTH = len(grid_map[0])-1


trail_heads = []
for y, c in enumerate(grid_map):
    for x, r in enumerate(c):
        if r == '0':
            trail_heads.append((x, y))

def valid_move(x, y, dir) -> bool:
    dx = move_map[dir][0]
    dy = move_map[dir][1]
    if x == 0 and dx == -1:
        return False
    if y == 0 and dy == -1:
        return False
    if x == WIDTH and dx == 1:
        return False
    if y == HEIGHT and dy == 1:
        return False

    if int(grid_map[y + dy][x + dx]) != (int(grid_map[y][x]) + 1):
        return False
    return True

def complete_trail(x, y):
    count = 0
    curr = grid_map[y][x]
    if int(curr) == 9:
        print("hit 9")
        return 1
    
    for d in move_map:
        if valid_move(x, y, d):
            dx = move_map[d][0]
            dy = move_map[d][1]
            # print("can go ", d, "from ", x, y)
            count += complete_trail(x + dx, y + dy)
    
    return count

final_amount = 0
for th in trail_heads:
    
    head = complete_trail(th[0], th[1])
    final_amount += head
    print(head)
        # complete_trail(th[0], th[1], d, count)
print(final_amount)
# print(len(count)/4)