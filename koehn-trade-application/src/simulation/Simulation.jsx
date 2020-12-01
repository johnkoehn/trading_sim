import React from 'react';
import fetch from 'node-fetch';
import Poll from './Poll';
import AverageFitness from './Graphs/AverageFitness';
import BestBot from './BestBot';
import './Simulation.css';

class Simulation extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            runningSimulation: false,
            simulationErrorMessage: undefined,
            simulationId: undefined,
            generations: []
        };
    }

    // eslint-disable-next-line class-methods-use-this
    onStatusUpdate(status, newGenerations) {
        if (status === 'COMPLETED') {
            this.setState({
                runningSimulation: false
            });
        }

        if (newGenerations.length > 0) {
            this.setState((prevState) => {
                return {
                    generations: prevState.generations.concat(newGenerations)
                };
            });
        }
    }

    async runSimulation() {
        this.setState({
            simulationErrorMessage: undefined,
            generations: []
        });

        const runSimulationResponse = await fetch(`${process.env.REACT_APP_SIMULATION_HOST}/simulations`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(this.props.config)
        });

        if (runSimulationResponse.status === 400) {
            this.setState({
                simulationErrorMessage: 'Fix Config Validation Errors'
            });
            return;
        }

        if (!runSimulationResponse.ok) {
            this.setState({
                simulationErrorMessage: 'Faild to run Simulation'
            });
            return;
        }

        const simulationId = (await runSimulationResponse.json()).id;
        this.setState({
            runningSimulation: true,
            simulationId
        });
    }

    render() {
        const getButtonText = () => {
            return this.state.runningSimulation ?
                'Running Simulation' :
                'Run Simualtion';
        };

        return (
            <>
                <div className="main">
                    <p>Simulation</p>
                    <button type="button" disabled={this.state.runningSimulation} onClick={this.runSimulation.bind(this)}>{getButtonText()}</button>
                    <p className="errorMessage">{this.state.simulationErrorMessage}</p>
                    <Poll
                        simulationId={this.state.simulationId}
                        runningSimulation={this.state.runningSimulation}
                        onStatusUpdate={(status, newGenerations) => this.onStatusUpdate(status, newGenerations)}
                        generations={this.state.generations}
                    />
                    <AverageFitness generations={this.state.generations} />
                    <div className="bot">
                        <BestBot generations={this.state.generations} runningSimulation={this.state.runningSimulation} />
                    </div>
                </div>
            </>
        );
    }
}

export default Simulation;
