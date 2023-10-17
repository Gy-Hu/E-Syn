data = [1.3996, 0.933312, 9, 26, 22] 

min_value = min(data)
max_value = max(data)

normalized_data = [(x - min_value) / (max_value - min_value) for x in data]

print("output", normalized_data)