with open("input-1.txt", 'r') as my_file:
    file_array = [int(word.strip()) for word in my_file]
my_file.close()

old = file_array[0]
new = 0
increased = 0

for i in range(len(file_array)):
    new = file_array[i]
    if old < new:
        increased += 1
    old = new
    
print(increased)