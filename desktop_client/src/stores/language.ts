import { writable } from "svelte/store"


type Language = 'auto' | 'ru' | 'en' | 'de'

export const languageStore = writable<Language>('en')

