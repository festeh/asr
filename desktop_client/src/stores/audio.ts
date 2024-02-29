import { writable } from "svelte/store";



const audio: HTMLAudioElement = new Audio();
const {set, subscribe, update} = writable(audio);

export const audioStore = {
    subscribe,
    set,
    update,
};
