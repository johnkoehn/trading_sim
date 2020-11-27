import React from 'react';
import Select from 'react-select';
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

const getConfigOptions = async () => {
    const listConfigsResponse = await fetch(`${process.env.REACT_APP_SIMULATION_HOST}/configs`);

    if (!listConfigsResponse.ok) {
        throw Error('Failed to get list of configs');
    }

    const configNames = await listConfigsResponse.json();

    return configNames.map((configName) => {
        return {
            value: configName,
            label: configName.replaceAll('_', '')
        };
    });
};

class Settings extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            configForm: undefined,
            config: undefined,
            configOptions: [],
            selectedConfigOption: undefined,
            configName: undefined,
            validationErrors: []
        };
    }

    async componentDidMount() {
        // eslint-disable-next-line no-underscore-dangle
        this._isMounted = true;

        const [defaultConfigResponse, configFormResponse, configOptions] = await Promise.all([
            fetch(`${process.env.REACT_APP_SIMULATION_HOST}/configs/default`),
            fetch(`${process.env.REACT_APP_SIMULATION_HOST}/configs/form`),
            getConfigOptions()
        ]);

        if (!defaultConfigResponse.ok) {
            throw Error('Failed to get default settings!');
        }

        if (!configFormResponse.ok) {
            throw Error('Failed to get config form');
        }

        const defaultConfig = await defaultConfigResponse.json();
        const configForm = await configFormResponse.json();

        const selectedConfigOption = configOptions.find((option) => option.value === 'default');

        this.setState({
            configForm,
            config: defaultConfig,
            configOptions,
            selectedConfigOption,
            configName: selectedConfigOption.label
        });

        this.props.onConfigChange(defaultConfig, []);
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

        return settingsHtml;
    }

    // TODO: Rewrite to not validate on each change
    async onSettingChange(event) {
        const getValue = (type, fieldType) => {
            const value = event.target.value;

            if (type === 'number') {
                if (value === '') {
                    return 0;
                }

                return fieldType === 'float' ?
                    parseFloat(event.target.value) :
                    parseInt(event.target.value);
            }

            return value;
        };

        const path = event.target.getAttribute('path');
        const type = event.target.getAttribute('type');
        const fieldType = event.target.getAttribute('fieldtype');

        const value = getValue(type, fieldType);

        if (Number.isNaN(value)) {
            return;
        }

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

    async handleConfigChange(selectedOption) {
        const configName = selectedOption.value;
        const getConfigResponse = await fetch(`${process.env.REACT_APP_SIMULATION_HOST}/configs/${configName}`);

        if (!getConfigResponse.ok) {
            throw new Error(`Failed to get config ${configName}`);
        }

        const config = await getConfigResponse.json();
        this.setState({
            selectedConfigOption: selectedOption,
            config,
            configName: selectedOption.label
        });
    }

    // eslint-disable-next-line class-methods-use-this
    async saveConfig() {
        // turn spaces to underscores
        const configName = this.state.configName;
        const configNameNoSpaces = configName.replaceAll(' ', '_');
        const config = this.state.config;

        // make put request
        const saveConfigResponse = await fetch(`${process.env.REACT_APP_SIMULATION_HOST}/configs/${configNameNoSpaces}`, {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(config)
        });

        if (!saveConfigResponse.ok) {
            console.log(saveConfigResponse.status);
            throw new Error('Failed to save config');
        }

        const configOptions = await getConfigOptions();
        const selectedConfigOption = configOptions.find((option) => option.value === configNameNoSpaces);
        this.setState({
            configOptions,
            selectedConfigOption
        });
    }

    updateConfigName(event) {
        const value = event.target.value;

        this.setState({
            configName: value
        });
    }

    buildConfigSelection() {
        return (<Select options={this.state.configOptions} value={this.state.selectedConfigOption} onChange={(event) => this.handleConfigChange(event)} />);
    }

    buildSaveConfig() {
        return (
            <>
                <label htmlFor="configName" type="text">
                    Config Name:&emsp;
                    <input type="text" id="configName" value={this.state.configName} onChange={this.updateConfigName.bind(this)} />
                </label>
                <button type="button" onClick={this.saveConfig.bind(this)}>Save Config</button>
            </>
        );
    }

    render() {
        // settingsHtml.push(());
        if (!this.state.configForm) {
            return (
                <div>
                    <p>Fetching settings</p>
                </div>
            );
        }

        return (
            <form key="settings">
                <h3>Simulation Settings</h3>
                {this.buildConfigSelection()}
                {this.buildSettings()}
                {this.buildSaveConfig()}
            </form>
        );
    }
}

export default Settings;
