with open('input_day9.txt', 'r') as f:
    input_string = f.read()

class FileSystem():
    def __init__(self):
        self.system = []
        self.empty_spaces = {}

    def __repr__(self):
        out = ''
        for b in self.system:
            out += str(b)
        return out

    def find_all_blanks(self):
        in_blank = False
        blank_id = 0
        empty = {}
        for i, b in enumerate(self.system):
            if (b.id is None) and not(in_blank):
                start = i
                in_blank = True
            if (b.id is not None) and in_blank:
                length = i - start
                empty[str(blank_id)] = {'start': start, 'length': length}
                blank_id += 1
                in_blank = False

        self.empty_spaces = empty


    def add_block(self, block):
        self.system.append(block)

    def find_first_blank(self, start):
        for i in range(start, len(self.system)):
            if self.system[i].id is None:
                return i 
    
    def find_last_item(self, start):
        for i in range(start, 0, -1):
            if self.system[i].id is not None:
                return i 

    def find_length(self, id, start, end, step):
        for l in range(start, end, step):
            if self.system[l].id != id:
                return abs(l - start) + step if step == 1 else abs(l - start)


    def sort_fs(self):
        first_blank = self.find_first_blank(0)
        last_item = self.find_last_item(len(self.system) - 1)
        while first_blank < last_item:
            self.system[first_blank], self.system[last_item] = self.system[last_item], self.system[first_blank]
            first_blank = self.find_first_blank(first_blank + 1)
            last_item = self.find_last_item(last_item - 1)


    def sort_fs_full_files(self):
        moved = []
        last_item = self.find_last_item(len(self.system) - 1)
        last_item_length = self.find_length(self.system[last_item].id, last_item, 0, -1)
        
        while self.system[last_item].id > 0:
            
            if self.system[last_item].id not in moved:
                item_start = last_item - last_item_length + 1
                for space in self.empty_spaces.values():
                    if space['start'] > item_start:
                        break
                    if space['length'] >= last_item_length:
                        moved.append(self.system[last_item].id)
                        for x in range(last_item_length):
                            self.system[space['start'] + x], self.system[last_item - x] = self.system[last_item - x], self.system[space['start'] + x]
                        space['start'] += last_item_length
                        space['length'] -= last_item_length
                        break

            last_item = self.find_last_item(last_item - last_item_length)
            last_item_length = self.find_length(self.system[last_item].id, last_item, 0, -1)
        
    def checksum(self):
        sum_total = 0
        for i, b in enumerate(self.system):
            if b.id is None:
                sum_total += 0
            else:
                sum_total += i * int(b.id)
        return sum_total


class Block():
    def __init__(self, id):
        self.id = id
    
    def __eq__(self, other):
        return self.id == other.id


    def __repr__(self):
        if self.id is None: 
            return "."
        return f"{self.id}"


file = True
fs = FileSystem()

id = 0
for c in input_string:
    if file:
        block = Block(id)
        id += 1
    else:
        block = Block(None)

    for _ in range(int(c)):
        fs.add_block(block)
    
    file = not(file)

def day1():

    fs.sort_fs()
    print(fs.checksum())

def day2():
    fs.find_all_blanks()
    fs.sort_fs_full_files()
    print(fs.checksum())
    
day2()