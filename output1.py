import os
import csv
import re
directory1 = input("请输入保存的路径：")
directory = input("请输入保存的文件名：")
output_directory = f"./analyze/{directory1}/" + directory
if not os.path.exists(output_directory):
    os.makedirs(output_directory)


command1 = f"cp ./sorted_delay.csv ./analyze/{directory1}/{directory}/sorted_delay.csv"
command2 = f"cp ./sorted_area.csv ./analyze/{directory1}/{directory}/sorted_area.csv"
command3 = f"cp ./new_data.csv ./analyze/{directory1}/{directory}/new_data.csv"
command4 = f"cp ./res_data_rc64b.csv ./analyze/{directory1}/{directory}/res_data_rc64b.csv"
command5 = f"cp ./pareto_optimal_points.csv ./analyze/{directory1}/{directory}/pareto_optimal_points.csv"
os.system(command1)
os.system(command2)
os.system(command3)
os.system(command4)
os.system(command5)


# with open('./sorted_delay.csv', 'r') as file:
#     reader = csv.reader(file)
#     next(reader)
    
#     data = []
#     for i, row in enumerate(reader):
#         if i >= 0 and i <= 2:
#             value = row[0]-1
#             match = re.search(r'\d+', value)
#             if match:
#                 num = float(match.group())
#                 data.append(int(num))
    
#     for value in data:
#         command5=f"cp ./test_data_beta_runner/optimized_circuit{value}.eqn ./analyze/{directory1}/{directory}/optimized_circuit{value}.eqn"
#         os.system(command5)

# with open('./sorted_area.csv', 'r') as file:
#     reader = csv.reader(file)
#     next(reader)
    
#     data = []
#     for i, row in enumerate(reader):
#         if i >= 0 and i <= 2:
#             value = row[0]
#             match = re.search(r'\d+', value)
#             if match:
#                 num = float(match.group())
#                 data.append(int(num))
    
#     for value in data:
#         command5=f"cp ./test_data_beta_runner/optimized_circuit{value}.eqn ./analyze/{directory1}/{directory}/optimized_circuit{value}.eqn"
#         os.system(command5)