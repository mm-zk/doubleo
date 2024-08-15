// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  app: {
    head: {
      title: "zkSync + wagmi + Nuxt 3"
    }
  },

  ssr: false,
  devtools: { enabled: true },
  modules: ['@pinia/nuxt'],

  pinia: {
    autoImports: [
      // automatically imports `defineStore`
      "defineStore",
      "storeToRefs",
    ],
  },

  imports: {
    dirs: ["store"],
  },

  compatibilityDate: "2024-08-15",
})