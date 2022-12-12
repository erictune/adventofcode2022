import helpers as h

containments = 0
overlaps = 0

f = open("input.txt", "r")
for line in f.read().split("\n"):
    l = len(line)
    if l == 0: break
    print(line)
    (r1, r2) = h.line_to_ranges(line)
    if h.fully_contains(r1, r2) or h.fully_contains(r2, r1):
        containments+=1
    if h.overlaps(r1, r2):
        overlaps += 1
print("prob1")
print(containments)
print("prob2")
print(overlaps)