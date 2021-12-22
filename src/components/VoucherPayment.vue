<template>
  <main>
    <header class="pt-3 mb-4">
      <div class="container d-flex justify-content-between">
        <span>{{ accountId }}</span>
        <button class="btn btn-primary" v-on:click="logout">Sign out</button>
      </div>
    </header>

    <div class="container">
      <h1 class="text-center">Payment</h1>

      <b-row class="text-center mt-3">
        <b-col></b-col>
        <b-col lg="3" md="5">
          <form @submit.prevent="transfer">
            <b-input-group>
              <b-form-input type="number" required min="0.1" max="10" step="0.1" v-model="paymentAmount"></b-form-input>
              <b-input-group-append>
                <b-button variant="primary" type="submit">Pay</b-button>
              </b-input-group-append>
            </b-input-group>
          </form>
        </b-col>
        <b-col></b-col>
      </b-row>
    </div>
  </main>
</template>

<script>
import {logout} from "../utils";

export default {
  name: "VoucherPayment",
  beforeMount() {
    if (this.isSignedIn) {
      console.log('...')
    }
  },
  components: {},
  data: function () {
    return {
      is_ready: false,
      paymentAmount: null
    }
  },
  computed: {
    isSignedIn() {
      return window.walletConnection ? window.walletConnection.isSignedIn() : false
    },
    accountId() {
      return window.accountId
    },
    contractId() {
      return window.contract ? window.contract.contractId : null
    },
    networkId() {
      return window.networkId
    },
  },
  methods: {
    transfer() {
      console.log('!')
    },
    logout: logout,
  },
}
</script>
