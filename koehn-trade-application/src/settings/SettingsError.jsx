import React from 'react';
import './SettingsError.css';

function SettingsError(props) {
    const errors = props.errors.filter((error) => error.path === props.path);

    if (errors.length === 0) {
        return (
            <>
            </>
        );
    }

    const errorList = errors.map((error) => <li key={error.message}>{error.message}</li>);

    return (
        <ul>
            {errorList}
        </ul>
    );
}

export default SettingsError;
