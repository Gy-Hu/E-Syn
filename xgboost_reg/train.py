import xgboost as xgb
from xgboost import plot_importance
from matplotlib import pyplot as plt
from sklearn.model_selection import KFold, GridSearchCV, train_test_split
from sklearn.preprocessing import StandardScaler
import pandas as pd
import numpy as np
from sklearn import metrics
import m2cgen as m2c

def mape(y_true, y_pred):
    return np.mean(np.abs((y_true - y_pred) / y_true)) * 100

def rrse(y_true, y_pred):
    return np.sqrt(np.sum((y_true - y_pred)**2) / np.sum((y_true - np.mean(y_true))**2))

def r(y_true, y_pred):
    return np.corrcoef(y_true, y_pred)[0, 1]

df = pd.read_csv('collect_dataset/fuzz_circuit_analysis_merge_size_51000_23_10_26.csv')
X = df.iloc[:, :8].values
#y = ( df['area'] * df['delay']).values
y = df['delay'].values

# Scale the features
# scaler = StandardScaler()
# X = scaler.fit_transform(X)

# Split the dataset into train and test sets
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)

params = {
    'max_depth': [3, 5, 10],
    'learning_rate': [0.01, 0.1, 0.2],
    'n_estimators': [100, 160, 200],
    'objective': ['reg:gamma'],
    'booster': ['gbtree'],
    'tree_method': ['gpu_hist'],
    'predictor': ['gpu_predictor'],
    'n_jobs': [-1],
    'seed': [123]
}

model = xgb.XGBRegressor()
kf = KFold(n_splits=10, shuffle=True, random_state=0)
# print data size
print("X_train size:", X_train.shape)
print("X_test size:", X_test.shape)
grid_search = GridSearchCV(estimator=model, param_grid=params, scoring='neg_mean_absolute_error', cv=kf, verbose=1)
grid_search.fit(X,y)

best_params = grid_search.best_params_
print("Best parameters found:", best_params)

# Train the model with the best parameters on the entire dataset for feature importance
model_full = xgb.XGBRegressor(**best_params).fit(X_train, y_train)
plot_importance(model_full)
plt.savefig('feature_importance.png')

# Print the best score (mean absolute error)
best_score = -grid_search.best_score_
print("Best Mean Absolute Error:", best_score)

y_pred = model_full.predict(X_test)
print("Mean Absolute Error Percentage (MAPE):", mape(y_test, y_pred))
print("Root Relative Square Error (RRSE):", rrse(y_test, y_pred))
print("Correlation Coefficient (R):", r(y_test, y_pred))
print("Mean Absolute Error (MAE):", metrics.mean_absolute_error(y_test, y_pred))

code = m2c.export_to_rust(model_full)

# write code in model.rs
with open('model.rs', 'w') as f:
    f.write(code)