#from xgboost import XGBClassifier
import xgboost as xgb
import pandas as pd
# # read data
# from sklearn.datasets import load_iris
# from sklearn.model_selection import train_test_split
# data = load_iris()
# X_train, X_test, y_train, y_test = train_test_split(data['data'], data['target'], test_size=.2)
# # create model instance
# bst = XGBClassifier(n_estimators=2, max_depth=2, learning_rate=1, objective='binary:logistic')
# # fit model
# bst.fit(X_train, y_train)
# # make predictions
# preds = bst.predict(X_test)

data = pd.read_csv('data.csv')
X = data.drop('delay', axis=1)
y = data['delay']
dtrain = xgb.DMatrix(X, label=y)
params = {
    'objective': 'reg:linear',
    'eval_metric': 'mape'
}

num_rounds = 100
num_folds = 10
early_stopping_rounds = 10
cv_results = xgb.cv(params, dtrain, num_rounds, num_folds, early_stopping_rounds=early_stopping_rounds, verbose_eval=True)
best_rounds = cv_results['test-mape-mean'].idxmin() + 1
print("Best evaluation MAPE:", cv_results['test-mape-mean'].min())
best_model = xgb.train(params, dtrain, best_rounds)

# save model
best_model.save_model('xgb_best_model.model')

# load model and make predictions
loaded_model = xgb.Booster()
loaded_model.load_model('xgb_best_model.model')
preds = loaded_model.predict(dtrain)
# set pandas to display all rows
pd.set_option('display.max_rows', None)
# show the original data and predictions
print(pd.concat([y, pd.Series(preds, name='delay_pred')], axis=1))
