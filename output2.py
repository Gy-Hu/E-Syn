import argparse
import os
import time

parser = argparse.ArgumentParser(description="Output 2 Script")
parser.add_argument("delay", type=int, help="目标延迟值（以ps为单位）")
parser.add_argument("directory1", type=str, help="目录1")
args = parser.parse_args()

start_time = time.time()

os.system(f"python test_variation.py {args.delay} > log_var.txt")
os.system("python analyze_data1.py")
os.system("python reorder_datasheet1.py")

os.system(f"python test_variation1.py {args.delay} > log_var1.txt")
os.system("python analyze_data2.py")
os.system("python reorder_datasheet2.py")

output_directory = f"./analyze/var/{args.directory1}/" 
if not os.path.exists(output_directory):
    os.makedirs(output_directory)

command1 = f"cp ./sorted_delay_var.csv {output_directory}/sorted_delay_var.csv"
command2 = f"cp ./sorted_delay_var1.csv {output_directory}/sorted_delay_var1.csv"
command3 = f"cp ./sorted_area_var.csv {output_directory}/sorted_area_var.csv"
command4 = f"cp ./sorted_area_var1.csv {output_directory}/sorted_area_var1.csv"
command5 = f"cp ./log_var.txt {output_directory}/log_var.txt"
command6 = f"cp ./log_var1.txt {output_directory}/log_var1.txt"
os.system(command1)
os.system(command2)
os.system(command3)
os.system(command4)
os.system(command5)
os.system(command6)