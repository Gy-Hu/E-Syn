import csv
import pandas as pd
# 读取CSV文件中的数据
data = []
with open('res_data_rc64b.csv', 'r') as csvfile:
    reader = csv.reader(csvfile)
    next(reader)  # 跳过表头
    for row in reader:
        data.append([row[0], float(row[1]), float(row[2]), float(row[3]), float(row[4])])

# 按照delay大小排序
data_sorted = sorted(data, key=lambda x: x[4])

# 按照area大小排序
data_sorted1 = sorted(data, key=lambda x: x[3])

# 将排序后的数据保存到新的CSV文件
with open('sorted_delay.csv', 'w', newline='') as csvfile:
    writer = csv.writer(csvfile)
    writer.writerow(['WireLoad', 'Gates', 'Cap', 'Area', 'Delay'])  # 写入表头
    writer.writerows(data_sorted)  # 写入排序后的数据

with open('sorted_area.csv', 'w', newline='') as csvfile:
    writer = csv.writer(csvfile)
    writer.writerow(['WireLoad', 'Gates', 'Cap', 'Area', 'Delay'])  # 写入表头
    writer.writerows(data_sorted1)  # 写入排序后的数据

res_data = pd.read_csv('res_data_rc64b.csv')

# 读取 data.csv 文件
data = pd.read_csv('./test_data_beta_runner/data.csv', header=None)
# 读取data.csv文件中的数据
area_delay = res_data[['Area', 'Delay']].iloc[1:].reset_index(drop=True)
print(area_delay)
# 将 area 和 delay 列添加到 data.csv 的第九列和第十列
data.insert(8, 'area', area_delay['Area'])
data.insert(9, 'delay', area_delay['Delay'])

# 保存合并后的结果为新的文件 new_data.csv
data.to_csv('new_data.csv', index=False, header=False)
new_data = pd.read_csv('new_data.csv')
new_data.to_csv('total.csv', mode='a', index=False, header=False)