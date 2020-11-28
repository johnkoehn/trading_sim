import React from 'react';
import ReactJson from 'react-json-view';

function findBestBotInGeneration(generation) {
    return generation.reduce((bestBot, bot) => {
        if (!bestBot) {
            return bot;
        }

        return bestBot.fitness > bot.fitness ?
            bestBot :
            bot;
    }, undefined);
}

function findBestBot(generations) {
    return generations.reduce((bestBot, currentGeneration) => {
        const bestBotInGeneration = findBestBotInGeneration(currentGeneration);

        if (!bestBot) {
            return bestBotInGeneration;
        }

        return bestBot.fitness > bestBotInGeneration.fitness ?
            bestBot :
            bestBotInGeneration;
    }, undefined);
}

function BestBot(props) {
    if (props.generations.length === 0) {
        return (
            <></>
        );
    }

    const bestBot = findBestBot(props.generations);
    return (
        <ReactJson src={bestBot} theme="monokai" />
    );
}

export default BestBot;
