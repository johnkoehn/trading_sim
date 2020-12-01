import React from 'react';
import { LineChart, Line, CartesianGrid, XAxis, YAxis, Legend, Tooltip } from 'recharts';
import { DateTime } from 'luxon';

function buildData(bot) {
    const valueHistory = bot.valueHistory;
    return valueHistory.map((history) => {
        return {
            value: history.value,
            date: DateTime.fromSeconds(history.time).toFormat('yyyy-MM-dd-hh-mm')
        };
    });
}

function ValueHistory(props) {
    if (!props.bot) {
        return (
            <></>
        );
    }

    const data = buildData(props.bot);

    return (
        <LineChart width={1000} height={300} data={data}>
            <Line type="monotone" dataKey="value" stroke="#8884d8" />
            <CartesianGrid stroke="#ccc" />
            <XAxis dataKey="date" />
            <YAxis />
            <Tooltip />
            <Legend />
        </LineChart>
    );
}

export default ValueHistory;
