import { Rpc } from '../modules/Rpc.js'

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

    test('should start from the beginning end of list is reached', () => {
        const boba = new Rpc('boba');
        expect(boba.node).toBe('https://mainnet.boba.network');
        expect(boba.node).toBe('https://boba-ethereum.gateway.tenderly.co');
        expect(boba.node).toBe('https://gateway.tenderly.co/public/boba-ethereum');
        expect(boba.node).toBe('https://mainnet.boba.network');
    });

    test('should throw error for invalid network', () => {
        const createInvalidRpc = () => {
            new Rpc('invalidNetwork');
        };
        expect(createInvalidRpc).toThrow(Error);
    });
});

