import React from 'react';
import fetch from 'node-fetch';
import './Settings.css';

const getInputType = (type) => {
    if (type === 'float' || type === 'integer') {
        return 'number';
    }

    return 'text';
};

const mapConfigObject = (currentField, currentFieldName, html = []) => {
    // const fieldNames = Object.keys(currentField);
    html.push((<h3>{currentField.Label}</h3>));

    const fieldNames = Object.keys(currentField.Fields);

    fieldNames.forEach((fieldName) => {
        const field = currentField.Fields[fieldName];
        const fieldType = field.Type;
        if (fieldType === 'object') {
            mapConfigObject(field, fieldName, html);
            return;
        }

        const label = field.Label;
        console.log(field);
        html.push((
            <>
                <label type="text" htmlFor={field}>{label}</label>
                <input type={getInputType(fieldType)} />
            </>
        ));
    });

    return html;
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

        const response = await fetch(`${process.env.REACT_APP_SIMULATION_HOST}/configs/form`);

        if (!response.ok) {
            throw Error('Failed to get default settings!');
        }

        const configForm = await response.json();
        this.setState({
            configForm
        });
    }

    componentWillUnmount() {
        // eslint-disable-next-line no-underscore-dangle
        this._isMounted = false;
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
                const additionalHtml = mapConfigObject(field, fieldName);
                return accumulator.concat(additionalHtml);
            }

            const label = field.Label;
            accumulator.push((
                <>
                    <label type="text" htmlFor={field}>{label}</label>
                    <input type={getInputType(fieldType)} />
                </>
            ));
            return accumulator;
        }, []);
    }

    render() {
        return (
            <form>
                {this.buildSettings()}
            </form>
        );
    }
}

export default Settings;
