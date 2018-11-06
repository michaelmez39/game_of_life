iterations = 0
while iterations < 1000:
    board = [[False*100]*100]
    board2 = board
    for x, row in enumerate(board):
        for y, item in enumerate(board):
            if x == 99 or y == 99: 
                continue

            neighbors = 0

            for a in range(-1, 1):
                for b in range(-1, 1):
                    if board2[x + a+1][y + b+ 1]:
                        neighbors += 1
            
            if neighbors < 2:
                board[x+1][y+1] = False
            
            if neighbors == 3:
                board[x+1][y+1] = True

            if neighbors > 3:
                board[x+1][y+1] = False
            
            if board[x+1][y+1]:
                print("*", end="")
            else:
                print("o", end="")
        print()
    iterations += 1