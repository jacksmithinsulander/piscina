import { Rpc } from '../modules/Rpc'

describe('Rpc Class', () => {
    let eth: Rpc;

    beforeEach(() => {
        eth = new Rpc('eth');
    });

    test('should return an instance of Rpc', () => {
        expect(eth instanceof Rpc).toBe(true);
    });

    test('should return incremented indexes', () => {
        expect(eth.node).toBe('https://endpoints.omniatech.io/v1/eth/mainnet/public');
        expect(eth.node).toBe('https://eth.llamarpc.com');
        expect(eth.node).toBe('https://rpc.ankr.com/eth');
    });

    test('should handle multiple instances with separate mappings', () => {
        const matic = new Rpc('matic');
        expect(eth.node).toBe('https://endpoints.omniatech.io/v1/eth/mainnet/public');
        expect(matic.node).toBe('https://polygon-bor.publicnode.com');
    });
});

