import itertools

def test_values(answer, inputs):
    n = len(inputs) - 1 
    # print(len(inputs), inputs)
    searches = list(itertools.product('+*|', repeat = n))
    for symbol_options in searches:
        total = int(inputs[0])
        # print(total)
        for i in range(1, len(inputs)):
            if symbol_options[i-1] == '*':
                total *= int(inputs[i])
            elif symbol_options[i-1] == '+':
                total += int(inputs[i])
            elif symbol_options[i-1] == '|':
                total = int(str(total) + inputs[i])
        if total == int(answer):
            return int(answer)
        
    return 0

with open('input_day7.txt', 'r') as f:
    inp = f.read()

calibrations = {x.split(": ")[0]: x.split(": ")[1] for x in inp.split('\n')}

total = 0   
for answer, inputs in calibrations.items():
    # if test_values(answer, inputs.split(' ')) == int(answer):
    total += test_values(answer, inputs.split(' '))

print(total)