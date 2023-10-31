import csv
import pandas as pd
data = []
with open('res_var1.csv', 'r') as csvfile:
    reader = csv.reader(csvfile)
    next(reader)  
    for row in reader:
        data.append([row[0], float(row[1]), float(row[2]), float(row[3]), float(row[4])])

data_sorted = sorted(data, key=lambda x: x[4])

data_sorted1 = sorted(data, key=lambda x: x[3])

with open('sorted_delay_var1.csv', 'w', newline='') as csvfile:
    writer = csv.writer(csvfile)
    writer.writerow(['WireLoad', 'Gates', 'Cap', 'Area', 'Delay'])  
    writer.writerows(data_sorted)  

with open('sorted_area_var1.csv', 'w', newline='') as csvfile:
    writer = csv.writer(csvfile)
    writer.writerow(['WireLoad', 'Gates', 'Cap', 'Area', 'Delay'])  
    writer.writerows(data_sorted1)  
