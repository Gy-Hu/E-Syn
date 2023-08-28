from pysr import PySRRegressor
import pandas as pd
import numpy as np
import sympy
from sklearn.preprocessing import StandardScaler
from sklearn.model_selection import train_test_split
from symbolic_reg_model import model1, model2, model3, model4
import argparse


def parse_table(path):
    # parse the csv table to pandas dataframe
    df = pd.read_csv(path)
    print(df)
    return df
    
def validate_model(X, y, model_file):
    model = PySRRegressor.from_file(model_file)
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
    
    df = parse_table("simple_circuit_analysis.csv")
    # convert the dataframe to numpy array, X = [power, lev, area, delay, *, !, +]
    X = df.iloc[:, :6].to_numpy()
    scaler = StandardScaler()
    X = scaler.fit_transform(X)
    # y = [&]
    y = df.iloc[:, -1].to_numpy()

    if args.validate: validate_model(X, y, args.model_file)

    X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)
    model = model1 if args.model == 1 else model2 if args.model == 2 else model3 if args.model == 3 else model4 if args.model == 4 else None
    model.fit(X_train, y_train)
    
    y_pred = model.predict(X_test)
    mse = np.mean((y_test - y_pred)**2)
    mae = np.mean(np.abs(y_test - y_pred))
    print(f"MSE: {mse}, MAE: {mae}")