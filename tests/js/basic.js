import { init, QuantumCircuit, circuits, utils } from 'qucom-sdk';

/* 

Test script for QUCOM SDK 
PLATFORM: NODE

All scripts are pre-tested in Rust, so no need to test quantum logic here.

*/
async function main() {
    try {
        await init();
        const bellCircuit = circuits.bell();
        console.log('Bell state circuit created');

        bellCircuit.measure();
        const result = bellCircuit.execute();
        console.log('Bell state measurement:', result);

        const stats = utils.runTrials(() => circuits.bell(), 1000);
        console.log('Bell state trial stats:', stats);

        const ghzCircuit = circuits.ghz(3);
        ghzCircuit.measure();
        const ghzResult = ghzCircuit.execute();
        console.log('GHZ state measurement:', ghzResult);

        const rawCircuit = new QuantumCircuit(2);
        rawCircuit.h(0).cx(0, 1).measure();
        console.log('Raw circuit measurement:', rawCircuit.execute());

		console.log('QASM output of Bell circuit:\n', bellCircuit.toQASM());
    } catch (err) {
        console.error('Error testing QUCOM SDK:', err);
    }
}

main();

