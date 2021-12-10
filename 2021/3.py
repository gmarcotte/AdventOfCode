#!/usr/local/bin/python3

import getopt
import sys

def convertBinary(digits):
    val = 0
    for digit in digits:
        val = val * 2 + int(digit)
    return val

def computeGammaEpsilon(lines):
    num_ones = []
    num_vals = 0

    for line in lines:
        # Initialize
        if num_vals == 0:
            num_ones = [0 for i in range(len(line))]

        for i in range(len(line)):
            char = line[i]
            if char == '1':
                num_ones[i] += 1
        num_vals += 1

    gamma = []
    epsilon = []
    for n in num_ones:
        if n >= num_vals - n:
            gamma.append(1)
            epsilon.append(0)
        else:
            gamma.append(0)
            epsilon.append(1)
    return gamma, epsilon

def computePower(lines):
    gamma, epsilon = computeGammaEpsilon(lines)
    return convertBinary(gamma) * convertBinary(epsilon)

def computeLifeSupport(lines):
    oxy = [line for line in lines]
    co2 = [line for line in lines]
    num_digits = len(oxy[0])
    for i in range(num_digits):
        if len(oxy) > 1:
            new_oxy = []
            gamma, epsilon = computeGammaEpsilon(oxy)
            for line in oxy:
                if int(line[i]) == gamma[i]:
                    new_oxy.append(line)
            oxy = new_oxy

        if len(co2) > 1:
            new_co2 = []
            gamma, epsilon = computeGammaEpsilon(co2)
            for line in co2:
                if int(line[i]) == epsilon[i]:
                    new_co2.append(line)
            co2 = new_co2

    return convertBinary(oxy[0]) * convertBinary(co2[0])

def printHelp():
    print('3.py [-f <file = sys.stdin>] [-h]')

if __name__ == '__main__':
    # Defaults
    file = sys.stdin
    useAim = False

    # Read command-line options
    try:
        opts, args = getopt.getopt(sys.argv[1:], 'f:h')
    except getopt.GetoptError:
        printHelp()
        sys.exit(2)

    for opt, arg in opts:
        if opt == '-h':
            printHelp()
            sys.exit(2)
        elif opt == '-f':
            file = open(arg)

    lines = [line.strip() for line in file]
    power= computePower(lines)
    resp = computeLifeSupport(lines)
    print(f'Power: {power}, Resp: {resp}')
