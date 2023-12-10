import rpcList from '../data/rpcList.js';

/**
 * Interface defining the index for RPCs by network.
 */
interface RpcIndex {
    [network: string]: number;
}

/**
 * Class representing an RPC.
 */
class Rpc {
    /**
     * The network associated with the RPC.
     */
    public network: string;

    /**
     * Index to keep track of RPCs by network.
     */
    private rpcIndex: RpcIndex = {};

    /**
     * Constructs an RPC instance for a given network.
     * @param {string} network - The network to which the RPC belongs.
     * @throws {Error} Throws an error if the network is not present in the rpcList.
     */
    constructor(network: string) {
        this.network = network;

        if (!rpcList[network]) {
            throw new Error(
                `Network '${network}' is not present in the rpcList.`
            );
        }

        if (!this.rpcIndex[network]) {
            this.rpcIndex[network] = 0;
        }
    }
    
    /**
     * Retrieves the next RPC node for the network.
     * @returns {string} The RPC node URL.
     */
    get node(): string {
        const links: string[] = rpcList[this.network];
        const index: number = this.rpcIndex[this.network] % links.length;
        this.rpcIndex[this.network] += 1;
        return links[index];
    }
}

export { Rpc };

