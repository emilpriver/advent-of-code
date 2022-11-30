""" Advent of code day 2 part 2 """

with open("input.txt") as text_file:
  choices = list(text_file.readlines())
  valid_passwords = []

  for item in choices:
    items = item.split(" ")
    first_value, last_value = items[0].split("-")
    char = items[1].strip(":")
    string = items[2].strip("\n")
    first_value = int(first_value) - 1
    last_value = int(last_value) - 1
    
    if string[first_value] == char and string[last_value] == char:
      continue

    if string[first_value] == char or string[last_value] == char:
      valid_passwords.append(string)

  print(len(valid_passwords))

text_file.close()