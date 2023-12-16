with open("prova.txt", 'r') as my_file:
    my_input = [word.strip() for word in my_file]
    drawn = map(int, my_file.readline().split(','))
my_file.close()


def make_board(i, in_board):
    board = []
    for _ in range(6):
        if my_input[i] == '':
            in_board = ~in_board
        elif in_board:
            board.append(my_input[i].split(' '))
        i += 1
    return board

all_boards = []
i = 0
for _ in range(len(my_input)):
    while (my_input[i] != ''): 
        i += 1
    all_boards.append(make_board(i, False))



print(drawn)
