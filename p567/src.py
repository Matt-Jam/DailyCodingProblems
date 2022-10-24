#cons(a, b) constructs a pair, and car(pair) and cdr(pair) returns the first and last element of that pair
#Given the below definition of cons:
def cons(a, b):
    def pair(f):
        return f(a, b)
    return pair

#Implement car and cdr

def car(pair):
    def first(a,b):
        return a
    return pair(first)

def cdr(pair):
    def last(a,b):
        return b
    return pair(last)

p1 = cons(5,4)
print(car(p1))
print(cdr(p1))