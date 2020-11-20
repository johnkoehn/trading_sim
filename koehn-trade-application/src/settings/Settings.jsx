import React from 'react';
import fs from 'fs';

const test = async () => {
    fs.readdirSync('./');
    // console.log(result);
};

const Settings = (props) => {
    return (
        <div>
            <p>{props.dog}</p>
            <button onClick={test} type="button">Click Me!</button>
        </div>
    );
};

export default Settings;
