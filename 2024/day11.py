from functools import cache

class Stones():
    def __init__(self, data):
        self.stones = [int(value) for value in data.split(' ')]

    def stone_blink(stone):
        if stone == 0:
            return [1]
        elif len(str(stone)) % 2 == 0:
            stone_a = int(str(stone)[ :len(str(stone)) // 2])
            stone_b = int(str(stone)[len(str(stone)) // 2: ])
            return [stone_a, stone_b]
        else:
            return [2024 * stone]

    def total_count(self, blinks):
        count = 0 
        for stone in self.stones:
            count += self.count_stones(stone, blinks)
        return count 

    @staticmethod
    @cache
    def count_stones(stone, blinks):
        if blinks == 0:
            return 1
        count = 0
        new_stones = Stones.stone_blink(stone)
        for new_stone in new_stones:
            count += Stones.count_stones(new_stone, blinks - 1)
        return count 

inp = "41078 18 7 0 4785508 535256 8154 447"
stones = Stones(inp)

# day1
print(stones.total_count(25))

# day2
print(stones.total_count(75))