import React from 'react';
import fetch from 'node-fetch';
import * as objectPath from 'object-path-immutable';
import SettingsError from './SettingsError';
import './Settings.css';

const getInputType = (type) => {
    if (type === 'float' || type === 'integer' || type === 'unsigned_integer') {
        return 'number';
    }

    return 'text';
};

const validateConfig = async (path, value, currentConfig) => {
    const updatedConfig = objectPath.set(currentConfig, path, value);
    const response = await fetch(`${process.env.REACT_APP_SIMULATION_HOST}/configs/validate`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(updatedConfig)
    });

    if (response.ok) {
        return [];
    }

    if (response.status !== 400) {
        throw new Error('Failed to validate config file');
    }

    try {
        const body = await response.json();
        return body;
    } catch (err) {
        console.log(err);
        throw new Error('Failed to validate config file');
    }
};

class Settings extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            configForm: undefined,
            config: undefined,
            validationErrors: []
        };
    }

    async componentDidMount() {
        // eslint-disable-next-line no-underscore-dangle
        this._isMounted = true;

        const [defaultConfigResponse, configFormResponse] = await Promise.all([
            fetch(`${process.env.REACT_APP_SIMULATION_HOST}/configs/default`),
            fetch(`${process.env.REACT_APP_SIMULATION_HOST}/configs/form`)
        ]);

        if (!defaultConfigResponse.ok) {
            throw Error('Failed to get default settings!');
        }

        if (!configFormResponse.ok) {
            throw Error('Failed to get config form');
        }

        const defaultConfig = await defaultConfigResponse.json();
        const configForm = await configFormResponse.json();
        this.setState({
            configForm,
            config: defaultConfig
        });
    }

    componentWillUnmount() {
        // eslint-disable-next-line no-underscore-dangle
        this._isMounted = false;
    }

    mapConfigObject(currentField, currentFieldName, level = 0, html = []) {
        const heading = level === 0 ?
            <h3 key={currentField.Label}>{currentField.Label}</h3> :
            <label key={`label-${currentFieldName}`} type="text" className={`level-${level}`} htmlFor={currentFieldName}>{currentField.Label}</label>;

        html.push(heading);

        const fieldNames = Object.keys(currentField.Fields);

        fieldNames.forEach((fieldName) => {
            const field = currentField.Fields[fieldName];
            const fieldType = field.Type;
            if (fieldType === 'object') {
                this.mapConfigObject(field, fieldName, level + 1, html);
                return;
            }

            const label = field.Label;
            const path = field.Path;
            const value = objectPath.get(this.state.config, path);

            html.push((
                <>
                    <label key={`label-${path}`} type="text" className={`label-${level}`} htmlFor={currentFieldName} path={path}>{label}</label>
                    <input key={path} type={getInputType(fieldType)} path={path} onChange={this.onSettingChange.bind(this)} value={value} fieldtype={fieldType} />
                    <SettingsError errors={this.state.validationErrors} path={path} className=".error" />
                </>
            ));
        });

        return html;
    }

    buildSettings() {
        if (!this.state.configForm) {
            return (
                <div>
                    <p>Fetching settings</p>
                </div>
            );
        }

        const configForm = this.state.configForm;
        const fieldNames = Object.keys(configForm);
        const settingsHtml = fieldNames.reduce((accumulator, fieldName) => {
            const field = configForm[fieldName];
            const fieldType = field.Type;
            if (fieldType === 'object') {
                const additionalHtml = this.mapConfigObject(field, fieldName, 0);
                return accumulator.concat(additionalHtml);
            }

            // TODO: CLEAN THIS UP
            const label = field.Label;
            const path = field.path;
            const value = objectPath.get(this.state.config, path);
            accumulator.push((
                <>
                    <label type="text" htmlFor={field}>{label}</label>
                    <input type={getInputType(fieldType)} path={path} onChange={this.onSettingChange.bind(this)} value={value} />
                </>
            ));
            return accumulator;
        }, []);

        // settingsHtml.push((<button type="button" onClick={() => this.props.runSimulation(this.state.config)}>Run Simulation</button>));
        return settingsHtml;
    }

    async onSettingChange(event) {
        const path = event.target.getAttribute('path');
        const type = event.target.getAttribute('type');
        const value = type === 'number' ? parseInt(event.target.value) : event.target.value;

        if (Number.isNaN(value)) {
            return;
        }

        const fieldType = event.target.getAttribute('fieldtype');
        if (fieldType === 'unsigned_integer' && value < 0) {
            return;
        }

        const validationErrors = await validateConfig(path, value, this.state.config);

        this.setState((prevState) => {
            const currentConfig = prevState.config;
            const updatedConfig = objectPath.set(currentConfig, path, value);

            this.props.onConfigChange(updatedConfig, validationErrors);

            return {
                config: updatedConfig,
                validationErrors
            };
        });
    }

    render() {
        return (
            <form key="settings">
                <h3>Simulation Settings</h3>
                {this.buildSettings()}
            </form>
        );
    }
}

export default Settings;
