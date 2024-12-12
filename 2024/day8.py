test_input = """.....wV....q.....................................n
.......w......q.h.....Vn.........................D
............w.S..G.....................DT.........
......S........h......e..T.....y......D...........
......m.......Ae.......T........o.................
....m....S........................................
...m..........................n........8..........
.........2...G......................n.............
..2........V.......h................Q.............
............................o.....................
.Z......I..U....e...u.....G....o..................
...N..G.........................................y.
.....I............q.......h...................s...
......U........qI....o.V..Rz........8........k....
......d.Z.........................R.......8y......
.........e..............T.....l...................
.......2.........................u...R............
.....d.............................Q..............
...................v.....................s.Q....M.
........2..........4.....................8..7.k...
...........x..N..................A..........k.....
...........ZN...........v...............K.........
...d.......N.....................Ky.6.............
...........................l6.....................
....L....g.................4.......k..K.......0...
..............L...........4R................s.....
U......r..............H.4.........................
.......U.............a.......H.............u......
......xY...............l..........................
...................................6..u...........
........Y......L......l............0..............
......9..L...........A.....v..HEa........K........
..................v........6.EX.............z.....
d..Y.............m......A.........................
......................a.i......M...........z......
...................g.......................0......
...............................H.........i........
..........3................W........E...i...0.....
.................t.a....g.................5.......
.r...t...........................7.....5..........
....................................7....5........
....................g.Y...wMz.....................
9..........O....3................W.7..E..XD...1...
t..............3.x.....9..........W.M.............
...9............W.................................
Z.............x................X.i......5.........
...........3.....................................1
...................O.......s....X.................
..............r...................................
..........................O.................1....."""

old_test_input="""............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"""

class Grid():
    def __init__(self, grid_string):
        self.grid = [[s for s in r] for r in grid_string.split('\n')]

    def __repr__(self):
        v = ''
        for c in self.grid:
            for r in c:
                v += r
            v += '\n'
        return v

    
    def add_node(self, x, y, node_mark):
        if x < 0 or y < 0:
            return
 
        try:
            # if self.grid[y][x] != '.':
            #     return
            self.grid[y][x] = node_mark
        except IndexError:
            return


    def clear(self):
        for y, c in enumerate(self.grid):
            for x, r in enumerate(c):
                self.grid[y][x] = '.'

    def find_locations(self, label):
        locations = []
        for y, c in enumerate(self.grid):
            for x, r in enumerate(c):
                if self.grid[y][x] == label:
                    locations.append((x, y))
        return locations

    def count(self, label):
        v = 0
        for c in self.grid:
            for r in c:
                v += 1 if r == label else 0
        return v

def find_distance(node1, node2):
    x = (node2[0] - node1[0])
    y = (node2[1] - node1[1])
    return x, y


def find_antinodes(locations, antinode_map):
    height = len(test_input.split('\n'))
    width = len(test_input.split('\n')[0])
    for i in range(len(locations)):
        for j in range(i+1, len(locations)):
            dx, dy = find_distance(locations[i], locations[j])
            mul = 0
            
            while mul < max(height, width) :
                antinode_map.add_node(locations[i][0] - (dx * mul) , locations[i][1] - (dy * mul), '#')
                antinode_map.add_node(locations[j][0] + (dx * mul) , locations[j][1] + (dy * mul), '#')
                mul += 1


node_map = Grid(test_input)
antinode_map = Grid(test_input)
antinode_map.clear()

antennas = {label: node_map.find_locations(label) for label in set(test_input) if label != '.' and label != '\n'}
print(antennas)
for label, locations in antennas.items():
    find_antinodes(locations, antinode_map)

print(antinode_map)
print(antinode_map.count('#'))
