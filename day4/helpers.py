def line_to_ranges(line):

    (r1,r2) = line.split(",")

    a, b = r1.split("-")
    c, d = r2.split("-")
    a = int(a)
    b = int(b)
    c = int(c)
    d = int(d)
    assert(a<=b)
    assert(c<=d)
    return ((int(a), int(b)), (int(c), int(d)))

def test_line_to_ranges():
    assert(line_to_ranges("1-2,3-4") == ((1,2),(3,4)))
    assert(line_to_ranges("99-101,100-567") == ((99,101), (100, 567)))

test_line_to_ranges()

# True iff r1 fully contains r2.
def fully_contains(r1, r2):
    (a, b) = r1
    (c, d) = r2
    assert(a<=b);
    assert(c<=d);
    return a<=c and d<=b

def test_fully_contains():
    assert(fully_contains((1,4), (2,3)))
    assert(not fully_contains((2,3), (1,4)))
    assert(not fully_contains((1,2), (3,4)))
    assert(not fully_contains((1,2), (3,4)))
    assert(not fully_contains((1,3), (2,4)))

test_fully_contains()

# True iff r1 and r2 overlap, including being equal (these are closed ranges).
def overlaps(r1, r2):
    return not disjoint(r1,r2)

def disjoint(r1, r2):
    (a, b) = r1
    (c, d) = r2
    assert(a<=b);
    assert(c<=d);
    # Given: r1 from a to b, c to d can be the following ways and _not_ overlap (be disjoint)
    #         a___b        
    #   c__d
    #                c__d
    return d<a or c>b 

def test_overlaps():
    assert(overlaps((1,4), (2,3)))
    assert(overlaps((1,3), (2,3)))
    assert(overlaps((1,2), (2,3)))
    assert(overlaps((2,2), (2,3)))
    assert(overlaps((3,3), (2,3)))
    assert(overlaps((2,3), (1,4)))
    assert(overlaps((2,3), (1,3)))
    assert(overlaps((2,3), (1,2)))
    assert(overlaps((2,3), (2,2)))
    assert(overlaps((2,3), (3,3)))

    assert(not overlaps((1,2), (3,4)))
    assert(not overlaps((1,2), (3,4)))
    assert(overlaps((1,3), (2,4)))


test_overlaps()