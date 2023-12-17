
import { Rpc } from '../models/Rpc.js';
import { Signer } from '../models/Signer.mjs';

describe('Signer Class', () => {
    let signer: Signer;
    let eth: Rpc;

    beforeEach(() => {
        eth = new Rpc('eth');
        signer = new Signer(eth.node); 
    });

    test('should return an instance of Rpc', () => {
        expect(eth instanceof Rpc).toBe(true);
    });

    test('should return an instance of Signer', () => {
        expect(signer instanceof Signer).toBe(true);
    });
});
