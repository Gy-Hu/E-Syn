import numpy as np
import matplotlib.pyplot as plt

# 定义数据点
data = np.array([
    [2463, 1705.1],
    [2442, 1716.55],
    [2469, 1745.9],
    [2496, 1755.58],
    [2427, 1759.88],
    [2507, 1759.91],
    [2456, 1769.39],
    [2550, 1771.29],
    [2565, 1771.87],
    [2296, 1802.26],
    [2569, 1802.63],
    [2442, 1814.15],
    [2446, 1822.26],
    [2418, 1822.88],
    [2367, 1829.14],
    [2495, 1836.44],
    [2523, 1839.79],
    [2412, 1858.9],
    [2462, 1861.56],
    [2392, 1862.72],
    [2534, 1871.39],
    [2186, 1884.19],
    [2370, 1891.27],
    [2356, 1913.69],
    [2305, 1921.47],
    [2372, 1927.52],
    [2565, 2019.06],
    [2207, 2027.61],
    [2416, 2078.63],
    [2221, 2098.83],
    [2400, 2141.49]
])

# 找到 Pareto 最优解
pareto_optimal_points = []
for i in range(len(data)):
    is_pareto_optimal = True
    for j in range(len(data)):
        if j != i and (data[j][0] <= data[i][0] and data[j][1] <= data[i][1]):
            is_pareto_optimal = False
            break
    if is_pareto_optimal:
        pareto_optimal_points.append(data[i])

# 将数据点分为 x 和 y 坐标
x_pareto = [point[0] for point in pareto_optimal_points]
y_pareto = [point[1] for point in pareto_optimal_points]

# 计算 R-method 得分
p1=1
p2=(1/(1+1/2))
pt = p1 +p2
w1 =p1/pt
w2 =p2/pt

scores = []
for point in pareto_optimal_points:
    score = w2*point[0] + w1*point[1]
    scores.append(score)
    print("points: ",point , "scores: ",score,"a*d :" ,point[0]*point[1])
# 找到最优得分的点
best_index = np.argmin(scores)
best_point = pareto_optimal_points[best_index]



print("best_point",best_point)
# 绘制散点图
plt.scatter(data[:, 0], data[:, 1], color='red', label='Non-Pareto Points')
plt.scatter(x_pareto, y_pareto, color='blue', label='Pareto Optimal Points')
plt.scatter(best_point[0], best_point[1], color='green', s=100, label='Best Point')

# 添加图例和标签
plt.legend()
plt.xlabel('Area')
plt.ylabel('Delay')

# 显示图形
plt.show()