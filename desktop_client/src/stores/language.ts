import { writable } from "svelte/store"


export type Language = 'auto' | 'ru' | 'en' | 'de'

export const languageStore = writable<Language>('en')

