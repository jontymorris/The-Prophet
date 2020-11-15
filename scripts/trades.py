import json
import pandas
import os
from datetime import datetime
from matplotlib import pyplot
from tqdm import tqdm
from time import sleep


BASE = '../backtest/assets/'
MARKET = 'us'

def get_symbols():
    with open(BASE + f'stocks_{MARKET}.json') as handle:
        contents = handle.read()
        stocks = json.loads(contents)

        return [stock['symbol'] for stock in stocks]

def get_history(symbol):
    path = BASE + f'history/{symbol.upper()}.csv'

    history = pandas.read_csv(path)
    history['Date'] = pandas.to_datetime(history['Date'])
    history = history[history['Date'] >= '2018-01-01']

    return history

def get_trades():
    with open(BASE + 'trades.json') as handle:
        contents = handle.read()
        return json.loads(contents)

def find_trades(symbol, was_buying, all_trades):
    found = []
    
    for trade in all_trades:
        if trade['symbol'] == symbol:
            if trade['was_buying'] == was_buying:
                trade['date'] = datetime.strptime(trade['date'], '%Y-%m-%d')
                found.append(trade)
    
    return found

def empty_output_folder():
    files = os.listdir(BASE + 'output')
    for item in files:
        path = os.path.join(BASE, 'output', item)
        os.remove(path)


symbols = get_symbols()
trades = get_trades()

empty_output_folder()

print('Generating images')
for symbol in tqdm(symbols):
    try:
        history = get_history(symbol)
        buys = find_trades(symbol, True, trades)
        sells = find_trades(symbol,False, trades)

        if len(buys) == 0 and len(sells) == 0:
            continue

        fig = pyplot.gcf()
        fig.set_size_inches(18.5, 10.5)
        
        pyplot.plot(history['Date'].values, history['Close'].values, label='Price/Time')
        pyplot.plot_date([trade['date'] for trade in buys], [trade['price'] for trade in buys], label='Buys')
        pyplot.plot_date([trade['date'] for trade in sells], [trade['price'] for trade in sells], label='Sells')

        pyplot.savefig(BASE + f'output/{symbol}.jpeg')
        pyplot.show()
    except:
        pass

print('Done.')