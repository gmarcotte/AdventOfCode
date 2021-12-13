#!/usr/local/bin/python3

import getopt
import sys

# Board size NxN
N = 5

def printHelp():
    print('1.py [-f <file = sys.stdin>] [-w <window_size = 1>] [-h]')


def readBoard(file):
    board = []
    for i in range(N):
        line = file.readline()
        board.append([int(x) for x in line.strip().split()])
    return board

def readBoards(file):
    boards = []
    while file.readline():
        boards.append(readBoard(file))
    return boards

def testBoard(board, pick_lookup):
    row_marks = []
    col_marks = [[] for i in range(N)]
    for row in board:
        row_marks_row = []
        for i in range(len(row)):
            row_marks_row.append(pick_lookup[row[i]])
            col_marks[i].append(pick_lookup[row[i]])
        row_marks.append(row_marks_row)

    row_moves = min([max(row) for row in row_marks])
    col_moves = min([max(col) for col in col_marks])

    moves = min([row_moves, col_moves])

    score = 0
    for i in range(len(row_marks)):
        for j in range(len(row_marks[i])):
            if row_marks[i][j] > moves:
                score += board[i][j]

    return moves, score

def runGame(file):
    picks = [int(x) for x in file.readline().strip().split(',')]
    pick_lookup = {picks[i]: i for i in range(len(picks))}

    boards = readBoards(file)

    winner_moves = 0
    winner_score = 0

    last_moves = 0
    last_score = 0

    for board in boards:
        moves, score = testBoard(board, pick_lookup)
        if winner_moves == 0 or moves < winner_moves:
            winner_moves = moves
            winner_score = score * picks[moves]

        if moves > last_moves:
            last_moves = moves
            last_score = score * picks[moves]

    return winner_score, last_score
    

if __name__ == '__main__':
    # Defaults
    file = sys.stdin
    size = 1

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

    print(runGame(file))
