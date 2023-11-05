import pandas as pd
import numpy as np
from sklearn.linear_model import LinearRegression, Ridge, Lasso, ElasticNet
from sklearn.tree import DecisionTreeRegressor
from sklearn.ensemble import RandomForestRegressor
from sklearn.neighbors import KNeighborsRegressor
from sklearn.model_selection import cross_val_score

data = pd.read_csv('/data/guangyuh/coding_env/E-Brush/xgboost_reg/collect_dataset/fuzz_circuit_analysis_merge_size_51000_23_10_26.csv')
# X = data.drop('delay', axis=1)
# y = data['delay']
# take the first 8 features
X = data.iloc[:, :8].values
# y is area * 0.4 + delay * 0.6
#y = (0.4 * data['area'] + 0.6 * data['delay'])
y = data['delay'].values

def cross_val_evaluate(model, X, y, n_folds=10):
    mae_scores = -cross_val_score(model, X, y, cv=n_folds, scoring='neg_mean_absolute_error')
    rrse_scores = np.sqrt(-cross_val_score(model, X, y, cv=n_folds, scoring='neg_mean_squared_error')) / np.std(y)
    r_scores = cross_val_score(model, X, y, cv=n_folds, scoring='r2')
    return mae_scores.mean(), rrse_scores.mean(), r_scores.mean()

# Linear Regression
lr = LinearRegression()
lr_mae, lr_rrse, lr_r = cross_val_evaluate(lr, X, y)

# Decision Tree
dt = DecisionTreeRegressor()
dt_mae, dt_rrse, dt_r = cross_val_evaluate(dt, X, y)

# Random Forest
rf = RandomForestRegressor()
rf_mae, rf_rrse, rf_r = cross_val_evaluate(rf, X, y)

# Ridge Regression
ridge = Ridge()
ridge_mae, ridge_rrse, ridge_r = cross_val_evaluate(ridge, X, y)

# Lasso Regression
lasso = Lasso()
lasso_mae, lasso_rrse, lasso_r = cross_val_evaluate(lasso, X, y)

# Elastic Net Regression
elastic_net = ElasticNet()
elastic_net_mae, elastic_net_rrse, elastic_net_r = cross_val_evaluate(elastic_net, X, y)

# KNN Regression
knn = KNeighborsRegressor()
knn_mae, knn_rrse, knn_r = cross_val_evaluate(knn, X, y)

print("Linear Regression: MAE={}, RRSE={}, R={}".format(lr_mae, lr_rrse, lr_r))
print("Decision Tree: MAE={}, RRSE={}, R={}".format(dt_mae, dt_rrse, dt_r))
print("Random Forest: MAE={}, RRSE={}, R={}".format(rf_mae, rf_rrse, rf_r))
print("Ridge Regression: MAE={}, RRSE={}, R={}".format(ridge_mae, ridge_rrse, ridge_r))
print("Lasso Regression: MAE={}, RRSE={}, R={}".format(lasso_mae, lasso_rrse, lasso_r))
print("Elastic Net Regression: MAE={}, RRSE={}, R={}".format(elastic_net_mae, elastic_net_rrse, elastic_net_r))
print("KNN Regression: MAE={}, RRSE={}, R={}".format(knn_mae, knn_rrse, knn_r))