import React from 'react';
import { LineChart, Line, CartesianGrid, XAxis, YAxis } from 'recharts';

function buildData(generations) {
    return generations.map((generation, index) => {
        const totalMoney = generation.reduce((total, bot) => total + bot.money, 0);

        return {
            name: index,
            amt: totalMoney / generation.length
        };
    });
}

function Graph(props) {
    if (props.generations.length === 0) {
        return (
            <></>
        );
    }

    const data = buildData(props.generations);

    return (
        <LineChart width={600} height={300} data={data}>
            <Line type="monotone" dataKey="amt" stroke="#8884d8" />
            <CartesianGrid stroke="#ccc" />
            <XAxis dataKey="name" />
            <YAxis />
        </LineChart>
    );
}

export default Graph;
