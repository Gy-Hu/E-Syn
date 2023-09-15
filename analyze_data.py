import re

# 定义表头
headers = ['WireLoad', 'Gates', 'Cap', 'Area', 'Delay']

# 定义空的表格
table = []

# 读取文本文件
i=0
with open('log_rc64b.txt', 'r') as f:
    lines = f.readlines()

    # 遍历每一行
    
    for line in lines:
        # 找到每行含Wireload的开始
        
        if 'WireLoad' in line:
            
            row = []
            i+=1
            if i==1:
                row.append("origin")
            else:
                row.append("op"+str(i-1))

            # 提取Gates和后面的数值
            gates_match = re.search(r'Gates\s*=\s*(\d+)', line)
            if gates_match:
                gates = gates_match.group(1)
                row.append(gates)
            # else:
                # row.append('')

            # 提取Cap和后面的数值
            cap_match = re.search(r'Cap\s*=\s*([\d.]+)\s+ff', line)
            if cap_match:
                cap = cap_match.group(1)
                row.append(cap)
            # else:
                # row.append('')

            # 提取Area和后面的数值
            #area_match = re.search(r'Area\s*=\s*([\d.]+)\s+', line)
            area_match = re.search(r'Area\s*=\s*(\d+)', line)
            if area_match:
                area = area_match.group(1)
                row.append(area)
            # else:
                # row.append('')

            # 提取Delay和后面的数值
            delay_match = re.search(r'Delay\s*=\s*([\d.]+)\s+ps', line)
            if delay_match:
                delay = delay_match.group(1)
                
                row.append(delay)
            # else:
                # row.append('')

            #如果该行缺少元素，可以将缺少的元素补充为默认值
            # while len(row) < len(headers):
                # row.append('')
            
            # 如果行中元素数量等于表头的数量，则将该行数据添加到表格中
            if len(row) == len(headers):
                table.append(row)


# 打印表头
print('{:<10}{:<10}{:<10}{:<10}{:<10}'.format(*headers))

# 打印表格内容
for row in table:
    print('{:<10}{:<10}{:<10}{:<10}{:<10}'.format(*row))