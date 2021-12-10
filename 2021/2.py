#!/usr/local/bin/python3

import getopt
import sys

def computePosition(file):
    h_pos = 0
    d_pos = 0
    for line in file:
        dir, val = line.strip().split(' ')
        num = int(val)
        if dir == 'forward':
            h_pos += num
        elif dir == 'up':
            d_pos -= num
        elif dir == 'down':
            d_pos += num

    return h_pos, d_pos

def computePositionWithAim(file):
    h_pos = 0
    d_pos = 0
    aim = 0
    for line in file:
        dir, val = line.strip().split(' ')
        num = int(val)
        if dir == 'forward':
            h_pos += num
            d_pos += aim * num
        elif dir == 'up':
            aim -= num
        elif dir == 'down':
            aim += num

    return h_pos, d_pos

def printHelp():
    print('2.py [-f <file = sys.stdin>] [-h]')


if __name__ == '__main__':
    # Defaults
    file = sys.stdin
    useAim = False

    # Read command-line options
    try:
        opts, args = getopt.getopt(sys.argv[1:], 'f:ha')
    except getopt.GetoptError:
        printHelp()
        sys.exit(2)

    for opt, arg in opts:
        if opt == '-h':
            printHelp()
            sys.exit(2)
        elif opt == '-f':
            file = open(arg)
        elif opt == '-a':
            useAim = True

    h_pos, d_pos = computePositionWithAim(file) if useAim else computePosition(file)
    print(f'Horizontal: {h_pos}, Depth: {d_pos}')
    print(f'Product: {h_pos * d_pos}')
