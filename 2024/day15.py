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


class Box():
    def __init__(self, x, y):
        self.x = x
        self.y = y
    
    def __repr__(self):
        return f"Row: {self.y} Col: {self.x}"



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

    def __init__(self, grid, part):
        self.boxes = []
        if part == 1:
            self.grid = [[v for v in row] for row in grid.split('\n')]
        elif part == 2:
            self.grid = self.create_grid(grid)
        x, y = self.find_robot()
        self.robot = Robot(x, y)
        self.boxes = []


    def __repr__(self):
        v = ''
        for r in self.grid:
            for c in r:
                v += c
            v += '\n'
        return v


    def create_grid(self, grid_input):
        full = []
        for y, line_input in enumerate(grid_input.split('\n')):
            line = []
            for x, c in enumerate(line_input):
                if c == 'O':
                    self.boxes.append(Box(x*2, y))
                line.extend(self.DOUBLE_MAP[c])
            full.append(line)
        return full


    def find_robot(self):
        for y, r in enumerate(self.grid):
            for x, c in enumerate(r):
                if c == '@':
                    return x, y


    # def find_boxes(self):
    #     boxes = []
    #     for y, r in enumerate(self.grid):
    #         for x, c in enumerate(r):
    #             if c == 'O':
    #                 boxes.append(Box(x, y))
    #     return boxes

    def move(self, item_x, item_y, dx, dy):
        # print(self.grid[item_y][item_x])
        self.grid[item_y + dy][item_x + dx] = self.grid[item_y][item_x]
        self.grid[item_y][item_x] = '.'

    def move_item(self, item_x, item_y, dx, dy):
        moved = True
        current_space = self.grid[item_y][item_x]
        next_space = self.grid[item_y + dy][item_x + dx]
        
        if next_space == '#':
            # if can't move - return nothing
            # print("Dont move", current_space)
            return False
        
        # if next_space == '.':
        #     # self.move(item_x, item_y, dx, dy)
        #     moves.append((item_x, item_y))
        #     # return moves
        if (next_space == 'O') | (next_space == '[') | (next_space == ']')  :
            # moves.append((item_x, item_y))
            if dy == 0:

                moved = self.move_item(item_x + dx, item_y + dy, dx, dy)
            else: 
                print("uppy downy box")
            # moves.append((item_x, item_y))

        if moved:
            # print(f"move {item_x}, {item_y} in ({dx}, {dy}) to ({item_x + dx}, {item_y + dy})" )
            self.move(item_x, item_y, dx, dy)
            if current_space == '@':
                self.robot.x += dx
                self.robot.y += dy
            return moved
        else:
            return moved
        
        

        # match self.grid[item_y + dy][item_x + dx]:
        #     case '#':
        #         return None, None
        #     case 'O':
        #         return self.find_end(item_x + dx, item_y + dy, direction)
        #     case '.':
        #         return item_x+dx, item_y+dy


    def move_robot(self, direction):
         

        start_x, start_y = self.robot.x, self.robot.y
        dx = self.MOVES[direction]['dx']
        dy = self.MOVES[direction]['dy']
        moves = []
        self.move_item(start_x, start_y, dx, dy)

        # for move in moves

    def sum_boxes(self):
        final_total = 0
        for y, r in enumerate(self.grid):
            for x, c in enumerate(r):
                if c == 'O':
                    final_total += (100 * y) + x
         
        return final_total


# test_input ="""########
# #..O.O.#
# ##@.O..#
# #...O..#
# #.#.O..#
# #...O..#
# #......#
# ########

# <^^>>>vv<v>>v<<"""

def day1():
    with open('input_day15.txt', 'r') as f:
        test_input = f.read()

    grid_input, instructions = test_input.split('\n\n')

    grid = Grid(grid_input, 1)
    # print(grid)
    # c = 1
    for i in instructions:
        if i == "^" or i == ">" or i == "<" or i == "v":
            grid.move_robot(i)
            # c += 1
            # print(i)
            # print(grid)
            # if c == 10: 
            #     break
    # print(grid)
    print(grid.sum_boxes())
        
