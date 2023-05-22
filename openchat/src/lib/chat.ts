import { server } from './config';

export let join = async (handle: string): Promise<string> => {
    let url = new URL('join', server);
    let response = await fetch(url, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ handle: handle })
    });
    if (response.status != 200) {
        throw 'Cannot join room, ' + (await response.text());
    } else {
        let js = await response.json();
        return js['cid'];
    }
};

export let onlineCount = async (topic: string): Promise<number> => {
    let url = new URL('online', server);
    let response = await fetch(url);
    if (response.status != 200) {
        return -1;
    } else {
        let js = await response.json();
        return js[topic];
    }
}
