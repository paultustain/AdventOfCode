test_input = """##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"""


class Robot():
    def __init__(self, x, y):
        self.x = x
        self.y = y 

    def __repr__(self):
        return f"Robot({self.x}, {self.y})"



class Grid():
    MOVES = {
        "^": {"dx": 0, "dy": -1}, 
        ">": {"dx": 1, "dy": 0}, 
        "v": {"dx": 0, "dy": 1}, 
        "<": {"dx": -1, "dy": 0}, 
    }

    DOUBLE_MAP = {
        '#': ['#', '#'],
        '.': ['.', '.'], 
        'O': ['[', ']'],
        '@': ['@', '.'] 
    }

    def __init__(self, grid):
        self.grid = [[v for v in row] for row in grid.split('\n')]
        x, y = self.find_robot()
        self.robot = Robot(x, y)


    def __repr__(self):
        v = ''
        for r in self.grid:
            for c in r:
                v += c
            v += '\n'
        return v

    def find_robot(self):
        for y, r in enumerate(self.grid):
            for x, c in enumerate(r):
                if c == '@':
                    return x, y


    def find_end(self, item_x, item_y, direction):
       
        dx = self.MOVES[direction]['dx']
        dy = self.MOVES[direction]['dy']
        match self.grid[item_y + dy][item_x + dx]:
            case '#':
                return None, None
            case 'O':
                return self.find_end(item_x + dx, item_y + dy, direction)
            case '.':

                return item_x+dx, item_y+dy


    def move_robot(self, direction):
                
        dx = self.MOVES[direction]['dx']
        dy = self.MOVES[direction]['dy']
        end_x, end_y = self.find_end(self.robot.x, self.robot.y, direction)

        if end_x is not None and end_y is not None:
            while not((end_x == self.robot.x) and (end_y == self.robot.y)):
                self.grid[end_y][end_x] = self.grid[end_y - dy][end_x - dx]
                end_x -= dx
                end_y -= dy
            
            self.grid[self.robot.y][self.robot.x] = '.'
            self.robot.x += dx
            self.robot.y += dy
            self.grid[self.robot.y][self.robot.x] = '@'

    def sum_boxes(self):
        final_total = 0
        for y, r in enumerate(self.grid):
            for x, c in enumerate(r):
                if c == 'O':
                    print(x, y)

                    final_total += (100 * y) + x
         
        return final_total


class Grid2():
    MOVES = {
        "^": {"dx": 0, "dy": -1}, 
        ">": {"dx": 1, "dy": 0}, 
        "v": {"dx": 0, "dy": 1}, 
        "<": {"dx": -1, "dy": 0}, 
    }

    DOUBLE_MAP = {
        '#': ['#', '#'],
        '.': ['.', '.'], 
        'O': ['[', ']'],
        '@': ['@', '.'] 
    }

    def __init__(self, grid):
        self.grid = self.create_grid(grid)
        x, y = self.find_robot()
        self.robot = Robot(x, y)


    def __repr__(self):
        v = ''
        for r in self.grid:
            for c in r:
                v += c
            v += '\n'
        return v
    
    
    def find_robot(self):
        for y, r in enumerate(self.grid):
            for x, c in enumerate(r):
                if c == '@':
                    return x, y
    

    def create_grid(self, grid_input):
        full = []
        for line_input in grid_input.split('\n'):
            line = []
            for c in line_input:
                line.extend(self.DOUBLE_MAP[c])
            full.append(line)
        return full

    def find_end(self, item_x, item_y, direction):
       
        dx = self.MOVES[direction]['dx']
        dy = self.MOVES[direction]['dy']

        match self.grid[item_y + dy][item_x + dx]:
            case '#':
                return None, None
            case '[' | ']':
                return self.find_end(item_x + dx, item_y + dy, direction)
            case '.':

                return item_x+dx, item_y+dy


    def move_robot(self, direction):
                
        dx = self.MOVES[direction]['dx']
        dy = self.MOVES[direction]['dy']
        end_x, end_y = self.find_end(self.robot.x, self.robot.y, direction)
        last_pushed = None

        if end_x is not None and end_y is not None:
            if dy == 0:
                # Not gonna change left and right as it worked for part 1 
                while not((end_x == self.robot.x) and (end_y == self.robot.y)):
                    self.grid[end_y][end_x] = self.grid[end_y - dy][end_x - dx]
                    end_x -= dx
                    end_y -= dy
                
                self.grid[self.robot.y][self.robot.x] = '.'
                self.robot.x += dx
                self.robot.y += dy
                self.grid[self.robot.y][self.robot.x] = '@'
            else:
                # uppy downy changes 
                while not(end_y == self.robot.y):
                    push_char = self.grid[end_y - dy][end_x - dx]
                    if push_char != '@':
                        last_pushed = push_char
                    if push_char == '[':
                        x_adj = 1 
                    elif push_char == ']':
                        x_adj = -1
                    else:
                        x_adj = 0
                    
                    self.grid[end_y][end_x] = self.grid[end_y - dy][end_x]
                    if push_char == '[' or push_char == ']':
                        self.grid[end_y][end_x + x_adj] = self.grid[end_y - dy][end_x + x_adj]
                    end_x -= dx
                    end_y -= dy

                self.grid[self.robot.y][self.robot.x] = '.'
                self.robot.x += dx
                self.robot.y += dy
                self.grid[self.robot.y][self.robot.x] = '@'
                # last_pushed = self.grid[self.robot.y + dy][self.robot.x]
                # print(last_pushed)

                if last_pushed == '[':
                    self.grid[self.robot.y][self.robot.x + 1 ] = '.'
                elif last_pushed == ']':
                    self.grid[self.robot.y][self.robot.x - 1 ] = '.'

def day1():
    with open('input_day15.txt', 'r') as f:
        test_input = f.read()

    grid_input, instructions = test_input.split('\n\n')


    grid = Grid(grid_input)

    for i in instructions:
        if i == "^" or i == ">" or i == "<" or i == "v":
            grid.move_robot(i)

    print(grid.sum_boxes())
        
def day2():
    grid_input, instructions = test_input.split('\n\n')

    grid = Grid2(grid_input)

    print(grid)
    c = 1
    for i in instructions:
        if i == "^" or i == ">" or i == "<" or i == "v":
            print(i, c)
            grid.move_robot(i)
            print(grid)
            c+=1
            # if c == 25:
            #     break
        
    # print(grid)
day2()
