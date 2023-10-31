import os
import time
start_time = time.time()
os.system("python run_beta.py>log_rc64b.txt")
os.system("python analyze_data.py")
os.system("python reorder_datasheet.py")
#os.system("rm ./test_data_beta_runner/data.csv")
#folder_path = "/data/cchen/E-Brush/out_dot"
#command = f'rm -rf "{folder_path}"'
#os.system(command)
os.system("python pareto_analyze.py")
end_time = time.time()  # 记录结束时间

execution_time = end_time - start_time  # 计算运行时间

print("run time :", execution_time, "s")