def day2():
    test_input = """#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^""" 

    grid_input, instructions = test_input.split('\n\n')

    grid = Grid(grid_input, 2)

    print(grid)

    #c = 1
    for i in instructions:
        if i == "^" or i == ">" or i == "<" or i == "v":
            print(i)
            grid.move_robot(i)
            print(grid)
            # c+=1
            # if c == 25:
            #     break
        
    print(grid)
day2()





























# class Grid2():
#     MOVES = {
#         "^": {"dx": 0, "dy": -1}, 
#         ">": {"dx": 1, "dy": 0}, 
#         "v": {"dx": 0, "dy": 1}, 
#         "<": {"dx": -1, "dy": 0}, 
#     }

#     DOUBLE_MAP = {
#         '#': ['#', '#'],
#         '.': ['.', '.'], 
#         'O': ['[', ']'],
#         '@': ['@', '.'] 
#     }

#     def __init__(self, grid):
#         self.boxes = []
#         self.grid = self.create_grid(grid)
#         x, y = self.find_robot()
#         self.robot = Robot(x, y)

#     def __repr__(self):
#         v = ''
#         for r in self.grid:
#             for c in r:
#                 v += c
#             v += '\n'
#         return v
    
    
#     def find_robot(self):
#         for y, r in enumerate(self.grid):
#             for x, c in enumerate(r):
#                 if c == '@':
#                     return x, y
    

#     def create_grid(self, grid_input):
#         full = []
#         for y, line_input in enumerate(grid_input.split('\n')):
#             line = []
#             for x, c in enumerate(line_input):
#                 if c == 'O':
#                     self.boxes.append(Box(x*2, x*2+1, y))
#                 line.extend(self.DOUBLE_MAP[c])
#             full.append(line)
#         return full

#     '''
#     def find_end(self, item_x, item_y, direction):
       
#         dx = self.MOVES[direction]['dx']
#         dy = self.MOVES[direction]['dy']

#         match self.grid[item_y + dy][item_x + dx]:
#             case '#':
#                 return None, None
#             case '[' | ']':
#                 return self.find_end(item_x + dx, item_y + dy, direction)
#             case '.':

#                 return item_x+dx, item_y+dy


#     def move_robot(self, direction):
                
#         dx = self.MOVES[direction]['dx']
#         dy = self.MOVES[direction]['dy']
#         end_x, end_y = self.find_end(self.robot.x, self.robot.y, direction)
#         last_pushed = None

#         if end_x is not None and end_y is not None:
#             if dy == 0:
#                 # Not gonna change left and right as it worked for part 1 
#                 while not((end_x == self.robot.x) and (end_y == self.robot.y)):
#                     self.grid[end_y][end_x] = self.grid[end_y - dy][end_x - dx]
#                     end_x -= dx
#                     end_y -= dy
                
#                 self.grid[self.robot.y][self.robot.x] = '.'
#                 self.robot.x += dx
#                 self.robot.y += dy
#                 self.grid[self.robot.y][self.robot.x] = '@'
#             else:
#                 # uppy downy changes 
#                 while not(end_y == self.robot.y):
#                     push_char = self.grid[end_y - dy][end_x - dx]
#                     if push_char != '@':
#                         last_pushed = push_char
#                     if push_char == '[':
#                         x_adj = 1 
#                     elif push_char == ']':
#                         x_adj = -1
#                     else:
#                         x_adj = 0
                    
#                     self.grid[end_y][end_x] = self.grid[end_y - dy][end_x]
#                     if push_char == '[' or push_char == ']':
#                         self.grid[end_y][end_x + x_adj] = self.grid[end_y - dy][end_x + x_adj]
#                     end_x -= dx
#                     end_y -= dy

#                 self.grid[self.robot.y][self.robot.x] = '.'
#                 self.robot.x += dx
#                 self.robot.y += dy
#                 self.grid[self.robot.y][self.robot.x] = '@'
#                 # last_pushed = self.grid[self.robot.y + dy][self.robot.x]
#                 # print(last_pushed)

#                 if last_pushed == '[':
#                     self.grid[self.robot.y][self.robot.x + 1 ] = '.'
#                 elif last_pushed == ']':
#                     self.grid[self.robot.y][self.robot.x - 1 ] = '.'
#     '''
