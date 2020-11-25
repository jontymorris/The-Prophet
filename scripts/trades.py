import json
import pandas
import os
from datetime import datetime
from matplotlib import pyplot
from tqdm import tqdm
from time import sleep


BASE = '../backtest/assets/'

def get_symbols():
    stocks = []

    for market in ['nz', 'us']:
        with open(BASE + f'stocks_{market}.json') as handle:
            contents = handle.read()
            stocks += json.loads(contents)

    return [stock['symbol'] for stock in stocks]

def get_history(symbol):
    path = BASE + f'history/{symbol.upper()}.csv'

    history = pandas.read_csv(path)
    history['Date'] = pandas.to_datetime(history['Date'])

    return history

def get_trades():
    with open(BASE + 'trades.json') as handle:
        contents = handle.read()
        trades = json.loads(contents)

        for trade in trades:
            trade['buy_date'] = datetime.strptime(trade['buy_date'], '%Y-%m-%d')
            trade['sell_date'] = datetime.strptime(trade['sell_date'], '%Y-%m-%d')

        return trades

def empty_graphs_folder():
    files = os.listdir(BASE + 'graphs')
    
    for item in files:
        path = os.path.join(BASE, 'graphs', item)
        os.remove(path)


symbols = get_symbols()
trades = get_trades()

empty_graphs_folder

print('Generating images')
for symbol in tqdm(symbols):
    try:
        history = get_history(symbol)
        symbol_trades = [trade for trade in trades if trade['symbol'] == symbol]

        if len(symbol_trades) == 0:
            continue

        fig = pyplot.gcf()
        fig.set_size_inches(18.5, 10.5)
        
        buy_prices = [trade['buy_price'] for trade in symbol_trades]
        buy_dates = [trade['buy_date'] for trade in symbol_trades]

        sell_prices = [trade['sell_price'] for trade in symbol_trades]
        sell_dates = [trade['sell_date'] for trade in symbol_trades]

        pyplot.plot(history['Date'].values, history['Close'].values, label='Price/Time')
        pyplot.plot_date(buy_dates, buy_prices, label='Buys')
        pyplot.plot_date(sell_dates, sell_prices, label='Sells')

        pyplot.savefig(BASE + f'graphs/{symbol}.jpeg')
        pyplot.show()
    except:
        pass

print('Done.')