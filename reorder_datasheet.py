with open('res_data_rc64b.txt', 'r') as f:
    lines = f.readlines()

# 将每一行的数据转换为列表
data = []
for line in lines[1:]:
    line = line.strip().split()
    data.append([line[0], float(line[1]), float(line[2]), float(line[3]), float(line[4])])

# 按照delay大小排序
data_sorted = sorted(data, key=lambda x: x[4])

# 按照area大小排序
data_sorted1 = sorted(data, key=lambda x: x[3])


# 将排序后的数据写入新的文本文件
with open('sorted_delay.txt', 'w') as f:
    f.write('{:<10} {:<10} {:<10} {:<10} {:<10}\n'.format('WireLoad', 'Gates', 'Cap', 'Area', 'Delay'))
    for d in data_sorted:
        f.write('{:<10} {:<10} {:<10} {:<10} {:<10}\n'.format(d[0], d[1], d[2], d[3], d[4]))
        
with open('sorted_area.txt', 'w') as f:
    f.write('{:<10} {:<10} {:<10} {:<10} {:<10}\n'.format('WireLoad', 'Gates', 'Cap', 'Area', 'Delay'))
    for d in data_sorted1:
        f.write('{:<10} {:<10} {:<10} {:<10} {:<10}\n'.format(d[0], d[1], d[2], d[3], d[4]))