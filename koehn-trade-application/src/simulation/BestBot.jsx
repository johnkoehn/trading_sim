import React from 'react';
import ReactJson from 'react-json-view';
import PurchaseHistory from './Graphs/PurchaseHistory';

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

function calculateBotStats(bestBot) {
    const soldHoldings = bestBot.soldHoldings;
    const { wins, losses } = soldHoldings.reduce((acc, holding) => {
        return {
            wins: holding.win ? acc.wins + 1 : acc.wins,
            losses: !holding.win ? acc.losses + 1 : acc.losses
        };
    }, {
        wins: 0,
        losses: 0
    });

    return {
        wins,
        losses,
        winRation: (wins / soldHoldings.length) * 100
    };
}

function renderBotStats(runningSimulation, bestBot) {
    if (runningSimulation) {
        return (
            <></>
        );
    }

    return (
        <>
            <PurchaseHistory bot={bestBot} />
            <ReactJson src={calculateBotStats(bestBot)} theme="monokai" />
        </>
    );
}

function BestBot(props) {
    if (props.generations.length === 0) {
        return (
            <></>
        );
    }

    const bestBot = findBestBot(props.generations);

    const bestBotSimple = JSON.parse(JSON.stringify(bestBot));
    bestBotSimple.numberOfSoldHolding = bestBotSimple.soldHoldings.length;
    delete bestBotSimple.soldHoldings;
    return (
        <>
            <ReactJson src={bestBotSimple} theme="monokai" />
            {renderBotStats(props.runningSimulation, bestBot)}
        </>
    );
}

export default BestBot;
