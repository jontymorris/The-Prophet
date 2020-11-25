import json
from datetime import datetime
from matplotlib import pyplot


BASE = '../backtest/assets/'

def get_trades():
    with open(BASE + 'trades.json') as handle:
        contents = handle.read()
        return json.loads(contents)

def get_profit(trade):
    buy_amount = trade['buy_price'] * trade['quantity']
    sell_amount = trade['sell_price'] * trade['quantity']

    return sell_amount - buy_amount

def get_date(trade):
    return datetime.strptime(trade['sell_date'], '%Y-%m-%d')

def get_daily_profits(dates, profits):
    daily_profits = {}
    for i in range(0, len(profits)):
        profit = profits[i]
        date = dates[i]

        if date in daily_profits:
            daily_profits[date] += profit
        else:
            daily_profits[date] = profit
    
    return daily_profits

def add_consecutive_profits(profits):
    total_profit = 0
    for i in range(0, len(profits)):
        total_profit += profits[i]
        profits[i] = total_profit
    
    return profits

trades = get_trades()
profits = [get_profit(trade) for trade in trades]
dates = [get_date(trade) for trade in trades]

daily_profits = get_daily_profits(dates, profits)

dates = list(daily_profits.keys())
dates.sort()

profits = [daily_profits[date] for date in dates]
profits = add_consecutive_profits(profits)

fig = pyplot.gcf()
fig.set_size_inches(18.5, 10.5)

pyplot.plot(dates, profits)
pyplot.show()
