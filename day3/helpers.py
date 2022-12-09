_prio = {}
for c in range(ord('a'), ord('z')+1):
    _prio[chr(c)] = c - ord('a') + 1
for c in range(ord('A'), ord('Z')+1):
    _prio[chr(c)] = c - ord('A') + 27

def prio(c):
    return _prio[c]

def test_prio():
    assert(prio('a') == 1)
    assert(prio('Z') == 52)

def shared_chars(x, y):
    return [c for c in x if c in y]  # quadratic but fine for this input size

def test_shared_chars():
    assert(shared_chars("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn") == ['v', 'v'])

def item_types(l):
    return [x for x in set(l)]

def test_item_types():
    assert(sorted(item_types(["v","v","x"])) == ["v","x"])

# TODO: install pytest
test_prio()
test_shared_chars()
test_item_types()

