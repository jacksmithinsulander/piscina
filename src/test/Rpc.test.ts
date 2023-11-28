import { Rpc } from '../modules/Rpc'

describe('Rpc Class', () => {
    let rpc: Rpc;

    beforeEach(() => {
        rpc = new Rpc('PulseChain');
    });

    test('should return an instance of Rpc', () => {
        expect(rpc instanceof Rpc).toBe(true);
    });

    test('should return incremented indexes', () => {
        expect(rpc.node).toBe(0);
        expect(rpc.node).toBe(1);
        expect(rpc.node).toBe(2);
    });

    test('should handle multiple instances with separate mappings', () => {
        const anotherRpc = new Rpc('Ethereum');
        expect(rpc.node).toBe(0);
        expect(anotherRpc.node).toBe(0);
    });
});

