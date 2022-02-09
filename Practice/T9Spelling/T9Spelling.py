import sys

groupMap = {'2':('a', 'b', 'c'), '3':('d', 'e', 'f'), '4':('g', 'h', 'i'), '5':('j', 'k', 'l'),
        '6':('m', 'n', 'o'), '7':('p', 'q', 'r', 's'), '8':('t', 'u', 'v'), '9':('w', 'x', 'y', 'z'), '0':(' '), '$':('$')}

charMap = {'a':'2', 'b':'22', 'c':'222', 'd':'3', 'e':'33', 'f':'333', 'g':'4', 'h':'44', 'i':'444', 'j':'5', 'k':'55', 'l':'555',
           'm':'6', 'n':'66', 'o':'666', 'p':'7', 'q':'77', 'r':'777', 's':'7777', 't':'8', 'u':'88', 'v':'888', 'w':'9', 
           'x':'99', 'y':'999', 'z':'9999', ' ':'0', '$':'$'}
 
def CreateMaped(chars):
    lastChar = '$' 
    lst = []
    for char in chars:
        if char == lastChar:
            lst.append(' ')
        
        if char != '\n':
            if groupMap[charMap[char][0]] == groupMap[charMap[lastChar][0]]:
                lst.append(' ')
            
            lst.append(charMap[char])
            lastChar = char

    return ''.join(lst)

def T9Spelling(inFile, outFile):
    with open(inFile, 'r') as oFile, open(outFile, 'w') as wFile:
        oFile.readline()
        i = 1
        for line in oFile:
            chars = list(line)
            mapedStr = CreateMaped(chars)
            wFile.write('Case #' + str(i) + ': ' + mapedStr + '\n')
            i +=1

if __name__ == '__main__':
    if len(sys.argv) > 2:
        T9Spelling(sys.argv[1], sys.argv[2])
