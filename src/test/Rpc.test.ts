import { Rpc } from '../modules/Rpc'

const rpc: Rpc = new Rpc("PulseChain");

test('should return an instance of Rpc', () => {
    expect(rpc instanceof Rpc);
});