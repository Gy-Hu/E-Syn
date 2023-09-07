import numpy as np

def normalize_array(arr):
    # 计算数组的平均值和标准差
    mean = np.mean(arr)
    std_dev = np.std(arr)
    
    # 对数组进行正则化
    normalized_arr = (arr - mean) / std_dev
    
    return normalized_arr

arr = np.array([5,727,1678,5,4104,30])
normalized_arr = normalize_array(arr)
print(normalized_arr)