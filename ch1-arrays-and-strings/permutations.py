# O(nlogn)
def permuted(v,w):
  if len(v) != len(w):
    return False

  v = sorted(v)
  w = sorted(w)
  
  for i in range(len(v)):
    if v[i] != w[i]:
      return False

  return True

# O(nlogn)
def permuted_pythonic(v,w):
  return sorted(v) == sorted(w)

# O(n)
def permuted_count(v,w):
  if len(v) != len(w):
    return False

  no_letters = 26
  count_v = [0] * no_letters
  count_w = [0] * no_letters

  for c in v:
    count_v[ord(c) - 97] += 1

  for c in w:
    count_w[ord(c) - 97] += 1

  return count_v == count_w
