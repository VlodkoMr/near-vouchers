<template>
  <main>
    <Header></Header>

    <div class="container">
      <h1 class="text-center">Payment</h1>

      <b-row class="text-center mt-3" v-if="!isPaymentLoader && !paymentSuccess">
        <b-col></b-col>
        <b-col lg="3" md="5">
          <p><b>Take payment using Voucher:</b></p>
          <form @submit.prevent="transfer">
            <b-input-group>
              <b-form-input type="number" required min="0.1" max="10" step="0.1" placeholder="Pay Amount" v-model="paymentAmount"></b-form-input>
              <b-input-group-append>
                <b-button variant="primary" type="submit">Pay</b-button>
              </b-input-group-append>
            </b-input-group>
          </form>
        </b-col>
        <b-col></b-col>
      </b-row>

      <b-row class="text-center mt-3 success-payment" v-if="!isPaymentLoader && paymentSuccess">
        <b-col></b-col>
        <b-col lg="5" md="6">
          <img src="../assets/success.png" alt="success" width="60">
          <p class="mt-3">
            <b>You received {{ paymentAmount }} NEAR.</b>
            <br>
            Rest voucher balance refunded to the Owner wallet.
          </p>
        </b-col>
        <b-col></b-col>
      </b-row>

      <div v-if="isPaymentLoader" class="text-center">
        <p>Processing payment...</p>
        <img src="../assets/loader.gif" alt="" width="70">
      </div>
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
      paymentSuccess: false,
      isPaymentLoader: false,
      key: "",
      id: "",
      userId: "",
    }
  },
  computed: {
    isSignedIn() {
      return window.walletConnection ? window.walletConnection.isSignedIn() : false
    },
  },
  methods: {
    async transfer() {
      const pay_amount = Big(this.paymentAmount).times(10 ** 24).toFixed();
      this.isPaymentLoader = true;
      let isError = false;
      try {
        await window.contract.transfer({
          key: this.key,
          id: this.id,
          account_id: this.userId,
          pay_amount,
        });
      } catch (e) {
        isError = true;
        let errorText = 'Error: Something went wrong!';
        if (e.kind.ExecutionError) {
          const reg = /'(.*)'/g;
          let error = e.kind.ExecutionError.match(reg);
          if (error.length) {
            errorText = "Error: " + error[0].replaceAll("'", "");
          }
        }
        alert(errorText);
        throw e //re-throw
      } finally {
        this.isPaymentLoader = false;

        if (!isError) {
          this.paymentSuccess = true;
        }
      }
    },
  },
}
</script>
