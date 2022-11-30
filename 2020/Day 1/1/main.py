""" Advent of code day 2 part 1 """
from itertools import combinations

with open('input.txt') as text_file:
    choices = list(map(int, text_file.readlines()))

    for numbers in combinations(choices, 2):
            if sum(numbers) == 2020:
                first_choice, second_choice = numbers
                print(f"{first_choice} and {second_choice} sum is 2020")
                print(
                    f"Multiplying {first_choice} and {second_choice} is {first_choice * second_choice}")

    text_file.close()