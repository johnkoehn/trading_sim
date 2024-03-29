import React from 'react';
import { LineChart, Line, CartesianGrid, XAxis, YAxis } from 'recharts';

function buildData(generations) {
    return generations.map((generation, index) => {
        const totalFitness = generation.reduce((total, bot) => total + bot.fitness, 0);

        return {
            name: index + 1,
            amt: totalFitness / generation.length
        };
    });
}

function AverageFitness(props) {
    if (props.generations.length === 0) {
        return (
            <></>
        );
    }

    const data = buildData(props.generations);

    return (
        <LineChart width={1000} height={300} data={data}>
            <Line type="monotone" dataKey="amt" stroke="#8884d8" />
            <CartesianGrid stroke="#ccc" />
            <XAxis dataKey="name" />
            <YAxis />
        </LineChart>
    );
}

export default AverageFitness;
