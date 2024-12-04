def create_lists():
    with open('input_day1.txt', 'r') as f:
        lines = []
        for line in f:
            lines.append((line.strip().split('   ')))
    list1 = sorted([int(l[0]) for l in lines])
    list2 = sorted([int(l[1]) for l in lines])
    return list1, list2

def main():
    l1, l2 = create_lists()
    
    # Part 1.
    # diff = [abs(x1 - x2) for x1, x2 in zip(l1, l2)]
    #print(sum(diff))
    
    # Part 2 
    similarity_list = [x1 * l2.count(x1) for x1 in l1]
    print(sum(similarity_list))

if __name__ == '__main__':
    main()
