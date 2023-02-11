inp = [[1],[1,2],[10,5,3]]

def soln(input):
    if len(input) == 1:
        return input[0][0]
    
    for i in range(len(input[-2])):
        input[-2][i] += max(input[-1][i],input[-1][i+1])
    
    return soln(input[:-1])
