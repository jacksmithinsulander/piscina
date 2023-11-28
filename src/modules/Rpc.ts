//import { rpcList } from '../data/rpcList';

interface RpcList {
    [network: string]: number;
}


class Rpc {
    public network: string;
    private list: RpcList = {};

    constructor(network: string) {
        this.network = network;

        if (!this.list[network]) {
            this.list[network] = 0;
        }
    }

    get node() {
        const networkList = this.list[this.network];
        this.list[this.network] += 1;
        return networkList;
    }
}

export {Rpc};
