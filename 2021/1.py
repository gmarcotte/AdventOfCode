#!/usr/local/bin/python3

import getopt
import sys

def computeWindowIncreases(file, size):
    vals = []
    ups = 0
    for line in file:
        val = int(line.strip())
        if len(vals) < size:
            vals.append(val)
            continue

        last = vals.pop(0)
        if val > last:
            ups += 1
        vals.append(val)
    return ups

def printHelp():
    print('1.py [-f <file = sys.stdin>] [-w <window_size = 1>] [-h]')


if __name__ == '__main__':
    # Defaults
    file = sys.stdin
    size = 1

    # Read command-line options
    try:
        opts, args = getopt.getopt(sys.argv[1:], 'f:w:h')
    except getopt.GetoptError:
        printHelp()
        sys.exit(2)

    for opt, arg in opts:
        if opt == '-h':
            printHelp()
            sys.exit(2)
        elif opt == '-f':
            file = open(arg)
        elif opt == '-w':
            size = int(arg)

    print(computeWindowIncreases(file, int(size)))
