dict = {
    "3"     : "Fizz",
    "5"     : "Buzz",
    "7"     : "Bizz",
    "11"    : "Fuzz"
}

for i in range(255):
    output = ""
    
    for key,value in dict.items():
        if i % int(key) == 0:
            output += value

    print(output if output != "" else i)
