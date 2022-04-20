<template>
  <div id="root">
    <SignedOut v-show="!isSignedIn" />
    <div v-show="isSignedIn">
      <Vouchers v-show="!isPayment" />
      <VoucherPayment v-show="isPayment" />
    </div>
  </div>
</template>

<script>
import "./global.css"
import getConfig from "./config"
import SignedOut from "./components/SignedOut.vue"
import Vouchers from "./components/Vouchers.vue"
import VoucherPayment from "./components/VoucherPayment.vue"

const nearConfig = getConfig("development")
window.networkId = nearConfig.networkId

export default {
  created() {
    document.title = "NEAR Payment Vouchers"
  },
  name: "App",
  components: {
    SignedOut,
    Vouchers,
    VoucherPayment
  },

  computed: {
    isSignedIn() {
      return window.walletConnection.isSignedIn()
    },
    isPayment() {
      return window.location.search.indexOf('user=') !== -1;
    }
  },
}
</script>

