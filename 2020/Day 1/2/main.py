""" Advent of code day 2 part 2 """
from itertools import combinations

with open('input.txt') as text_file:
    choices = list(map(int, text_file.readlines()))

    for numbers in combinations(choices, 3):
        if sum(numbers) == 2020:
            first_choice,second_choice, third_choice = numbers
            multiple_numbers = first_choice * second_choice * third_choice
            print(f"{first_choice} and {second_choice} and {third_choice} sum is 2020")
            print(
                f"Multiplying {first_choice} and {second_choice} and {third_choice} is {multiple_numbers}")

    text_file.close()
