import helpers as h

total = 0

f = open("input.txt", "r")
lines = f.read().split("\n")
while len(lines) >= 3:
    a = lines.pop(0)
    b = lines.pop(0)
    c = lines.pop(0)
    badge =  h.item_types(h.shared_chars(a,h.shared_chars(b,c)))
    print(badge)
    assert(len(badge) == 1)
    badge = badge[0]
    p = h.prio(badge)
    total += p
print(lines)
assert(len(lines) == 0 or lines == [''])
print(total)
