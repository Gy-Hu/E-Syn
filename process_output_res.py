import os
os.system("python run_beta.py>log_rc64b.txt")
os.system("python analyze_data.py>res_data_rc64b.txt")
os.system("python reorder_datasheet.py")
