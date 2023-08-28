# /data/guangyuh/coding_env/AIG2INV/AIG2INV_main/clause-learning/data-collect/stat/size20.csv
from pysr import PySRRegressor
import pandas as pd
import numpy as np
import sympy
from sklearn.preprocessing import StandardScaler
from sklearn.model_selection import train_test_split
# import model1() model2()
from symbolic_reg_model import model1, model2, model3, model4
import argparse


def parse_table(path):
    # parse the csv table to pandas dataframe
    with open(path, 'r') as f:
        df = pd.read_csv(f)
        
        # only keep the row that  res is "unsat"
        df = df[df["res"] == "unsat"]
        print(df)
        return df
    
def validate_model(X, y, model_file):
    model = PySRRegressor.from_file(model_file)
    #print(model.latex())
    #model.refresh()
    #model = PySRRegressor.from_file(model_file)
    #print(model)
    # predict y by feeding X to the model
    pred_y = model.predict(X)
    # print y without scientific notation
    np.set_printoptions(suppress=True)
    pred_y = np.array([np.format_float_positional(i, trim='-') for i in pred_y])
    #print(pred_y[:-20])
    #print(y[:-20])
    # print tuple of (y, pred_y)
    #print(list(zip(y, pred_y)))
    for t in list(zip(y, pred_y)):
        print(t)
    #exit
    #print(model.latex())
    exit()

if __name__ == '__main__':
    # add parser
    parser = argparse.ArgumentParser(description="Convert aig to aag automatically")
    parser.add_argument('--model-file', type=str, default=None, help='The path to the model file')
    parser.add_argument('--validate', action='store_true', help='Determin whether to validate the model')
    parser.add_argument('--model', type=int, default=1, help='Determin which model to use')
    args = parser.parse_args()
    
    '''
    --------------------Get the aig list (and their path)-------------------
    '''
    # read table at first    
    df = parse_table("/data/guangyuh/coding_env/AIG2INV/AIG2INV_main/clause-learning/data-collect/stat/size20.csv")
    # prepare data
    #X = 2 * np.random.randn(100, 5)
    
    # convert the dataframe to numpy array, X = [M, I, L, O, A]
    df = df.drop(columns=["O"])
    X = df.iloc[:, 1:5].to_numpy()
    scaler = StandardScaler()
    X = scaler.fit_transform(X)
    # y = [n_clauses]
    y = df.iloc[:, -3].to_numpy() # -3 -> time, -1 -> n_clauses
    
    '''
    -------------------------------------Validation-------------------------------------
    '''
    if args.validate: validate_model(X, y,args.model_file)
    # will exit if validation finished

    '''
    -------------------------------------Model Selection and train-----------------------------
    '''
    # Split data into training and testing sets
    X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)
    # select model 
    model = model1 if args.model == 1 else model2 if args.model == 2 else model3 if args.model == 3 else model4 if args.model == 4 else None
    model.fit(X_train, y_train)
    
    # Evaluate model performance on test data
    y_pred = model.predict(X_test)
    mse = np.mean((y_test - y_pred)**2)
    mae = np.mean(np.abs(y_test - y_pred))
    print(f"MSE: {mse}, MAE: {mae}")
    