with open("input-1.txt", 'r') as my_file:
    file_array = [int(word.strip()) for word in my_file]
my_file.close()

old = file_array[0] + file_array[1] + file_array[2]
new = 0
increased = 0

for i in range(len(file_array)-2):
    new = file_array[i] + file_array[i+1] + file_array[i+2]
    if new > old:
        increased += 1
    old = new

print(increased)
