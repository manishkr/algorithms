import sys

def ReverseWords(inFile, outFile):
    with open(inFile, 'r') as oFile, open(outFile, 'w') as wFile:
        oFile.readline()
        i = 1
        for line in oFile:
            words = line.split()
            words.reverse()
            wFile.write("Case #" + str(i) + ": ")
            wFile.write(' '.join(words))
            wFile.write('\n')
            i +=1


if __name__ == '__main__':
    if len(sys.argv) > 2:
        ReverseWords(sys.argv[1], sys.argv[2])
