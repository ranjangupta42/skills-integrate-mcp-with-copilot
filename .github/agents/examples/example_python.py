"""
Example Python code for testing the Code Review Agent

This file contains some intentional issues that the agent should detect:
- Missing error handling
- Security vulnerability
- Performance issues
- PEP 8 violations
"""

import sqlite3

def get_user_data(user_id):
    # Potential SQL injection vulnerability
    conn = sqlite3.connect('users.db')
    cursor = conn.cursor()
    query = f"SELECT * FROM users WHERE id = {user_id}"  # UNSAFE!
    cursor.execute(query)
    result = cursor.fetchone()
    # Missing resource cleanup (connection not closed)
    return result

def process_numbers(numbers):
    # Inefficient - should use list comprehension
    result = []
    for num in numbers:
        result.append(num * 2)
    return result

def divide(a, b):
    # Missing error handling for division by zero
    return a / b

class UserManager:
    def __init__(self):
        self.users={}  # PEP 8: missing spaces around assignment operator
    
    def AddUser(self,name,email):  # PEP 8: method should be snake_case, missing spaces after commas
        self.users[email]=name  # PEP 8: missing spaces around assignment operator
        return True
