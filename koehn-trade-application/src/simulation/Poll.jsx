import React from 'react';
import useInterval from '../utils/useInterval';

const POLL_INTERVAL = 1000; // 1 second

async function getStatus(simulationId) {
    const response = await fetch(`${process.env.REACT_APP_SIMULATION_HOST}/simulations/${simulationId}/status`, {
        method: 'GET'
    });

    if (!response.ok) {
        throw new Error('Failed to get simulation state');
    }

    return (await response.json()).status;
}

async function getNewGenerations(simulationId, generations) {
    const listGenerationsResponse = await fetch(`${process.env.REACT_APP_SIMULATION_HOST}/simulations/${simulationId}/generations`, {
        method: 'GET'
    });

    if (!listGenerationsResponse.ok) {
        throw new Error('Failed to list generations');
    }

    const generationsIds = await listGenerationsResponse.json();
    const currentPosition = generations.length;
    if (currentPosition >= generationsIds.length) {
        return [];
    }

    const newGenerations = await Promise.all(
        generationsIds.splice(currentPosition).map(async (id) => {
            const response = await fetch(`${process.env.REACT_APP_SIMULATION_HOST}/simulations/${simulationId}/generations/${id}`);

            if (!response.ok) {
                throw new Error(`Failed to get generation ${id}`);
            }

            return response.json();
        })
    );

    return newGenerations;
}

function Poll(props) {
    useInterval(async () => {
        if (!props.runningSimulation) {
            return;
        }

        const { simulationId, generations } = props;

        const status = await getStatus(simulationId);
        const newGenerations = await getNewGenerations(simulationId, generations);
        props.onStatusUpdate(status, newGenerations);
    }, POLL_INTERVAL);

    return (
        <></>
    );
}

export default Poll;
