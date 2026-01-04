def convert_token(token):
    # return an int if it's a number, otherwise return string
    if token.isdigit():
        return int(token)
    else:
        return token


def parse(code):
    stack = []
    current_token = ""

    for char in code:
        if char == '(':
            stack.append([])
        elif char == ' ':
            if current_token:
                stack[-1].append(convert_token(current_token))
                current_token = ""
        elif char not in '() ':
            current_token += char
        elif char == ')':
            if current_token:
                stack[-1].append(convert_token(current_token))
                current_token = ""
            completed = stack.pop()
            if stack:
                stack[-1].append(completed)
            else:
                return completed
    return stack


result = parse("(first (list 1 (+ 2 3) 9))")
print(result)
