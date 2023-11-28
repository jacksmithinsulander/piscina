//import { rpcList } from '../data/rpcList';

interface RpcList {
    [network: string]: number;
}


class Rpc {
    public network: string;
    private static list: RpcList = {};

    constructor(network: string) {
        this.network = network;

        if (!Rpc.list[network]) {
            Rpc.list[network] = 0;
        }
    }

    get node() {
        const networkList = Rpc.list[this.network];
        Rpc.list[this.network] += 1;
        return networkList;
    }
}

export {Rpc};
