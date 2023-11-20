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
y = data['area'].values

def cross_val_evaluate(model, X, y, n_folds=10):
    mae_scores = -cross_val_score(model, X, y, cv=n_folds, scoring='neg_mean_absolute_error')
    mape = cross_val_score(model, X, y, scoring='neg_mean_absolute_percentage_error', cv=n_folds)
    rrse_scores = np.sqrt(-cross_val_score(model, X, y, cv=n_folds, scoring='neg_mean_squared_error')) / np.std(y)
    r_scores = cross_val_score(model, X, y, cv=n_folds, scoring='r2')
    rmse_scores = np.sqrt(-cross_val_score(model, X, y, cv=n_folds, scoring='neg_mean_squared_error'))
    return mae_scores.mean(), rrse_scores.mean(), r_scores.mean(), mape.mean(), rmse_scores.mean()

# Linear Regression
lr = LinearRegression()
lr_mae, lr_rrse, lr_r , lr_mape, lr_rmse = cross_val_evaluate(lr, X, y)

# Decision Tree
dt = DecisionTreeRegressor()
dt_mae, dt_rrse, dt_r , dt_mape, dt_rmse = cross_val_evaluate(dt, X, y)

# Random Forest
rf = RandomForestRegressor()
rf_mae, rf_rrse, rf_r, rf_mape, rf_rmse = cross_val_evaluate(rf, X, y)

# Ridge Regression
ridge = Ridge()
ridge_mae, ridge_rrse, ridge_r , ridge_mape, ridge_rmse = cross_val_evaluate(ridge, X, y)

# Lasso Regression
lasso = Lasso()
lasso_mae, lasso_rrse, lasso_r , lasso_mape, lasso_rmse = cross_val_evaluate(lasso, X, y)

# Elastic Net Regression
elastic_net = ElasticNet()
elastic_net_mae, elastic_net_rrse, elastic_net_r , elastic_net_mape, elastic_net_rmse = cross_val_evaluate(elastic_net, X, y)

# KNN Regression
knn = KNeighborsRegressor()
knn_mae, knn_rrse, knn_r, knn_mape, knn_rmse = cross_val_evaluate(knn, X, y)

print("Linear Regression: MAE={}, RRSE={}, R_square={}, MAPE={}, RMSE={}".format(lr_mae, lr_rrse, lr_r,lr_mape, lr_rmse))
print("Decision Tree: MAE={}, RRSE={}, R_square={}, MAPE={}, RMSE = {}".format(dt_mae, dt_rrse, dt_r,dt_mape, dt_rmse))
print("Random Forest: MAE={}, RRSE={}, R_square={}, MAPE={}, RMSE = {}".format(rf_mae, rf_rrse, rf_r,rf_mape, rf_rmse))
print("Ridge Regression: MAE={}, RRSE={}, R_square={}, MAPE={}, RMSE = {}".format(ridge_mae, ridge_rrse, ridge_r, ridge_mape, ridge_rmse))
print("Lasso Regression: MAE={}, RRSE={}, R_square={}, MAPE={}, RMSE = {}".format(lasso_mae, lasso_rrse, lasso_r,lasso_mape, lasso_rmse))
print("Elastic Net Regression: MAE={}, RRSE={}, R_square={}, MAPE={}, RMSE = {}".format(elastic_net_mae, elastic_net_rrse, elastic_net_r,elastic_net_mape, elastic_net_rmse))
print("KNN Regression: MAE={}, RRSE={}, R_square={}, MAPE={}, RMSE = {}".format(knn_mae, knn_rrse, knn_r, knn_mape, knn_rmse))