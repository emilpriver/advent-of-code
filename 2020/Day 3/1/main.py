""" Advent of code day 3 part 1 """

with open("input.txt") as text_file:
  lines = list(text_file.readlines())
  total_tress = 0
  x = 3

  for (index, line) in enumerate(lines):
    line = line.strip("\n")
    if index == 0:
      continue

    if line[x] == "#":
      total_tress += 1

    x += 3

    if len(line) <= x:
      x = x - len(line)

  print(total_tress)
