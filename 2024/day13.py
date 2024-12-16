test_input = """Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"""

with open('input_day13.txt', 'r') as f:
    puzzle_input = f.read()

def alg_solve(prize, button_a, button_b):
    x1 = button_a['X']
    x2 = button_b['X']
    x = prize['X']
    y1 = button_a['Y']
    y2 = button_b['Y']
    y = prize['Y']

    if x1*y2 == x2*y1:
        '''
        Check for div 0 error
        '''
        return (0, 0)

    b = (x1*y - x*y1) / (x1*y2 - x2 * y1)
    if b != ((x1*y - x*y1) // (x1*y2 - x2 * y1)):
        return (0, 0)
    
    a = ((x - b * x2)) / x1
    if a != (((x - b * x2)) / x1):
        return (0, 0)
    
    return (a, b)


def find_value(line, str):
    value = ''
    split = line.split(str)[1]
    for c in split:    
        try:
            int(c)
            value += c
        except ValueError:
            return int(value)
    return int(value)


machines = puzzle_input.split('\n\n')

sum_cost = 0

for machine in machines:
    button_a, button_b, prize = machine.split('\n')
    
    button_a_dict = {
        'X': find_value(button_a, 'X+'), 
        'Y': find_value(button_a, 'Y+')
    } 
    button_b_dict = {
        'X': find_value(button_b, 'X+'), 
        'Y': find_value(button_b, 'Y+')
    } 
    # Find cheapest way to get close to 10000000000000
    prize_dict = {
        'X': 10000000000000 + find_value(prize, 'X='), 
        'Y': 10000000000000 + find_value(prize, 'Y=')
    } 
    pushes = alg_solve(prize_dict, button_a_dict, button_b_dict)
    sum_cost += 3*pushes[0] + pushes[1]

    
print(sum_cost)