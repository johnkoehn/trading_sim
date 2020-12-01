/* eslint-disable no-loop-func */
import React from 'react';
import { BarChart, Bar, CartesianGrid, XAxis, YAxis, Legend, Tooltip } from 'recharts';
import { DateTime } from 'luxon';

const YEAR_MONTH_FORMAT = 'yyyy-MM';

function buildData(bot) {
    const soldHoldings = bot.soldHoldings;
    const startYearMonth = DateTime.fromSeconds(bot.startTime).toFormat(YEAR_MONTH_FORMAT);
    const endYearMonth = DateTime.fromSeconds(bot.endTime).toFormat(YEAR_MONTH_FORMAT);
    const chartDate = [];

    // build the year month hash
    let currentYearMonth = startYearMonth;
    while (currentYearMonth <= endYearMonth) {
        const buys = soldHoldings.filter((holding) => DateTime.fromSeconds(holding.purchaseTime).toFormat(YEAR_MONTH_FORMAT) === currentYearMonth);
        const sells = soldHoldings.filter((holding) => DateTime.fromSeconds(holding.sellTime).toFormat(YEAR_MONTH_FORMAT) === currentYearMonth);

        const { wins, losses } = sells.reduce((acc, sell) => {
            return {
                wins: sell.win ? acc.wins + 1 : acc.wins,
                losses: !sell.win ? acc.losses + 1 : acc.losses
            };
        }, { wins: 0, losses: 0 });

        chartDate.push({
            name: currentYearMonth,
            buys: buys.length,
            sells: sells.length,
            wins,
            losses
        });

        currentYearMonth = DateTime.fromFormat(currentYearMonth, YEAR_MONTH_FORMAT).plus({ month: 1 }).toFormat(YEAR_MONTH_FORMAT);
    }

    return chartDate;
}

function PurchaseHistory(props) {
    if (!props.bot) {
        return (
            <></>
        );
    }

    const bot = props.bot;

    const data = buildData(bot);

    return (
        <BarChart width={1000} height={300} data={data}>
            <CartesianGrid stroke="#ccc" />
            <XAxis dataKey="name" />
            <YAxis />
            <Legend />
            <Tooltip />
            <Bar dataKey="buys" fill="#8884d8" />
            <Bar dataKey="sells" fill="#ffc658" />
            <Bar dataKey="wins" fill="#82ca9d" />
            <Bar dataKey="losses" fill="#fc030f" />
        </BarChart>
    );
}

export default PurchaseHistory;
