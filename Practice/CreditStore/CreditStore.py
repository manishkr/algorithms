import sys

def Process(credit, items):
    index1, index2 = -1, -1,
    for elem in items:
        if elem < credit :
            index1 = items.index(elem)
            try:
                index2 = index1 + 1 + items[(index1+1):].index(credit - elem)
                if index2 > -1:
                    break
            except:
                continue

    ret = str(index1 + 1) + ' ' + str(index2 + 1)
    
    return ret

def CreditScore(inFile, outFile):
    i = 0
    credit = 0;
    with open(inFile, 'r') as oFile, open(outFile, 'w') as wFile:
        for line in oFile:
            if i == 0:
                i +=1
            else:
                if i % 3 == 0:
                    items = map(int, line.split())
                    ret = Process(credit, items)
                    wFile.write("Case #" + str(i / 3) + ": " + ret + '\n')
                elif (i-1) % 3 == 0:
                    credit = int(line)

                i +=1

if __name__ == '__main__':
    if len(sys.argv) > 2:
        CreditScore(sys.argv[1], sys.argv[2])
