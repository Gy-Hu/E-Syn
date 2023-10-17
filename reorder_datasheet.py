import csv
import pandas as pd
data = []
with open('res_data_rc64b.csv', 'r') as csvfile:
    reader = csv.reader(csvfile)
    next(reader)  
    for row in reader:
        data.append([row[0], float(row[1]), float(row[2]), float(row[3]), float(row[4])])

data_sorted = sorted(data, key=lambda x: x[4])

data_sorted1 = sorted(data, key=lambda x: x[3])

with open('sorted_delay.csv', 'w', newline='') as csvfile:
    writer = csv.writer(csvfile)
    writer.writerow(['WireLoad', 'Gates', 'Cap', 'Area', 'Delay'])  
    writer.writerows(data_sorted)  

with open('sorted_area.csv', 'w', newline='') as csvfile:
    writer = csv.writer(csvfile)
    writer.writerow(['WireLoad', 'Gates', 'Cap', 'Area', 'Delay'])  
    writer.writerows(data_sorted1)  

res_data = pd.read_csv('res_data_rc64b.csv')


data = pd.read_csv('./test_data_beta_runner/data.csv', header=None)

area_delay = res_data[['Area', 'Delay']].iloc[1:].reset_index(drop=True)
print(area_delay)

data.insert(8, 'area', area_delay['Area'])
data.insert(9, 'delay', area_delay['Delay'])

data.to_csv('new_data.csv', index=False)  # 省略header=False参数
new_data = pd.read_csv('new_data.csv')
new_data.to_csv('total.csv', mode='a', index=False, header=False)