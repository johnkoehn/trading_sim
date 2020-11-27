import React from 'react';
import fetch from 'node-fetch';
import './Simulation.css';

class Simulation extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            config: undefined,
            validationErrors: [],
            runningSimulation: false,
            simulationErrorMessage: undefined
        };
    }

    onConfigChange(config, validationErrors) {
        this.setState({
            config,
            validationErrors
        });
    }

    async runSimulation() {
        this.setState({
            simulationErrorMessage: undefined
        });

        const runSimulationResponse = await fetch(`${process.env.REACT_APP_SIMULATION_HOST}/simulations`, {
            method: 'POST'
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

        this.setState({
            runningSimulation: true
        });
    }

    render() {
        const getButtonText = () => {
            return this.state.runningSimulation ?
                'Running Simulation' :
                'Run Simualtion';
        };

        console.log(this.state.config);
        console.log(this.state.validationErrors);
        return (
            <div className="main">
                <p>Here is the simulation page</p>
                <button type="button" disabled={this.state.runningSimulation} onClick={this.runSimulation.bind(this)}>{getButtonText()}</button>
                <p className="errorMessage">{this.state.simulationErrorMessage}</p>
            </div>
        );
    }
}

export default Simulation;
