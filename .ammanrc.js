// @ts-check
'use strict';
const path = require('path');
const accountProviders = require('./js/dist/generated/accounts');

const localDeployDir = path.join(__dirname, 'program', 'target', 'deploy');
const MY_PROGRAM_ID = require("./js/idl/stakebob.json").metadata.address;

function localDeployPath(programName) {
    return path.join(localDeployDir, `${programName}.so`);
}

const programs = [
    {
        label: 'stakebob',
        programId: MY_PROGRAM_ID,
        deployPath: localDeployPath('stakebob')
    },
    // {
    //     label: 'token_metadata',
    //     programId: 'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s',
    //     deployPath: "https://api.metaplex.solana.com"
    // },
];

const validator = {
    programs,
    verifyFees: false,
    limitLedgerSize: 10000000,
};

module.exports = {
    validator,
    relay: {
        accountProviders,
    },
};