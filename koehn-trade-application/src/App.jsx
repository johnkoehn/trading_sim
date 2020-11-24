import React from 'react';
import './App.css';
import Settings from './settings/Settings';

class App extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            config: undefined,
            validationErrors: []
        };
    }

    onConfigChange(config, validationErrors) {
        this.setState({
            config,
            validationErrors
        });
    }

    render() {
        console.log(this.state.config);
        console.log(this.state.validationErrors);
        return (
            <div className="App" key="App">
                <Settings onConfigChange={(config, validationErrors) => this.onConfigChange(config, validationErrors)}> </Settings>
            </div>
        );
    }
}

export default App;
