import re

def find_value(string):
    string = string[4:-1]
    values = string.split(',')
    return int(values[0]) * int(values[1])


def split(string, on):
    output_list = []

    if on:
        str_split = string.split("don't()", 1)
        output_list.append(str_split[0])
        if "do()" in str_split[1]:
            output_list.append(split(str_split[1], False))
        else:
            return output_list
    else:
        str_split = string.split("do()", 1)
        if "don't()" in str_split[1]:
            output_list.append(split(str_split[1], True))
        else:
            return output_list
    



with open('input_day3.txt', 'r') as f:
    # input_string = f.read()
    input_string = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+do()mul(32,64)don't().(mul(11,8)undo()?mul(8,5))" 

# part 1 
# values = re.findall('mul\(\d{1,3},\d{1,3}\)', input_string)

print(split(input_string, True))

