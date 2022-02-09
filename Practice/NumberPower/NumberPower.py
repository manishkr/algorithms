import sys
import math

def MatrixMult(a, b):
    c = [[0,0], [0,0]]
    for i in range(2):
        for j in range(2):
            for k in range(2):
                c[i][k] = (c[i][k] + a[i][j]*b[j][k]) % 1000

    return c

def exponent(a, num):
    val = a
    if num != 1:
        if num % 2 == 0:
            tmp = exponent(a, num/2)
            val = MatrixMult(tmp, tmp)
        else:
            val = MatrixMult(a, exponent(a, num-1))

    return val

def FindSuffix(num):
    a = [[3,5],[1,3]]
    an = exponent(a, num)
    val = (2 * an[0][0] + 999) % 1000
    strVal = str(val)
    if len(strVal) < 3:
        strVal = ''.join(['0']*(3-len(strVal))) + strVal

    return strVal

def NumberPower(inFile, outFile):
    with open(inFile, 'r') as oFile, open(outFile, 'w') as wFile:
        oFile.readline()
        i = 1
        for line in oFile:
            val = int(line)
            last3 = FindSuffix(val)
            wFile.write('Case #' + str(i) + ': ' + last3 + '\n')
            i += 1

if __name__ == '__main__':
    if len(sys.argv) > 2:
        NumberPower(sys.argv[1], sys.argv[2])
