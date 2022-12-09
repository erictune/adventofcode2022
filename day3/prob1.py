import helpers as h

total = 0

f = open("input.txt", "r")
for line in f.read().split("\n"):
    l = len(line)
    if l == 0: break
    i = int(l / 2)
    assert(l == 2*i)
    x = line[0:i]
    y = line[i:l]
    t = h.item_types(h.shared_chars(x,y))
    print(t)
    assert(len(t) == 1)
    t = t[0]
    p = h.prio(t)
    total += p
print(total)
