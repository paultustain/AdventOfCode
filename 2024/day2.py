def check_sorted(line):
    return (sorted(line) == line) or (sorted(line, reverse=True) == line)


def check_diffs(line):
    diffs = [abs(line[i] - line[i+1]) for i in range(len(line) -1)]
    return all([(d>=1) and (d<=3) for d in diffs]) 


def overall_check(line):
    if not(check_sorted(line)):
        return False
    if not(check_diffs(line)):
        return False
    return True


def main():
    with open("input_day2.txt","r") as f:
        input_lines = f.read().split('\n')
    lines = [[int(l) for l in il.strip().split(' ')] for il in input_lines if il != '']
    count = 0
    for l in lines:
        if overall_check(l):
            count += 1
        else: 
            for idx in range(len(l)):
                print(l)

                new_list = l.copy()
                del new_list[idx]
                if overall_check(new_list):
                    count += 1
                    break

    print(count)
    
main()
