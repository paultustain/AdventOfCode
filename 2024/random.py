inputs = [1, 2, 3, 4]
final = []
final.append(inputs[1:])

add_list = [inputs[0]]
add_list.extend(inputs[2:])
print(add_list)
final.append(add_list)
print(final)