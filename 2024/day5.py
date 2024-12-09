def check_order(update, order):
    update_list = update.split(',')
    for i in range(len(update_list)):
        for j in range(i+1, len(update_list)):
            if f"{update_list[j]}|{update_list[i]}" in order:
                return False
    return True


def reorder(update_list, order):
    # bubble sort for ease, could be better
    swap = True
    end = len(update_list)
    while swap:
        swap = False
        for i in range(1, end):
            if f"{update_list[i]}|{update_list[i-1]}" in order:
                temp = update_list[i]
                update_list[i] = update_list[i-1]
                update_list[i-1] = temp
                swap = True
        end -= 1
    return update_list


def main():
    with open('test_input_day5.txt', 'r') as f:
        i = f.read()

    order = i.split('\n\n')[0].split('\n')
    updates = i.split('\n\n')[1].split('\n')

    total = 0
    for update in updates:
        if check_order(update, order):
            update_list = update.split(',')
            # total += int(update_list[len(update_list) // 2])
        else:
            pass
            update_list = update.split(',')
            new_list = reorder(update_list, order)
            total += int(new_list[len(update_list) // 2])
    print(total)

main()