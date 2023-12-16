with open("input.txt", "r") as my_file:
    my_input = [word.strip() for word in my_file]

oxygen = my_input[:]
co2_scrubber = my_input[:]

count_0 = 0
count_1 = 0

for i in range(len(my_input[0])):
    for item in oxygen:
        if item[i] == '0':
            count_0 += 1
        else:
            count_1 += 1

    if count_0 > count_1:
        most = 0
    else:
        most = 1
    
    oxygen = [x for x in oxygen if int(x[i]) == most]
    count_0 = 0
    count_1 = 0
    if len(oxygen) == 1:
        break

for i in range(len(my_input[0])):
    for item in co2_scrubber:
        if item[i] == '0':
            count_0 += 1
        else:
            count_1 += 1

    if count_0 > count_1:
        least = 1
    else:
        least = 0

    co2_scrubber = [x for x in co2_scrubber if int(x[i]) == least]
    count_0 = 0
    count_1 = 0
    if len(co2_scrubber) == 1:
        break


def bin_to_int(bin_array):
    dec = 0
    for i in range(len(bin_array)):
        dec += int(bin_array[i]) * 2**(len(bin_array)-i-1)
    return dec

print(bin_to_int(list(oxygen[0])))
print(bin_to_int(list(co2_scrubber[0])))
print(bin_to_int(list(oxygen[0])) * bin_to_int(list(co2_scrubber[0])))
