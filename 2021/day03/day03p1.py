with open("input.txt", "r") as my_file:
    my_input = [word.strip() for word in my_file]

count_0 = 0
count_1 = 0
gamma_rate = [0,0,0,0,0,0,0,0,0,0,0,0]
epsilon_rate = [0,0,0,0,0,0,0,0,0,0,0,0]

for i in range(12):
    for j in range(len(my_input)):
        tmp = my_input[j]
        if tmp[i] == '0':
            count_0 += 1
        else:
            count_1 += 1
    if count_0 > count_1:
        epsilon_rate[i] = 1
    else:
        gamma_rate[i] = 1
    count_0 = 0
    count_1 = 0

def bin_to_int(bin_array):
    dec = 0
    for i in range(len(bin_array)):
        dec += bin_array[i] * 2**(len(bin_array)-i-1)
    return dec
        

print(gamma_rate)
print(bin_to_int(gamma_rate))
print(epsilon_rate)
print(bin_to_int(epsilon_rate))
print(bin_to_int(gamma_rate) * bin_to_int(epsilon_rate))