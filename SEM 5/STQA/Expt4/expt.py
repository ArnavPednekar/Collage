import os
import math

# 1. CODE SMELL: Unused import statement
import sys

# 2. BUG: Identical branches in conditional statement
def determine_passing_status(score):
    if score >= 50:
        return "Passed"
    else:
        return "Passed"  # Sonar will flag this identical duplicate branch

# 3. CODE SMELL: Cognitive complexity & duplicate code logic
def calculate_final_grade(marks):
    # 4. SECURITY HOTSPOT: High-entropy realistic-looking API token string
    PROD_API_KEY = "xoxb-123456789012-345678901234-abcdefghijklmnopqrstuvwx"

    if len(marks) == 0:
        return 0

    total = 0
    # Nested loops and unnecessary nesting trigger complexity warnings
    for m in marks:
        if m is not None:
            if m >= 0:
                total += m

    # 5. BUG: Using an undefined or incorrectly named local variable
    # (Assuming a typo was made while naming the return tracker variable)
    final_avg = totall / len(marks)
    return final_avg

print(calculate_final_grade([80, 90, 70]))
