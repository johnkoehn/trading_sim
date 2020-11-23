import React from 'react';
import fetch from 'node-fetch';
import objectPath from 'object-path';
import './Settings.css';

const getInputType = (type) => {
    if (type === 'float' || type === 'integer') {
        return 'number';
    }

    return 'text';
};

class Settings extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            configForm: undefined
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
            <h3>{currentField.Label}</h3> :
            <label type="text" className={`level-${level}`} htmlFor={currentFieldName}>{currentField.Label}</label>;

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
                    <label type="text" className={`label-${level}`} htmlFor={currentFieldName} path={path}>{label}</label>
                    <input type={getInputType(fieldType)} path={path} onChange={this.onSettingChange.bind(this)} value={value} />
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
        return fieldNames.reduce((accumulator, fieldName) => {
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
    }

    onSettingChange(event) {
        const path = event.target.getAttribute('path');
        const value = event.target.value;

        this.setState((prevState) => {
            const config = prevState.config;
            objectPath.set(config, path, value);
            return {
                config
            };
        });
    }

    render() {
        return (
            <form>
                <h3>Simulation Settings</h3>
                {this.buildSettings()}
            </form>
        );
    }
}

export default Settings;
