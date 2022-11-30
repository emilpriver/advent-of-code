""" Advent of code day 3 part 2 """

with open("input.txt") as text_file:
  lines = list(text_file.readlines())
  total_tress = 1
  slopes = [[1,1], [3, 1],[5, 1],[7, 1],[1, 2]]

  for slope in slopes:
    slope_trees = 0
    x = slope[0]
    y = slope[1]

    for (index, line) in enumerate(lines[::y]):
      line = line.strip("\n")
      if index == 0:
        continue

      if line[x] == "#":
        slope_trees += 1

      x += slope[0]

      if len(line) <= x:
        x = x - len(line)

    total_tress *= slope_trees

  print(total_tress)
