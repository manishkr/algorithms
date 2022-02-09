import sys
import itertools

def CalculateProduct(v1, v2):
    sum = 0
    for e1, e2 in itertools.izip_longest(v1, v2):
        sum += e1 * e2

    return sum

def CalculateMinProduct(v1, v2):
    vs1 = sorted(v1)
    vs2 = sorted(v2, reverse=True)
    
    return CalculateProduct(vs1, vs2)

def MinimumScalarProduct(inFile, outFile):
    with open(inFile, 'r') as oFile, open(outFile, 'w') as wFile:
        oFile.readline()
        v1 = []
        i = 1
        for line in oFile:
            if (i - 2) % 3 == 0:
                v1 = map(int, line.split())
            elif i % 3 == 0:
                v2 = map(int, line.split())
                wFile.write("Case #" + str(i/3) + ': '+ str(CalculateMinProduct(v1, v2)) + '\n')
                        
            i += 1

if __name__ == '__main__':
    if len(sys.argv) > 2:
        MinimumScalarProduct(sys.argv[1], sys.argv[2])
