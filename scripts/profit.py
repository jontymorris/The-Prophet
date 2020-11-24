import json
from datetime import datetime
from matplotlib import pyplot


BASE = '../backtest/assets/'

def get_trades():
    with open(BASE + 'trades.json') as handle:
        contents = handle.read()
        return json.loads(contents)

def get_trades_by_symbol(trades):
    symbols = {}

    for trade in trades:
        symbol = trade['symbol']
        if symbol in symbols:
            symbols[symbol].append(trade)
        else:
            symbols[symbol] = [trade]
    
    return symbols

def get_daily_profit(symbol_trades):
    daily_profit = {}

    for symbol in symbol_trades.keys():
        trades = symbol_trades[symbol]
        for i in range(0, len(trades), 2):
            if i > len(trades) - 2:
                break

            buy_price = trades[i]['price']
            sell_price = trades[i+1]['price']
            quantity = trades[i]['quantity']

            bought = buy_price * quantity
            sold = sell_price * quantity 

            profit = sold - bought

            date = trades[i+1]['date']
            date = datetime.strptime(date, '%Y-%m-%d')

            if date in daily_profit:
                daily_profit[date] += profit
            else:
                daily_profit[date] = profit
    
    return daily_profit

trades = get_trades()
symbols = get_trades_by_symbol(trades)
daily_profit = get_daily_profit(symbols)

dates = list(daily_profit.keys())
dates.sort()

profits = [daily_profit[date] for date in dates]

fig = pyplot.gcf()
fig.set_size_inches(18.5, 10.5)

pyplot.plot_date(dates, profits)
pyplot.show()
