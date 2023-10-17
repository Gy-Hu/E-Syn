import networkx as nx
import matplotlib.pyplot as plt
# 读取DOT文件
path  = "out_dot/out_graph_dot4.dot"

graph = nx.Graph(nx.nx_pydot.read_dot(path))

# 打印节点和边的信息
#print("节点：", graph.nodes())
#print("边：", graph.edges())
pos = nx.spring_layout(graph)  # 定义节点的布局
nx.draw(graph, pos, with_labels=True, node_size=500, node_color='lightblue', font_size=10)  # 绘制节点和边

# 显示图形
plt.show()