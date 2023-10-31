import numpy as np
import matplotlib.pyplot as plt
import pandas as pd
import csv
# Read data from CSV file
data = pd.read_csv('sorted_delay.csv')

# Extract Area and Delay columns from the data
area = data['Area']
delay = data['Delay']

# Combine Area and Delay into a numpy array
data = np.column_stack((area, delay))

# Finding Pareto optimal points
pareto_optimal_points = []
for i in range(len(data)):
    is_pareto_optimal = True
    for j in range(len(data)):
        if j != i and (data[j][0] <= data[i][0] and data[j][1] <= data[i][1]):
            is_pareto_optimal = False
            break
    if is_pareto_optimal:
        pareto_optimal_points.append(data[i])

# Separate x and y coordinates
x_pareto = [point[0] for point in pareto_optimal_points]
y_pareto = [point[1] for point in pareto_optimal_points]

# # Calculate R-method scores
# p1 = 1
# p2 = (1 / (1 + 1 / 2))
# pt = p1 + p2
# w1 = p1 / pt
# w2 = p2 / pt

scores = []
for point in pareto_optimal_points:
    score = point[0] * point[1]
    scores.append(score)
   # print("points:", point, "scores:", score)

# Find the best score point
# Sort Pareto optimal points based on scores
sorted_indices = np.argsort(scores)
sorted_points = [pareto_optimal_points[i] for i in sorted_indices]
sorted_scores = [scores[i] for i in sorted_indices]

# Print sorted points and scores
for i in range(len(sorted_points)):
    print("points:", sorted_points[i], "scores:", sorted_scores[i])

# Find the best score point
best_index = np.argmin(sorted_scores)
best_point = sorted_points[best_index]
print("best_point:", best_point)

data1 = []
for i in range(len(sorted_points)):
    point = sorted_points[i]
    delay = point[0]
    area = point[1]
    score = sorted_scores[i]
    data1.append([i, delay, area, score])

# 将数据写入CSV文件
filename = "pareto_optimal_points.csv"
with open(filename, "w", newline="") as csvfile:
    writer = csv.writer(csvfile)
    writer.writerow(["Point", "Delay", "Area", "Score"])  # 写入标题行
    writer.writerows(data1)  # 写入数据行


# Scatter plot
plt.scatter(data[:, 0], data[:, 1], color='red', label='Non-Pareto Points')
plt.scatter(x_pareto, y_pareto, color='blue', label='Pareto Optimal Points')
plt.scatter(best_point[0], best_point[1], color='green', s=100, label='Best Point')

# Add legend and labels
plt.legend()
plt.xlabel('Area')
plt.ylabel('Delay')

# Display the plot
plt.show()