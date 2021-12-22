<template>
  <div id="root">
    <SignedOut v-show="!isSignedIn" />
    <SignedIn v-show="isSignedIn" />
  </div>
</template>

<script>
import "./global.css"
import getConfig from "./config"
import SignedOut from "./components/SignedOut.vue"
import SignedIn from "./components/SignedIn.vue"

const nearConfig = getConfig(process.env.NODE_ENV || "development")
console.log(
  `networkId:${nearConfig.networkId} CONTRACT_NAME:${nearConfig.contractName}`
)
window.networkId = nearConfig.networkId

export default {
  created() {
    document.title = "offline-payment-rust"
  },
  name: "App",
  components: {
    SignedOut,
    SignedIn,
  },

  computed: {
    isSignedIn() {
      return window.walletConnection.isSignedIn()
    },
  },
}
</script>

