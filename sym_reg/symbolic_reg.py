from pysr import PySRRegressor
import pandas as pd
import numpy as np
import sympy
from sklearn.preprocessing import StandardScaler
from sklearn.model_selection import train_test_split
from symbolic_reg_model import *
import argparse


def parse_table(path):
    # parse the csv table to pandas dataframe
    df = pd.read_csv(path)
    print(df)
    return df
    
def validate_model(X, y, model_file):
    model = (PySRRegressor.from_file(model_file)).set_params(
        extra_sympy_mappings={"cos2": lambda x: sympy.cos(x)**2})
    model.refresh()
    pred_y = model.predict(X)
    np.set_printoptions(suppress=True)
    pred_y = np.array([np.format_float_positional(i, trim='-') for i in pred_y])
    for t in list(zip(y, pred_y)):
        print(t)
    exit()

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description="Symbolic regression model selection and validation")
    parser.add_argument('--model-file', type=str, default=None, help='The path to the model file')
    parser.add_argument('--validate', action='store_true', help='Determin whether to validate the model')
    parser.add_argument('--model', type=int, default=1, help='Determin which model to use')
    args = parser.parse_args()
    
    df = parse_table("simple_circuit_analysis_large.csv")
    # convert the dataframe to numpy array, X = [power, lev, area, delay, *, !, +]
    #X = df.iloc[:, :6].to_numpy()
    # X -> 1,2,3,5,6
    #X = df.iloc[:, [0, 1, 2, 4, 5]].to_numpy()
    X = df.iloc[:,:6].to_numpy()
    scaler = StandardScaler()
    X = scaler.fit_transform(X)
    # y = [&]
    y = df.iloc[:, -1].to_numpy()

    if args.validate: validate_model(X, y, args.model_file)

    X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)
    model = model1 if args.model == 1 else model2 if args.model == 2 else model3 if args.model == 3 else model4 if args.model == 4 else model5 if args.model == 5 else model6 if args.model == 6 else None
    model.fit(X_train, y_train)
    
    y_pred = model.predict(X_test)
    mse = np.mean((y_test - y_pred)**2)
    mae = np.mean(np.abs(y_test - y_pred))
    # add RRSE
    rrse = np.sqrt(np.sum((y_test - y_pred)**2) / np.sum((y_test - np.mean(y_test))**2))
    # add MAPE: \text { MAPE }=\frac{1}{n} \sum_{i=1}^n \frac{\left|y_i-\hat{y}_i\right|}{y_i} \times 100 \%
    mape = 100 * np.mean(np.abs((y_test - y_pred) / y_test))
    print(f"MSE: {mse}, MAE: {mae}", f"MAPE: {mape}", f"RRSE: {rrse}")