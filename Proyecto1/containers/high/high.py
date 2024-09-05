import random

large_list = [random.randint(0, 1000000) for _ in range(100000000)]
large_list.sort()
