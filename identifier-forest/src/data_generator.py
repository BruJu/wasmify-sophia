import random


SIZE = 20
Q = 4



def make_quad():
    return ", ".join([str(random.randrange(0, 200)) for _ in range(Q)])



for _ in range(SIZE):
    print("tree.insert([{}]);".format(make_quad()))

