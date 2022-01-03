<template>
  <main>
    <Header></Header>

    <div class="container">
      <h1 class="text-center">Payment Request</h1>

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
import Big from 'big.js';
import Header from './Header';

export default {
  name: "VoucherPayment",
  beforeMount() {
    if (this.isSignedIn) {
      this.key = window.location.hash.replace('#', '');
      const urlParams = new URLSearchParams(window.location.search);
      this.id = urlParams.get("id");
      this.userId = urlParams.get("user");
    }
  },
  components: {
    Header,
  },
  data: function () {
    return {
      is_ready: false,
      paymentAmount: null,
      key: "",
      id: "",
      userId: "",
    }
  },
  computed: {
    isSignedIn() {
      return window.walletConnection ? window.walletConnection.isSignedIn() : false
    },
    // accountId() {
    //   return window.accountId
    // },
    // contractId() {
    //   return window.contract ? window.contract.contractId : null
    // },
    // networkId() {
    //   return window.networkId
    // },
  },
  methods: {
    async transfer() {
      const pay_amount = Big(this.paymentAmount).times(10 ** 24).toFixed();
      try {
        await window.contract.transfer({
          key: this.key,
          id: this.id,
          account_id: this.userId,
          pay_amount,
        });
      } catch (e) {
        alert("Something went wrong!");
        throw e //re-throw
      } finally {
        console.log('transferred');
      }
    },
  },
}
</script>
